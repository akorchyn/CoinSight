use crate::types::crypto;
use crate::Context;
use csb_db_crypto::models;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn crypto() -> cryptocurrency::CryptoQuery {
        cryptocurrency::CryptoQuery
    }

    fn price() -> price::PriceQuery {
        price::PriceQuery
    }

    fn aggregated_price() -> aggregated_price::AggregatedPriceQuery {
        aggregated_price::AggregatedPriceQuery
    }

    fn source() -> sources::SourceQuery {
        sources::SourceQuery
    }
}

mod price {

    use super::*;

    pub struct PriceQuery;

    #[graphql_object(context = Context)]
    impl PriceQuery {
        async fn latest(
            context: &Context,
            crypto_id: i32,
            currency_id: i32,
            source_id: i32,
        ) -> FieldResult<crypto::Price> {
            let mut connection = context.crypto_db.db_connection.get().await?;
            Ok(crypto::Price(
                models::Price::get_latest(&mut connection, crypto_id, currency_id, source_id)
                    .await?,
            ))
        }

        async fn history(
            context: &Context,
            crypto_id: i32,
            currency_id: i32,
            source_id: i32,
            limit: i32,
            offset: i32,
        ) -> FieldResult<Vec<crypto::Price>> {
            let mut connection = context.crypto_db.db_connection.get().await?;
            Ok(models::Price::get_history_paged(
                &mut connection,
                crypto_id,
                currency_id,
                source_id,
                limit,
                offset,
            )
            .await?
            .into_iter()
            .map(crypto::Price)
            .collect())
        }
    }
}

mod aggregated_price {
    use super::*;

    pub struct AggregatedPriceQuery;

    #[graphql_object(context = Context)]
    impl AggregatedPriceQuery {
        async fn latest(
            context: &Context,
            crypto_id: i32,
            currency_id: i32,
        ) -> FieldResult<crypto::AggregatedPrice> {
            let mut connection = context.crypto_db.db_connection.get().await?;
            Ok(crypto::AggregatedPrice(
                models::AggregatedPrice::get_latest(&mut connection, crypto_id, currency_id)
                    .await?,
            ))
        }

        async fn history(
            context: &Context,
            crypto_id: i32,
            currency_id: i32,
            limit: i32,
            offset: i32,
        ) -> FieldResult<Vec<crypto::AggregatedPrice>> {
            let mut connection = context.crypto_db.db_connection.get().await?;
            Ok(models::AggregatedPrice::get_history_paged(
                &mut connection,
                crypto_id,
                currency_id,
                limit,
                offset,
            )
            .await?
            .into_iter()
            .map(crypto::AggregatedPrice)
            .collect())
        }
    }
}

mod cryptocurrency {
    use super::*;

    pub struct CryptoQuery;

    #[graphql_object(context = Context)]
    impl CryptoQuery {
        async fn by_symbol(
            context: &Context,
            symbol: String,
        ) -> FieldResult<crypto::Cryptocurrency> {
            let mut connection = context.crypto_db.db_connection.get().await?;
            Ok(crypto::Cryptocurrency(
                models::Cryptocurrency::by_symbol(&mut connection, &symbol).await?,
            ))
        }

        async fn by_id(context: &Context, id: i32) -> FieldResult<crypto::Cryptocurrency> {
            let mut connection = context.crypto_db.db_connection.get().await?;
            Ok(crypto::Cryptocurrency(
                models::Cryptocurrency::by_id(&mut connection, id).await?,
            ))
        }

        async fn search(
            context: &Context,
            query: String,
        ) -> FieldResult<Vec<crypto::Cryptocurrency>> {
            if query.is_empty() {
                return Ok(vec![]);
            }
            let mut connection = context.crypto_db.db_connection.get().await?;
            Ok(
                models::Cryptocurrency::search_by_symbol_or_name(&mut connection, query)
                    .await?
                    .into_iter()
                    .map(crypto::Cryptocurrency)
                    .collect(),
            )
        }

        async fn top(
            context: &Context,
            #[graphql(default = 10)] limit: i32,
        ) -> FieldResult<Vec<crypto::Cryptocurrency>> {
            let mut connection = context.crypto_db.db_connection.get().await?;
            Ok(
                models::Cryptocurrency::top_cryptocurrencies(&mut connection, limit.into())
                    .await?
                    .into_iter()
                    .map(crypto::Cryptocurrency)
                    .collect(),
            )
        }
    }
}

mod sources {
    use crate::types::crypto::FullHistory;

    use super::*;

    pub struct SourceQuery;

    #[graphql_object(context = Context)]
    impl SourceQuery {
        async fn full_history(context: &Context, crypto_id: i32) -> FieldResult<FullHistory> {
            let mut connection = context.crypto_db.db_connection.get().await?;
            let sources = models::Source::all(&mut connection)
                .await?
                .into_iter()
                .map(crypto::Source)
                .collect();
            let aggregated_prices =
                models::AggregatedPrice::get_history_paged(&mut connection, crypto_id, 1, 0, 500)
                    .await?
                    .into_iter()
                    .map(crypto::AggregatedPrice)
                    .collect();
            Ok(FullHistory {
                aggregated_prices,
                sources,
            })
        }
    }
}
