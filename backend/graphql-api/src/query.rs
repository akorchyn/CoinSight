use csb_db::{
    models::{self, Cryptocurrency},
    Context,
};
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn cryptocurrency(context: &Context, symbol: String) -> FieldResult<Cryptocurrency> {
        let mut connection = context.db_connection.get()?;
        Ok(Cryptocurrency::by_symbol(&mut connection, &symbol)?)
    }

    fn cryptocurrency_by_id(context: &Context, id: i32) -> FieldResult<Cryptocurrency> {
        let mut connection = context.db_connection.get()?;
        Ok(Cryptocurrency::by_id(&mut connection, id)?)
    }

    fn currencies_by_id(context: &Context, id: i32) -> FieldResult<models::Currency> {
        let mut connection = context.db_connection.get()?;
        Ok(models::Currency::by_id(&mut connection, id)?)
    }

    fn aggregated_price_latest(
        context: &Context,
        crypto_id: i32,
        currency_id: i32,
    ) -> FieldResult<models::AggregatedPrice> {
        let mut connection = context.db_connection.get()?;
        Ok(models::AggregatedPrice::get_latest(
            &mut connection,
            crypto_id,
            currency_id,
        )?)
    }

    fn aggregated_price_history(
        context: &Context,
        crypto_id: i32,
        currency_id: i32,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<models::AggregatedPrice>> {
        let mut connection = context.db_connection.get()?;
        Ok(models::AggregatedPrice::get_history_paged(
            &mut connection,
            crypto_id,
            currency_id,
            limit,
            offset,
        )?)
    }

    fn price_latest(
        context: &Context,
        crypto_id: i32,
        currency_id: i32,
        source_id: i32,
    ) -> FieldResult<models::Price> {
        let mut connection = context.db_connection.get()?;
        Ok(models::Price::get_latest(
            &mut connection,
            crypto_id,
            currency_id,
            source_id,
        )?)
    }

    fn price_history(
        context: &Context,
        crypto_id: i32,
        currency_id: i32,
        source_id: i32,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<models::Price>> {
        let mut connection = context.db_connection.get()?;
        Ok(models::Price::get_history_paged(
            &mut connection,
            crypto_id,
            currency_id,
            source_id,
            limit,
            offset,
        )?)
    }

    fn search(context: &Context, query: String) -> FieldResult<Vec<Cryptocurrency>> {
        if query.is_empty() {
            return Ok(vec![]);
        }
        let mut connection = context.db_connection.get()?;
        Ok(Cryptocurrency::search_by_symbol_or_name(
            &mut connection,
            query,
        )?)
    }
}
