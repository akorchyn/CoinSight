use std::collections::HashMap;

use bigdecimal::BigDecimal;
use csb_db_crypto::models::{AggregatedPrice, Cryptocurrency, Price, Source};
use csb_db_user::models::{Notification, User};
use reqwest::header::HeaderMap;
use serde_json::json;

enum NotificationType {
    PriceChange,
    PercentChange,
}

impl NotificationType {
    fn from_str(notification_type: &str) -> Option<Self> {
        match notification_type {
            "by Value" => Some(Self::PriceChange),
            "by Percent" => Some(Self::PercentChange),
            _ => None,
        }
    }
}

enum SourceType {
    Source(i32),
    Median,
    Q1,
    Q3,
}

impl SourceType {
    fn from_str(source: &str) -> Option<Self> {
        match source {
            "Median Price" => Some(Self::Median),
            "First Quartile Price" => Some(Self::Q1),
            "Third Quartile Price" => Some(Self::Q3),
            _ => None,
        }
    }
}

struct ParsedNotification {
    user_id: i32,
    crypto_id: i32,
    notification_type: NotificationType,
    source: SourceType,
    value: BigDecimal,
    price: BigDecimal,
    name: String,
}

impl ParsedNotification {
    async fn verify_applicable(&self, crypto_db: &csb_db_crypto::Db) -> Option<bool> {
        let mut connection = crypto_db.db_connection.get().await.ok()?;
        let price = match self.source {
            SourceType::Source(source_id) => {
                Price::get_latest(&mut connection, self.crypto_id, 1, source_id)
                    .await
                    .ok()?
                    .price
            }
            SourceType::Median => {
                AggregatedPrice::get_latest(&mut connection, self.crypto_id, 1)
                    .await
                    .ok()?
                    .median_price
            }
            SourceType::Q1 => {
                AggregatedPrice::get_latest(&mut connection, self.crypto_id, 1)
                    .await
                    .ok()?
                    .first_quartile_price
            }
            SourceType::Q3 => {
                AggregatedPrice::get_latest(&mut connection, self.crypto_id, 1)
                    .await
                    .ok()?
                    .third_quartile_price
            }
        };

        let diff = (price - self.price.clone()).abs();
        Some(match self.notification_type {
            NotificationType::PriceChange => diff > self.value,
            NotificationType::PercentChange => {
                let percent_change = diff / self.price.clone() * BigDecimal::from(100);
                self.value > percent_change
            }
        })
    }
}

pub(crate) struct Notifier {
    user_db: csb_db_user::Db,
    crypto_db: csb_db_crypto::Db,
    brello_api_key: String,
}

impl Notifier {
    pub(crate) fn new(
        user_db: csb_db_user::Db,
        crypto_db: csb_db_crypto::Db,
        brello_api_key: String,
    ) -> Self {
        Self {
            user_db,
            crypto_db,
            brello_api_key,
        }
    }

    pub(crate) async fn run(&mut self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;

            let user_connection = self.user_db.db_connection.get().await;
            let crypto_connection = self.crypto_db.db_connection.get().await;
            if user_connection.is_err() || crypto_connection.is_err() {
                continue;
            }
            let mut user_connection = user_connection.unwrap();
            let mut crypto_connection = crypto_connection.unwrap();

            let user_notifications =
                csb_db_user::models::Notification::all(&mut user_connection).await;

            let latest_crypto_data =
                csb_db_crypto::models::Cryptocurrency::all(&mut crypto_connection).await;

            let sources = csb_db_crypto::models::Source::all(&mut crypto_connection).await;

            if user_notifications.is_err() || latest_crypto_data.is_err() || sources.is_err() {
                continue;
            }
            drop(user_connection);
            drop(crypto_connection);

            self.process_notifications(
                user_notifications.unwrap(),
                latest_crypto_data.unwrap(),
                sources.unwrap(),
            )
            .await;
        }
    }

    async fn process_notifications(
        &mut self,
        notifications: Vec<Notification>,
        crypto: Vec<Cryptocurrency>,
        sources: Vec<Source>,
    ) {
        let crypto: HashMap<String, i32> = crypto.into_iter().map(|c| (c.name, c.id)).collect();
        let sources: HashMap<String, i32> = sources.into_iter().map(|s| (s.name, s.id)).collect();

        for notification in notifications {
            let parsed_notification = self.parse_notification(&crypto, &sources, notification);
            if parsed_notification.is_none() {
                continue;
            }
            let parsed_notification = parsed_notification.unwrap();
            if let Some(true) = parsed_notification.verify_applicable(&self.crypto_db).await {
                println!(
                    "Sending notification to user {} for notification {}",
                    parsed_notification.user_id, parsed_notification.name
                );
                let connection = self.user_db.db_connection.get().await;
                if connection.is_err() {
                    println!("Failed to get connection to user db");
                    continue;
                }

                let user = csb_db_user::models::User::by_id(
                    &mut connection.unwrap(),
                    parsed_notification.user_id,
                )
                .await;
                if user.is_err() {
                    println!("Failed to get user {}", parsed_notification.user_id);
                    continue;
                }
                let user = user.unwrap();

                if user.is_none() {
                    println!("User {} does not exist", parsed_notification.user_id);
                    continue;
                }

                if send_email_brevo(&self.brello_api_key, &parsed_notification, &user.unwrap())
                    .await
                    .is_none()
                {
                    println!(
                        "Failed to send notification to user {}",
                        parsed_notification.user_id
                    );
                }
            }
        }
    }

    fn parse_notification(
        &self,
        crypto: &HashMap<String, i32>,
        sources: &HashMap<String, i32>,
        notification: Notification,
    ) -> Option<ParsedNotification> {
        let crypto_id = *crypto.get(&notification.cryptocurrency)?;
        let source_id = if let Some(id) = sources.get(&notification.source) {
            SourceType::Source(*id)
        } else {
            SourceType::from_str(&notification.source)?
        };
        let notification_type = NotificationType::from_str(&notification.type_)?;

        let value = match notification_type {
            NotificationType::PriceChange => notification.value_change?,
            NotificationType::PercentChange => notification.percent_change?,
        };

        Some(ParsedNotification {
            user_id: notification.user_id,
            crypto_id,
            notification_type,
            source: source_id,
            value,
            name: notification.name.clone(),
            price: notification.current_price,
        })
    }
}

fn email_text(notification: &ParsedNotification, user: &User) -> String {
    let user = &user.login;
    let notification_name = &notification.name;
    format!(
        "<html><head></head><body><p>Hello, {user}</p>Your '{notification_name}' notification has alerted. Please check it.</p></body></html>"
    )
}

async fn send_email_brevo(
    api_key: &str,
    notification: &ParsedNotification,
    user: &User,
) -> Option<()> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/json".parse().ok()?);
    headers.insert("api-key", api_key.parse().ok()?);
    headers.insert("content-type", "application/json".parse().ok()?);

    let body = json!({
        "sender": {
            "name": "Coin Sight Bot",
            "email": "artur.yurii.korchynskyi@gmail.com"
        },
        "to": [
            {
                "email": user.email.clone(),
                "name": user.login.clone()
            }
        ],
        "subject": "Notification triggered",
        "htmlContent": email_text(notification, user)
    });

    let res = client
        .post("https://api.brevo.com/v3/smtp/email")
        .headers(headers)
        .json(&body)
        .send()
        .await
        .ok()?;

    if res.status().is_success() {
        Some(())
    } else {
        println!("Failed to send email: {:?}", res.text().await);
        None
    }
}
