use diesel::prelude::*;
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
    pub fn get_latest(
        connection: &mut PgConnection,
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
    }

    pub fn get_history_paged(
        connection: &mut PgConnection,
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
    }
}

#[derive(Queryable, GraphQLObject)]
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
}

#[derive(Queryable, GraphQLObject)]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub symbol: String,
}

impl Currency {
    pub fn by_id(connection: &mut PgConnection, id: i32) -> QueryResult<Currency> {
        use crate::schema::currencies::dsl::currencies;

        currencies.find(id).first(connection)
    }
}

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

#[derive(Queryable, GraphQLObject)]
pub struct SourceCryptoMapping {
    pub id: i32,
    pub crypto_id: i32,
    pub source_id: i32,
    pub source_key: String,
}

#[derive(Queryable, GraphQLObject)]
pub struct Source {
    pub id: i32,
    pub name: String,
}

impl Source {
    pub fn by_id(connection: &mut PgConnection, id: i32) -> QueryResult<Source> {
        use crate::schema::sources::dsl::sources;

        sources.find(id).first(connection)
    }
}
