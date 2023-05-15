use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use juniper::GraphQLObject;

#[derive(Queryable, GraphQLObject)]
pub struct Source {
    pub id: i32,
    pub name: String,
}

impl Source {
    pub async fn by_id(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Source> {
        use crate::schema::sources::dsl::sources;

        sources.find(id).first(connection).await
    }

    pub async fn by_name(connection: &mut AsyncPgConnection, name: &str) -> QueryResult<Source> {
        use crate::schema::sources::dsl::{name as name_column, sources};

        sources.filter(name_column.eq(name)).first(connection).await
    }
}

#[derive(Queryable, GraphQLObject)]
pub struct SourceCryptoMapping {
    pub id: i32,
    pub crypto_id: i32,
    pub source_id: i32,
    pub source_key: String,
}

impl SourceCryptoMapping {
    pub async fn load_keys_by_source_name(
        connection: &mut AsyncPgConnection,
        source_name: &str,
    ) -> QueryResult<Vec<SourceCryptoMapping>> {
        use crate::schema::source_crypto_mappings::dsl::{source_crypto_mappings, source_id};

        let id = Source::by_name(connection, source_name).await?.id;

        source_crypto_mappings
            .filter(source_id.eq(id))
            .load(connection)
            .await
    }
}
