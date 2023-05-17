use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use juniper::GraphQLObject;

#[derive(Queryable, GraphQLObject)]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub symbol: String,
}

impl Currency {
    pub async fn by_id(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Currency> {
        use crate::schema::currencies::dsl::currencies;

        currencies.find(id).first(connection).await
    }

    pub async fn by_symbol(
        connection: &mut AsyncPgConnection,
        symbol: &str,
    ) -> QueryResult<Currency> {
        use crate::schema::currencies::dsl::{currencies, symbol as symbol_column};

        currencies
            .filter(symbol_column.eq(symbol))
            .first(connection)
            .await
    }
}
