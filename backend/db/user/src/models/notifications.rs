use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::notifications;

#[derive(Queryable)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub type_: String,
    pub source: Option<String>,
    pub value_change: Option<BigDecimal>,
    pub percent_change: Option<BigDecimal>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub user_id: i32,
    pub type_: String,
    pub source: Option<String>,
    pub value_change: Option<BigDecimal>,
    pub percent_change: Option<BigDecimal>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
