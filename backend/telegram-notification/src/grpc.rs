use csb_comm::{
    notification_service_server::{self, NotificationService},
    NotificationRequest,
};
use csb_db_user::{models::TelegramAuth, Db};
use teloxide::{
    prelude::*,
    types::{Chat, Update},
    utils::command::BotCommands,
};
use tonic::transport::Server;

struct GrpcServer {
    db: Db,
    bot: Bot,
}

impl GrpcServer {
    pub fn new(db: Db, bot: Bot) -> Self {
        Self { db, bot }
    }
}

#[tonic::async_trait]
impl NotificationService for GrpcServer {
    async fn send_notification(
        &self,
        request: tonic::Request<NotificationRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let NotificationRequest {
            user_id,
            title,
            body,
        } = request.into_inner();
        let mut db_connection = self
            .db
            .db_connection
            .get()
            .await
            .map_err(|_| tonic::Status::internal("Internal error"))?;

        let telegram_auth = TelegramAuth::by_user(
            &mut db_connection,
            user_id
                .parse()
                .map_err(|_| tonic::Status::invalid_argument("Invalid user id"))?,
        )
        .await
        .map_err(|_| tonic::Status::internal("Internal error"))?
        .ok_or_else(|| tonic::Status::not_found("User not found"))?;

        let chat_id = telegram_auth.telegram_id.unwrap_or_default();
        let message = format!("{}\n\n{}", title, body);

        self.bot
            .send_message(ChatId(chat_id), message)
            .send()
            .await
            .map_err(|e| tonic::Status::internal(format!("Couldn't send a msg: {}", e)))?;
        Ok(tonic::Response::new(()))
    }
}

pub async fn run(db: Db, bot: Bot, port: String) -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let addr = format!("0.0.0.0:{port}").parse().unwrap();
    let server = GrpcServer::new(db, bot);
    let server = csb_comm::notification_service_server::NotificationServiceServer::new(server);

    Server::builder().add_service(server).serve(addr).await?;
    Ok(())
}
