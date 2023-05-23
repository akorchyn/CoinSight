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
