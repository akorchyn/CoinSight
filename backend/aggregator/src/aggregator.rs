use csb_db::models::{AggregatorStatus, Cryptocurrency, NewAggregatorPrice, Price};
use futures::future::join_all;

pub(crate) struct Aggregator {
    context: csb_db::Context,
    currency_id: i32,
    process_currencies: Vec<Cryptocurrency>,
}

impl Aggregator {
    pub(crate) fn new(
        context: csb_db::Context,
        currency_id: i32,
        process_currencies: Vec<Cryptocurrency>,
    ) -> Self {
        Self {
            context,
            currency_id,
            process_currencies,
        }
    }

    async fn update(
        &self,
        crypto: &Cryptocurrency,
        update_time: chrono::NaiveDateTime,
    ) -> anyhow::Result<()> {
        let mut connection = self.context.db_connection.get().await?;
        let prices = csb_db::models::Price::get_any_price_between(
            &mut connection,
            crypto.id,
            self.currency_id,
            update_time - chrono::Duration::minutes(3),
            update_time,
        )
        .await?;
        if prices.is_empty() {
            return Ok(());
        }

        let median = prices[prices.len() / 2].price.clone();
        let first_quartile = prices[prices.len() / 4].price.clone();
        let third_quartile = prices[(prices.len() * 3) / 4].price.clone();
        let aggregated_price = NewAggregatorPrice::new(
            crypto.id,
            self.currency_id,
            median,
            first_quartile,
            third_quartile,
            update_time,
        );

        aggregated_price.save(&mut connection).await?;
        Price::mark_as_processed(&mut connection, crypto.id, self.currency_id, prices).await?;
        Ok(())
    }

    async fn update_time(&self, time: chrono::NaiveDateTime) -> anyhow::Result<AggregatorStatus> {
        let mut connection = self.context.db_connection.get().await?;
        Ok(
            csb_db::models::AggregatorStatus::update_time(&mut connection, self.currency_id, time)
                .await?,
        )
    }

    pub(crate) async fn run(&self) {
        let period = tokio::time::Duration::from_secs(60);
        let mut interval = tokio::time::interval(period);
        let period = chrono::Duration::from_std(period).expect("Failed to convert to chrono");
        let mut update_time = {
            let mut connection = self
                .context
                .db_connection
                .get()
                .await
                .expect("Failed to connect to db");

            csb_db::models::AggregatorStatus::by_crypto_id_and_currency_id(
                &mut connection,
                self.currency_id,
            )
            .await
            .map(|s| s.timestamp)
            .unwrap_or_else(|_| chrono::Utc::now().naive_utc())
        };

        loop {
            let current_time = chrono::Utc::now().naive_utc();
            if update_time > current_time {
                interval.tick().await;
            }

            let mut result = vec![];
            result.reserve(self.process_currencies.len());

            for currency in &self.process_currencies {
                result.push(self.update(currency, update_time));
            }
            let result = join_all(result).await;
            for r in result {
                if let Err(e) = r {
                    eprintln!("Failed to update cryptocurrency: {e}");
                }
            }
            if let Err(e) = self.update_time(update_time).await {
                eprintln!("Failed to update aggregator status: {e}");
            }
            update_time += period;
        }
    }
}
