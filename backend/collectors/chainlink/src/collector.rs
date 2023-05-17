use std::{sync::Arc, time::Duration};

use anyhow::Context as ErrorContext;
use bigdecimal::num_bigint::BigInt;
use chrono::NaiveDateTime;
use csb_db::{
    models::{NewPrice, SourceCryptoMapping},
    Context,
};
use ethers::{
    prelude::Http,
    providers::{Middleware, Provider},
    types::H160,
};
use futures::future::join_all;
use reqwest::Url;

use crate::error::CollectorError;

pub type TokenId = i32;
const EVENT_SLEEP_DURATION: Duration = Duration::from_secs(60);

mod chain_link_abi {
    use ethers::prelude::abigen;

    abigen!(AggregatorV3Contract, "aggregatorV3InterfaceABI.json");
}

/// The price oracle. It fetches prices from the chainlink oracle contracts on the Ethereum network.
pub struct Collector {
    provider: Arc<Provider<Http>>,
    db_context: Context,
    source_id: i32,
    data: Vec<(TokenId, H160)>,
}

impl Collector {
    /// Creates a new price oracle with the given subscription list.
    pub async fn new(
        ethereum_node: Url,
        input: Vec<SourceCryptoMapping>,
        db_context: Context,
    ) -> Result<Self, CollectorError> {
        let provider =
            Provider::<Http>::try_from(ethereum_node.to_string()).with_context(|| {
                format!("Couldn't connect to ethereum node under {ethereum_node} url",)
            })?;
        let provider = Arc::new(provider);
        let source_id = input[0].source_id;
        let input = input.into_iter().map(|item| async {
            let item = item;
            provider
                .resolve_name(&item.source_key)
                .await
                .map(|address| (item.crypto_id, address))
        });
        let input: Result<Vec<(TokenId, ethers::types::H160)>, _> =
            join_all(input).await.into_iter().collect();
        let data = input?;

        Ok(Collector {
            provider,
            data,
            db_context,
            source_id,
        })
    }

    /// Get the price for the given token name and address from the chainlink oracle contract.
    pub async fn fetch_price(
        &self,
        crypto_id: TokenId,
        address: H160,
    ) -> Result<(), CollectorError> {
        let oracle_contract =
            chain_link_abi::AggregatorV3Contract::new(address, self.provider.clone());
        let decimals = oracle_contract
            .decimals()
            .call()
            .await
            .context("Couldn't fetch decimals data for {crypto_id}")?;
        let (_, latest_price, _, updated_at, _) =
            oracle_contract.latest_round_data().call().await?;

        if latest_price.is_negative() {
            return Err(CollectorError::NegativeLatestPrice(crypto_id.to_string()));
        }

        let updated_at = NaiveDateTime::from_timestamp_opt(updated_at.as_u64() as i64, 0).context(
            "Couldn't convert the timestamp from the chainlink oracle contract to NaiveDateTime for {crypto_id}",
        )?;
        let mut connection = self.db_context.db_connection.get().await.context(
            "Couldn't get a connection from the pool to insert the new price into the database",
        )?;
        let mut bytes = [0u8; 32];
        latest_price.to_little_endian(&mut bytes);
        let latest_price =
            bigdecimal::BigDecimal::new(BigInt::from_signed_bytes_le(&bytes), decimals.into());

        let new_price = NewPrice::new(crypto_id, self.source_id, 1, latest_price, updated_at);

        new_price.insert(&mut connection).await?;
        Ok(())
    }

    /// Get the prices for all the tokens in the subscription list.
    pub async fn fetch_all(&self) -> Vec<Result<(), CollectorError>> {
        let mut result = vec![];
        result.reserve(self.data.len());

        for (token_id, address) in &self.data {
            result.push(self.fetch_price(*token_id, *address));
        }

        join_all(result).await
    }

    pub async fn run(&self) -> Result<(), CollectorError> {
        let mut interval = tokio::time::interval(EVENT_SLEEP_DURATION);
        loop {
            interval.tick().await;

            let result = self
                .fetch_all()
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>();
            if let Err(err) = result {
                eprintln!("Error fetching prices: {:?}", err);
            }
        }
    }
}
