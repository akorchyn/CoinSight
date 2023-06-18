use csb_db_user::Db;
use teloxide::Bot;

mod bot;
mod grpc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let bot = Bot::from_env();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").expect("PORT must be set");
    let db = Db::new(db_url)
        .await
        .expect("Failed to connect to database");

    let db1 = db.clone();
    let bot1 = bot.clone();
    let handle = tokio::spawn(async move {
        bot::run(db1, bot1).await;
    });
    let grpc_handle = tokio::spawn(async move {
        grpc::run(db, bot, port).await.unwrap();
    });

    tokio::try_join!(handle, grpc_handle).unwrap();
}
