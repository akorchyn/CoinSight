use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable)]
pub struct TelegramAuth {
    pub id: i32,
    pub user_id: i32,
    pub telegram_id: Option<i64>,
    pub auth_code: String,
}

impl TelegramAuth {
    pub async fn set_auth_code(
        connection: &mut AsyncPgConnection,
        user: i32,
        auth_code: &str,
    ) -> QueryResult<()> {
        use crate::schema::telegram_auth::dsl::{auth_code as auth_code_column, telegram_auth};

        diesel::insert_into(telegram_auth)
            .values((
                auth_code_column.eq(auth_code),
                crate::schema::telegram_auth::dsl::user_id.eq(user),
                crate::schema::telegram_auth::dsl::telegram_id.eq(Option::<i64>::None),
            ))
            .on_conflict(crate::schema::telegram_auth::dsl::user_id)
            .do_update()
            .set(auth_code_column.eq(auth_code))
            .execute(connection)
            .await?;

        Ok(())
    }

    pub async fn update_telegram_id(
        &self,
        connection: &mut AsyncPgConnection,
        telegram_id: i64,
    ) -> QueryResult<()> {
        use crate::schema::telegram_auth::dsl::{
            auth_code, telegram_auth, telegram_id as telegram_id_column,
        };

        diesel::update(telegram_auth)
            .set((telegram_id_column.eq(telegram_id), auth_code.eq("")))
            .filter(crate::schema::telegram_auth::dsl::id.eq(self.id))
            .execute(connection)
            .await?;

        Ok(())
    }

    pub async fn by_user(
        connection: &mut AsyncPgConnection,
        user: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::telegram_auth::dsl::{telegram_auth, user_id};

        telegram_auth
            .filter(user_id.eq(user))
            .first(connection)
            .await
            .optional()
    }

    pub async fn by_auth_code(
        connection: &mut AsyncPgConnection,
        auth_code: &str,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::telegram_auth::dsl::{auth_code as auth_code_column, telegram_auth};

        telegram_auth
            .filter(auth_code_column.eq(auth_code))
            .first(connection)
            .await
            .optional()
    }
}
