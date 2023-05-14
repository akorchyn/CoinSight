use diesel::prelude::*;
use juniper::GraphQLObject;

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
