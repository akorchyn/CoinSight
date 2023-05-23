use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct NotificationPreference {
    pub id: i32,
    pub notification_id: i32,
    pub notification_method: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
