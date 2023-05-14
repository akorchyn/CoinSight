use diesel::prelude::*;
use juniper::GraphQLObject;

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

#[derive(Queryable, GraphQLObject)]
pub struct SourceCryptoMapping {
    pub id: i32,
    pub crypto_id: i32,
    pub source_id: i32,
    pub source_key: String,
}
