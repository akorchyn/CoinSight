use juniper::{graphql_object, FieldResult};

use crate::types::user;
use crate::Context;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn users() -> FieldResult<users::UserMutation> {
        Ok(users::UserMutation)
    }
}

mod users {

    use chrono::NaiveDateTime;
    use csb_comm::LoginResponse;

    use super::*;

    pub struct UserMutation;

    #[graphql_object(context = Context)]
    impl UserMutation {
        async fn register(email: String, password: String, context: &Context) -> FieldResult<bool> {
            let mut user_service = context.user_service.client().await?;
            let request = tonic::Request::new(csb_comm::Register { email, password });
            user_service.register(request).await?;
            Ok(true)
        }

        async fn login(
            email: String,
            password: String,
            context: &Context,
        ) -> FieldResult<user::JWTToken> {
            let mut user_service = context.user_service.client().await?;
            let request = tonic::Request::new(csb_comm::Login { email, password });
            let LoginResponse { token, expires_at } =
                user_service.login(request).await?.into_inner();
            let token = user::JWTToken::new(
                token,
                NaiveDateTime::from_timestamp_opt(expires_at, 0).ok_or("Invalid timestamp")?,
            );
            Ok(token)
        }

        async fn logout(context: &Context, token: String) -> FieldResult<bool> {
            let mut user_service = context.user_service.client().await?;
            let request = tonic::Request::new(csb_comm::Token { token });
            user_service.logout(request).await?;
            Ok(true)
        }

        async fn notifications(context: &Context, token: String) -> FieldResult<bool> {
            let mut user_service = context.user_service.client().await?;
            let request = tonic::Request::new(csb_comm::Token { token });
            user_service.validate_token(request).await?;
            Ok(true)
        }
    }
}
