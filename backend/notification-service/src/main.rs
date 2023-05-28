mod notifier;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let user_db =
        csb_db_user::Db::new(std::env::var("USER_DATABASE_URL").expect("USER_DATABASE_URL"))
            .await
            .expect("Failed to connect to database");
    let crypto_db =
        csb_db_crypto::Db::new(std::env::var("CRYPTO_DATABASE_URL").expect("CRYPTO_DATABASE_URL"))
            .await
            .expect("Failed to connect to database");
    let brello_api_key = std::env::var("BRELLO_API_KEY").expect("BRELLO_API_KEY");

    notifier::Notifier::new(user_db, crypto_db, brello_api_key)
        .run()
        .await;
}
