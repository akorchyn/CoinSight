mod aggregator;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let context = csb_db::Context::new(std::env::var("DATABASE_URL").expect("DATABASE_URL"))
        .await
        .expect("Failed to connect to database");
    let currency = std::env::var("CURRENCY").expect("CURRENCY");
    let mut connection = context
        .db_connection
        .get()
        .await
        .expect("Expect connection to db");
    let currency_id = csb_db::models::Currency::by_symbol(&mut connection, &currency)
        .await
        .expect("Failed to get currency id");
    let cryptocurrencies = csb_db::models::Cryptocurrency::all(&mut connection)
        .await
        .expect("Failed to get cryptocurrencies");
    drop(connection);
    aggregator::Aggregator::new(context, currency_id.id, cryptocurrencies)
        .run()
        .await;
}
