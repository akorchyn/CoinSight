use chrono::NaiveDateTime;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct JWTToken {
    pub token: String,
    pub expires_at: NaiveDateTime,
}

impl JWTToken {
    pub fn new(token: String, expires_at: NaiveDateTime) -> Self {
        Self { token, expires_at }
    }
}

#[derive(GraphQLObject)]
pub struct Notification {
    pub id: i32,
    pub coin_name: String,
    pub source: String,
    pub change_type: String,
    pub change_value: String,
    pub current_price: String,
    pub name: String,
}

impl From<csb_comm::notifications::NotificationData> for Notification {
    fn from(notification: csb_comm::notifications::NotificationData) -> Self {
        Self {
            id: notification.id as i32,
            coin_name: notification.coin_name,
            source: notification.source,
            change_type: notification.change_type,
            change_value: notification.change_value,
            current_price: notification.current_price,
            name: notification.name,
        }
    }
}
