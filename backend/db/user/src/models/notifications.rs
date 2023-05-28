use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use crate::schema::notifications;

#[derive(Queryable)]
#[diesel(table_name = notifications)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub type_: String,
    pub source: String,
    pub value_change: Option<BigDecimal>,
    pub percent_change: Option<BigDecimal>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
    pub cryptocurrency: String,
    pub current_price: BigDecimal,
    pub name: String,
}

impl Notification {
    pub async fn all(db_connection: &mut AsyncPgConnection) -> QueryResult<Vec<Notification>> {
        notifications::table
            .load::<Notification>(db_connection)
            .await
    }

    pub async fn all_from_user(
        user_id: i32,
        db_connection: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<Notification>> {
        notifications::table
            .filter(notifications::user_id.eq(user_id))
            .load::<Notification>(db_connection)
            .await
    }

    pub async fn remove_by_id(
        id: i32,
        user_id: i32,
        db_connection: &mut AsyncPgConnection,
    ) -> QueryResult<usize> {
        diesel::delete(
            notifications::table
                .filter(notifications::id.eq(id))
                .filter(notifications::user_id.eq(user_id)),
        )
        .execute(db_connection)
        .await
    }

    pub async fn by_id(
        id: i32,
        user_id: i32,
        db_connection: &mut AsyncPgConnection,
    ) -> QueryResult<Notification> {
        notifications::table
            .filter(notifications::id.eq(id))
            .filter(notifications::user_id.eq(user_id))
            .first::<Notification>(db_connection)
            .await
    }

    pub async fn update(&self, db_connection: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::update(
            notifications::table
                .filter(notifications::id.eq(self.id))
                .filter(notifications::user_id.eq(self.user_id)),
        )
        .set((
            notifications::type_.eq(self.type_.clone()),
            notifications::source.eq(self.source.clone()),
            notifications::value_change.eq(self.value_change.clone()),
            notifications::percent_change.eq(self.percent_change.clone()),
            notifications::cryptocurrency.eq(self.cryptocurrency.clone()),
            notifications::current_price.eq(self.current_price.clone()),
            notifications::updated_at.eq(self.updated_at),
            notifications::is_active.eq(self.is_active),
            notifications::name.eq(self.name.clone()),
        ))
        .execute(db_connection)
        .await
    }
}

#[derive(Insertable)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub user_id: i32,
    pub type_: String,
    pub source: String,
    pub value_change: Option<BigDecimal>,
    pub percent_change: Option<BigDecimal>,
    pub cryptocurrency: String,
    pub current_price: BigDecimal,
    pub name: String,
}

impl NewNotification {
    pub async fn insert(&self, db_connection: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::insert_into(notifications::table)
            .values(self)
            .execute(db_connection)
            .await
    }
}
