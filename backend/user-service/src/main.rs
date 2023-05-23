use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT").expect("PORT must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let key = std::env::var("KEY").expect("KEY must be set");

    let db = csb_db_user::Db::new(database_url)
        .await
        .expect("Failed to create context");
    let addr = format!("0.0.0.0:{port}").parse().unwrap();
    let server = user_service::UserService::new(db, key.into_bytes());
    let server = csb_comm::user_service_server::UserServiceServer::new(server);

    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
