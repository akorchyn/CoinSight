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
}
