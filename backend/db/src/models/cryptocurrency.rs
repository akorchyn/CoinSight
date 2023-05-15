use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};

use crate::Context;

use super::{AggregatedPrice, Price};

#[derive(Queryable)]
pub struct Cryptocurrency {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub description: String,
}

impl Cryptocurrency {
    pub fn by_id(connection: &mut PgConnection, id: i32) -> QueryResult<Cryptocurrency> {
        use crate::schema::cryptocurrencies::dsl::cryptocurrencies;

        cryptocurrencies.find(id).first(connection)
    }

    pub fn by_symbol(connection: &mut PgConnection, symbol: &str) -> QueryResult<Cryptocurrency> {
        use crate::schema::cryptocurrencies::dsl::{cryptocurrencies, symbol as symbol_column};

        cryptocurrencies
            .filter(symbol_column.eq(symbol))
            .first(connection)
    }

    pub fn search_by_symbol_or_name(
        connection: &mut PgConnection,
        query: String,
    ) -> QueryResult<Vec<Cryptocurrency>> {
        use crate::schema::cryptocurrencies::dsl::{cryptocurrencies, id, name, symbol};

        cryptocurrencies
            .filter(symbol.ilike(format!("%{}%", query)))
            .or_filter(name.ilike(format!("%{}%", query)))
            .order(name.asc())
            .group_by(id)
            .limit(10)
            .load(connection)
    }
}

#[graphql_object(context = Context)]
impl Cryptocurrency {
    fn name(&self) -> &str {
        &self.name
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn latest_aggregated_price(
        &self,
        #[graphql(default = 1)] currency_id: i32,
        context: &Context,
    ) -> FieldResult<AggregatedPrice> {
        let mut connection = context.db_connection.get()?;
        Ok(AggregatedPrice::get_latest(
            &mut connection,
            self.id,
            currency_id,
        )?)
    }

    fn latest_price(
        &self,
        #[graphql(default = 1)] currency_id: i32,
        source_id: i32,
        context: &Context,
    ) -> FieldResult<Price> {
        let mut connection = context.db_connection.get()?;
        Ok(Price::get_latest(
            &mut connection,
            self.id,
            currency_id,
            source_id,
        )?)
    }

    fn aggregated_history(
        &self,
        #[graphql(default = 1)] currency_id: i32,
        context: &Context,
    ) -> FieldResult<Vec<AggregatedPrice>> {
        let mut connection = context.db_connection.get()?;
        Ok(AggregatedPrice::get_history_paged(
            &mut connection,
            self.id,
            currency_id,
            0,
            500,
        )?)
    }

    fn history(
        &self,
        #[graphql(default = 1)] currency_id: i32,
        source_id: i32,
        context: &Context,
    ) -> FieldResult<Vec<Price>> {
        let mut connection = context.db_connection.get()?;
        Ok(Price::get_history_paged(
            &mut connection,
            self.id,
            currency_id,
            source_id,
            0,
            500,
        )?)
    }
}
