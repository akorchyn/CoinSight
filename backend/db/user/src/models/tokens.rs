use crate::schema::tokens;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl Token {
    pub async fn by_user_token(
        connection: &mut AsyncPgConnection,
        user: i32,
        token: &str,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::tokens::dsl::{token as token_column, tokens, user_id};

        tokens
            .filter(user_id.eq(user))
            .filter(token_column.eq(token))
            .first(connection)
            .await
            .optional()
    }

    pub async fn remove_outdated(
        connection: &mut AsyncPgConnection,
        timestamp: NaiveDateTime,
    ) -> QueryResult<usize> {
        use crate::schema::tokens::dsl::*;

        diesel::delete(tokens.filter(expires_at.lt(timestamp)))
            .execute(connection)
            .await
    }

    pub async fn remove_user_token(
        connection: &mut AsyncPgConnection,
        user: i32,
        token: &str,
    ) -> QueryResult<usize> {
        use crate::schema::tokens::dsl::{token as token_column, tokens, user_id};

        diesel::delete(
            tokens
                .filter(user_id.eq(user))
                .filter(token_column.eq(token)),
        )
        .execute(connection)
        .await
    }
}

#[derive(Insertable)]
#[diesel(table_name = tokens)]
pub struct NewToken {
    pub user_id: i32,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl NewToken {
    pub async fn insert(&self, connection: &mut AsyncPgConnection) -> QueryResult<usize> {
        use crate::schema::tokens::dsl::*;

        diesel::insert_into(tokens)
            .values(self)
            .execute(connection)
            .await
    }
}
