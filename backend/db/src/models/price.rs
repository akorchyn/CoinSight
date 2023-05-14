use diesel::prelude::*;
use juniper::GraphQLObject;

#[derive(Queryable, GraphQLObject)]
pub struct Price {
    pub id: i32,
    pub crypto_id: i32,
    pub source_id: i32,
    pub currency_id: i32,
    pub price: bigdecimal::BigDecimal,
    pub timestamp: chrono::NaiveDateTime,
}

impl Price {
    pub fn get_latest(
        connection: &mut PgConnection,
        crypto_id: i32,
        currency_id: i32,
        source_id: i32,
    ) -> QueryResult<Price> {
        use crate::schema::prices::dsl::{
            crypto_id as crypto_id_column, currency_id as currency_id_column, prices,
            source_id as source_id_column, timestamp,
        };

        prices
            .filter(crypto_id_column.eq(crypto_id))
            .filter(currency_id_column.eq(currency_id))
            .filter(source_id_column.eq(source_id))
            .order_by(timestamp.desc())
            .first(connection)
    }

    pub fn get_history_paged(
        connection: &mut PgConnection,
        crypto_id: i32,
        currency_id: i32,
        source_id: i32,
        page: i32,
        page_size: i32,
    ) -> QueryResult<Vec<Price>> {
        use crate::schema::prices::dsl::{
            crypto_id as crypto_id_column, currency_id as currency_id_column, prices,
            source_id as source_id_column, timestamp,
        };

        prices
            .filter(crypto_id_column.eq(crypto_id))
            .filter(currency_id_column.eq(currency_id))
            .filter(source_id_column.eq(source_id))
            .order_by(timestamp.desc())
            .offset((page * page_size) as i64)
            .limit(page_size as i64)
            .load(connection)
    }
}
