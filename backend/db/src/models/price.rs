use crate::schema::*;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use juniper::GraphQLObject;

#[derive(Queryable, GraphQLObject)]
pub struct Price {
    pub id: i32,
    pub crypto_id: i32,
    pub source_id: i32,
    pub currency_id: i32,
    pub price: bigdecimal::BigDecimal,
    pub timestamp: chrono::NaiveDateTime,
    pub is_processed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = prices)]
pub struct NewPrice {
    pub crypto_id: i32,
    pub source_id: i32,
    pub currency_id: i32,
    pub price: bigdecimal::BigDecimal,
    pub timestamp: chrono::NaiveDateTime,
}

impl NewPrice {
    pub fn new(
        crypto_id: i32,
        source_id: i32,
        price: bigdecimal::BigDecimal,
        timestamp: chrono::NaiveDateTime,
    ) -> Self {
        Self {
            crypto_id,
            source_id,
            currency_id: 1, // Currently, supports only USD
            price,
            timestamp,
        }
    }

    pub async fn insert(&self, connection: &mut AsyncPgConnection) -> QueryResult<usize> {
        use crate::schema::prices::dsl::*;

        diesel::insert_into(prices)
            .values(self)
            .on_conflict((crypto_id, source_id, currency_id, timestamp))
            .do_nothing()
            .execute(connection)
            .await
    }
}

impl Price {
    pub async fn get_latest(
        connection: &mut AsyncPgConnection,
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
            .await
    }

    pub async fn get_history_paged(
        connection: &mut AsyncPgConnection,
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
            .await
    }
}
