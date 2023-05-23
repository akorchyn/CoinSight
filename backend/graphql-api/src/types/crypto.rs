use bigdecimal::BigDecimal;
use csb_db_crypto::models;
use juniper::{graphql_object, FieldResult};

use crate::Context;

pub struct Cryptocurrency(pub csb_db_crypto::models::Cryptocurrency);

#[graphql_object(context = Context)]
impl Cryptocurrency {
    fn name(&self) -> &str {
        &self.0.name
    }

    fn symbol(&self) -> &str {
        &self.0.symbol
    }

    fn description(&self) -> &str {
        &self.0.description
    }

    async fn latest_aggregated_price(
        &self,
        #[graphql(default = 1)] currency_id: i32,
        context: &Context,
    ) -> FieldResult<AggregatedPrice> {
        let mut connection = context.crypto_db.db_connection.get().await?;
        Ok(AggregatedPrice(
            models::AggregatedPrice::get_latest(&mut connection, self.0.id, currency_id).await?,
        ))
    }

    async fn latest_price(
        &self,
        #[graphql(default = 1)] currency_id: i32,
        source_id: i32,
        context: &Context,
    ) -> FieldResult<Price> {
        let mut connection = context.crypto_db.db_connection.get().await?;
        Ok(Price(
            models::Price::get_latest(&mut connection, self.0.id, currency_id, source_id).await?,
        ))
    }

    async fn aggregated_history(
        &self,
        #[graphql(default = 1)] currency_id: i32,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 500)] limit: i32,
        context: &Context,
    ) -> FieldResult<Vec<AggregatedPrice>> {
        let mut connection = context.crypto_db.db_connection.get().await?;
        Ok(models::AggregatedPrice::get_history_paged(
            &mut connection,
            self.0.id,
            currency_id,
            offset,
            limit,
        )
        .await?
        .into_iter()
        .map(AggregatedPrice)
        .collect())
    }

    async fn history(
        &self,
        source_id: i32,
        #[graphql(default = 1)] currency_id: i32,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 500)] limit: i32,
        context: &Context,
    ) -> FieldResult<Vec<Price>> {
        let mut connection = context.crypto_db.db_connection.get().await?;
        Ok(models::Price::get_history_paged(
            &mut connection,
            self.0.id,
            currency_id,
            source_id,
            offset,
            limit,
        )
        .await?
        .into_iter()
        .map(Price)
        .collect())
    }
}

pub struct AggregatedPrice(pub csb_db_crypto::models::AggregatedPrice);

#[graphql_object]
impl AggregatedPrice {
    fn median_price(&self) -> &BigDecimal {
        &self.0.median_price
    }

    fn timestamp(&self) -> chrono::NaiveDateTime {
        self.0.timestamp
    }

    fn first_quartile_price(&self) -> &BigDecimal {
        &self.0.first_quartile_price
    }

    fn third_quartile_price(&self) -> &BigDecimal {
        &self.0.third_quartile_price
    }
}

pub struct Price(pub csb_db_crypto::models::Price);

#[graphql_object]
impl Price {
    fn price(&self) -> &BigDecimal {
        &self.0.price
    }

    fn timestamp(&self) -> chrono::NaiveDateTime {
        self.0.timestamp
    }
}
