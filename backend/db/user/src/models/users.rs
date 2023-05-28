use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub email: String,
    pub password_hash: String,
    pub default_notification_method: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub async fn by_email(
        connection: &mut AsyncPgConnection,
        email: &str,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::users::dsl::{email as email_column, users};

        users
            .filter(email_column.eq(email))
            .first(connection)
            .await
            .optional()
    }

    pub async fn by_login(
        connection: &mut AsyncPgConnection,
        login: &str,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::users::dsl::{login as login_column, users};

        users
            .filter(login_column.eq(login))
            .first(connection)
            .await
            .optional()
    }

    pub async fn by_id(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::users::dsl::{id as id_column, users};

        users
            .filter(id_column.eq(id))
            .first(connection)
            .await
            .optional()
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub login: String,
    pub email: String,
    pub password_hash: String,
    pub default_notification_method: String,
}

impl NewUser {
    pub async fn insert(&self, connection: &mut AsyncPgConnection) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(self)
            .on_conflict_do_nothing()
            .execute(connection)
            .await
    }
}
