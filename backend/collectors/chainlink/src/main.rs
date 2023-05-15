use reqwest::Url;

mod collector;
mod error;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let node_url = std::env::var("NODE_URL").expect("NODE_URL must be set");
    let node_url = Url::parse(&node_url).expect("NODE_URL must be a valid url");
    let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let context = csb_db::Context::new(connection_string)
        .await
        .expect("Failed to connect to the database");
    let required_keys = csb_db::models::SourceCryptoMapping::load_keys_by_source_name(
        &mut context
            .db_connection
            .get()
            .await
            .expect("Expected to get a connection from the pool"),
        "ChainLink",
    )
    .await
    .expect("Failed to load keys");
    if required_keys.is_empty() {
        panic!("No keys found for ChainLink");
    }
    let collector = collector::Collector::new(node_url, required_keys, context)
        .await
        .expect("Failed to initialize the collector");
    collector.run().await.expect("Failed to run the oracle");
}
