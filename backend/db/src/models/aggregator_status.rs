use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use juniper::GraphQLObject;

#[derive(Queryable, GraphQLObject)]
pub struct AggregatorStatus {
    pub currency_id: i32,
    pub timestamp: chrono::NaiveDateTime,
}

impl AggregatorStatus {
    pub async fn by_crypto_id_and_currency_id(
        connection: &mut AsyncPgConnection,
        currency_id: i32,
    ) -> QueryResult<AggregatorStatus> {
        use crate::schema::aggregator_status::dsl::{
            aggregator_status, currency_id as currency_id_column,
        };

        aggregator_status
            .filter(currency_id_column.eq(currency_id))
            .first(connection)
            .await
    }

    pub async fn update_time(
        connection: &mut AsyncPgConnection,
        currency_id: i32,
        time: chrono::NaiveDateTime,
    ) -> QueryResult<AggregatorStatus> {
        use crate::schema::aggregator_status::dsl::{
            aggregator_status, currency_id as currency_id_column, timestamp,
        };

        diesel::insert_into(aggregator_status)
            .values((currency_id_column.eq(currency_id), timestamp.eq(time)))
            .on_conflict(currency_id_column)
            .do_update()
            .set(timestamp.eq(time))
            .get_result(connection)
            .await
    }
}
