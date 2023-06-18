use csb_db_user::{models::TelegramAuth, Db};
use teloxide::{
    prelude::*,
    types::{Chat, Update},
    utils::command::BotCommands,
};

#[derive(Clone)]
struct ConfigParameters {
    db: Db,
}

pub async fn run(db: Db, bot: Bot) {
    // setup logging
    env_logger::init();

    let parameters = ConfigParameters { db };

    let handler = Update::filter_message().branch(
        dptree::entry()
            .filter(|msg: Message| msg.chat.is_private())
            .filter_command::<SimpleCommand>()
            .endpoint(simple_commands_handler),
    );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![parameters])
        .default_handler(|_| async move {})
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Simple commands")]
enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "verify authorization.")]
    Verify(String),
}

async fn simple_commands_handler(
    cfg: ConfigParameters,
    bot: Bot,
    msg: Message,
    cmd: SimpleCommand,
) -> Result<(), anyhow::Error> {
    match cmd {
        SimpleCommand::Help => {
            bot.send_message(msg.chat.id, SimpleCommand::descriptions().to_string())
                .send()
                .await?;
        }
        SimpleCommand::Verify(code) => {
            let mut db = cfg.db.db_connection.get().await?;

            let telegram_id = msg.chat.id;
            if code.is_empty() {
                bot.send_message(msg.chat.id, "Invalid msg").send().await?;
                return Ok(());
            }

            let telegram_auth = TelegramAuth::by_auth_code(&mut db, &code).await?;
            if let Some(telegram_auth) = telegram_auth {
                telegram_auth
                    .update_telegram_id(&mut db, telegram_id.0)
                    .await?;
                bot.send_message(msg.chat.id, "You are authorized")
                    .send()
                    .await?;
            } else {
                bot.send_message(msg.chat.id, "Invalid auth code")
                    .send()
                    .await?;
            }
        }
    }

    Ok(())
}
