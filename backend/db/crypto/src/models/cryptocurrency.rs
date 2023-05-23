use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable)]
pub struct Cryptocurrency {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub is_top: bool,
}

impl Cryptocurrency {
    pub async fn by_id(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Cryptocurrency> {
        use crate::schema::cryptocurrencies::dsl::cryptocurrencies;

        cryptocurrencies.find(id).first(connection).await
    }

    pub async fn by_symbol(
        connection: &mut AsyncPgConnection,
        symbol: &str,
    ) -> QueryResult<Cryptocurrency> {
        use crate::schema::cryptocurrencies::dsl::{cryptocurrencies, symbol as symbol_column};

        cryptocurrencies
            .filter(symbol_column.eq(symbol))
            .first(connection)
            .await
    }

    pub async fn search_by_symbol_or_name(
        connection: &mut AsyncPgConnection,
        query: String,
    ) -> QueryResult<Vec<Cryptocurrency>> {
        use crate::schema::cryptocurrencies::dsl::{cryptocurrencies, id, name, symbol};

        cryptocurrencies
            .filter(symbol.ilike(format!("{query}%")))
            .or_filter(name.ilike(format!("{query}%")))
            .order(name.asc())
            .group_by(id)
            .limit(10)
            .load(connection)
            .await
    }

    pub async fn top_cryptocurrencies(
        connection: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Cryptocurrency>> {
        use crate::schema::cryptocurrencies::dsl::{cryptocurrencies, is_top};

        cryptocurrencies
            .filter(is_top.eq(true))
            .limit(limit)
            .load(connection)
            .await
    }

    pub async fn all(connection: &mut AsyncPgConnection) -> QueryResult<Vec<Cryptocurrency>> {
        use crate::schema::cryptocurrencies::dsl::cryptocurrencies;

        cryptocurrencies.load(connection).await
    }
}
