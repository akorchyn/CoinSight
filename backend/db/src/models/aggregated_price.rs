use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use juniper::GraphQLObject;

#[derive(Queryable, GraphQLObject)]
pub struct AggregatedPrice {
    pub id: i32,
    pub crypto_id: i32,
    pub currency_id: i32,
    pub median_price: bigdecimal::BigDecimal,
    pub first_quartile_price: bigdecimal::BigDecimal,
    pub third_quartile_price: bigdecimal::BigDecimal,
    pub timestamp: chrono::NaiveDateTime,
}

impl AggregatedPrice {
    pub async fn get_latest(
        connection: &mut AsyncPgConnection,
        crypto_id: i32,
        currency_id: i32,
    ) -> QueryResult<AggregatedPrice> {
        use crate::schema::aggregated_prices::dsl::{
            aggregated_prices, crypto_id as crypto_id_column, currency_id as currency_id_column,
            timestamp,
        };

        aggregated_prices
            .filter(crypto_id_column.eq(crypto_id))
            .filter(currency_id_column.eq(currency_id))
            .order_by(timestamp.desc())
            .first(connection)
            .await
    }

    pub async fn get_history_paged(
        connection: &mut AsyncPgConnection,
        crypto_id: i32,
        currency_id: i32,
        page: i32,
        page_size: i32,
    ) -> QueryResult<Vec<AggregatedPrice>> {
        use crate::schema::aggregated_prices::dsl::{
            aggregated_prices, crypto_id as crypto_id_column, currency_id as currency_id_column,
            timestamp,
        };

        aggregated_prices
            .filter(crypto_id_column.eq(crypto_id))
            .filter(currency_id_column.eq(currency_id))
            .order_by(timestamp.desc())
            .offset((page * page_size) as i64)
            .limit(page_size as i64)
            .load(connection)
            .await
    }
}
