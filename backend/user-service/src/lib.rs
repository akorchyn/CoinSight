use std::str::FromStr;

use csb_comm::{
    notifications::NotificationData, EditNotification, Login, LoginResponse, Notification,
    Notifications, Register, RemoveNotification, Token,
};

use csb_db_user::AsyncPgConnection;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use rand::Rng;
use tonic::{Request, Response, Status};

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    user_id: i32,
    exp: i64,
}

pub struct UserService {
    context: csb_db_user::Db,
    key: hmac::Hmac<sha2::Sha256>,
}

impl UserService {
    pub fn new(context: csb_db_user::Db, key: Vec<u8>) -> Self {
        let key = Hmac::new_from_slice(&key).expect("HMAC can take key of any size");
        Self { context, key }
    }
}

#[tonic::async_trait]
impl csb_comm::user_service_server::UserService for UserService {
    async fn login(&self, request: Request<Login>) -> Result<Response<LoginResponse>, Status> {
        let Login { email, password } = request.into_inner();
        let mut connection = self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?;
        let user = csb_db_user::models::User::by_email(&mut connection, &email)
            .await
            .map_err(|_| Status::internal("Database failure"))?
            .ok_or(Status::not_found("Incorrect email/password"))?;

        let result = argon2::verify_encoded(&user.password_hash, password.as_bytes())
            .map_err(|_| Status::internal("Error while verifying password hash"))?;
        if !result {
            return Err(Status::unauthenticated("Incorrect email/password"));
        }

        let login_response = create_token_for_user(user.id, &self.key, &mut connection).await?;
        Ok(Response::new(login_response))
    }

    async fn register(&self, request: Request<Register>) -> Result<Response<()>, Status> {
        let Register {
            login,
            email,
            password,
        } = request.into_inner();
        if !check_if_mail_is_valid(&email) {
            return Err(Status::invalid_argument("Invalid email"));
        }
        if password.len() < 8 {
            return Err(Status::invalid_argument("Password too short"));
        }
        let random_salt = rand::thread_rng().gen::<[u8; 16]>();
        let password_hash =
            argon2::hash_encoded(password.as_bytes(), &random_salt, &Default::default())
                .map_err(|_| Status::internal("Error while hashing password"))?;

        let new_user = csb_db_user::models::NewUser {
            login,
            email,
            password_hash,
            default_notification_method: "".to_string(),
        };
        let mut connection = self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?;

        if csb_db_user::models::User::by_email(&mut connection, &new_user.email)
            .await
            .map_err(|_| Status::internal("Database failure"))?
            .is_some()
        {
            return Err(Status::already_exists(
                "User with this email already exists",
            ));
        }

        if csb_db_user::models::User::by_login(&mut connection, &new_user.login)
            .await
            .map_err(|_| Status::internal("Database failure"))?
            .is_some()
        {
            return Err(Status::already_exists(
                "User with this login already exists",
            ));
        }

        let updated_rows = new_user
            .insert(&mut connection)
            .await
            .map_err(|_| Status::internal("Failed to insert into database"))?;
        if updated_rows == 0 {
            return Err(Status::internal("Failed to insert into database"));
        }

        Ok(tonic::Response::new(()))
    }

    async fn logout(&self, request: Request<Token>) -> Result<Response<()>, Status> {
        let token = request.into_inner().token;
        let claims: Claims = token
            .verify_with_key(&self.key)
            .map_err(|_| Status::unauthenticated("Invalid token"))?;
        let mut connection = self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?;
        csb_db_user::models::Token::remove_user_token(&mut connection, claims.user_id, &token)
            .await
            .map_err(|_| Status::internal("Error while removing token from the database"))?;
        Ok(Response::new(()))
    }

    async fn refresh_token(
        &self,
        request: Request<Token>,
    ) -> Result<Response<LoginResponse>, Status> {
        let token = request.into_inner().token;

        let claims = self.validate(token.clone()).await?; // Validate token first
        let mut connection = self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?; // Get connection from the pool
        let login_response =
            create_token_for_user(claims.user_id, &self.key, &mut connection).await?;
        self.logout(Request::new(Token {
            token: token.clone(),
        }))
        .await?; // Revoke old token

        Ok(Response::new(login_response))
    }

    async fn validate_token(&self, request: Request<Token>) -> Result<Response<()>, Status> {
        let token = request.into_inner().token;
        self.validate(token).await?;
        Ok(Response::new(()))
    }

    async fn add_notification(
        &self,
        request: Request<Notification>,
    ) -> Result<Response<()>, Status> {
        let Notification {
            token,
            coin_name,
            source,
            change_type,
            change_value,
            current_price,
            name,
        } = request.into_inner();

        let Claims { user_id, .. } = self.validate(token).await?;
        let db_connection = &mut self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?;

        let current_price = bigdecimal::BigDecimal::from_str(&current_price)
            .map_err(|_| Status::invalid_argument("Invalid current price"))?;
        let change_value = bigdecimal::BigDecimal::from_str(&change_value)
            .map_err(|_| Status::invalid_argument("Invalid change value"))?;
        let (type_, value_change, percent_change) = match change_type.as_str() {
            "by Value" => (change_type, Some(change_value), None),
            "by Percent" => (change_type, None, Some(change_value)),
            _ => return Err(Status::invalid_argument("Invalid change type")),
        };

        let notification = csb_db_user::models::NewNotification {
            name,
            user_id,
            cryptocurrency: coin_name,
            source,
            value_change,
            percent_change,
            type_,
            current_price,
        };
        notification.insert(db_connection).await.map_err(|_| {
            Status::internal("Error. Failed to insert notification into the database")
        })?;
        Ok(Response::new(()))
    }

    async fn remove_notification(
        &self,
        request: Request<RemoveNotification>,
    ) -> Result<Response<()>, Status> {
        let RemoveNotification { token, id } = request.into_inner();
        let Claims { user_id, .. } = self.validate(token).await?;
        let db_connection = &mut self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?;
        let updated =
            csb_db_user::models::Notification::remove_by_id(id as i32, user_id, db_connection)
                .await
                .map_err(|_| {
                    Status::internal("Error. Failed to remove notification from the database")
                })?;
        if updated == 0 {
            return Err(Status::not_found("Notification not found"));
        }
        Ok(Response::new(()))
    }

    async fn notifications(
        &self,
        request: Request<Token>,
    ) -> Result<Response<Notifications>, Status> {
        let token = request.into_inner().token;
        let Claims { user_id, .. } = self.validate(token).await?;
        let db_connection = &mut self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?;
        let notifications =
            csb_db_user::models::Notification::all_from_user(user_id, db_connection)
                .await
                .map_err(|_| {
                    Status::internal("Error. Failed to get notifications from the database")
                })?;
        let notifications = notifications
            .into_iter()
            .map(|notification| NotificationData {
                id: notification.id as i64,
                coin_name: notification.cryptocurrency,
                source: notification.source,
                change_type: notification.type_,
                change_value: notification
                    .value_change
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| notification.percent_change.unwrap().to_string()),
                current_price: notification.current_price.to_string(),
                name: notification.name,
            })
            .collect();
        Ok(Response::new(Notifications { notifications }))
    }

    async fn edit_notification(
        &self,
        request: Request<EditNotification>,
    ) -> Result<Response<()>, Status> {
        let EditNotification {
            token,
            id,
            coin_name,
            source,
            change_type,
            change_value,
            current_price,
            name,
        } = request.into_inner();
        let Claims { user_id, .. } = self.validate(token).await?;
        let db_connection = &mut self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?;

        let current_price = bigdecimal::BigDecimal::from_str(&current_price)
            .map_err(|_| Status::invalid_argument("Invalid current price"))?;
        let change_value = bigdecimal::BigDecimal::from_str(&change_value)
            .map_err(|_| Status::invalid_argument("Invalid change value."))?;
        let (type_, value_change, percent_change) = match change_type.as_str() {
            "by Value" => (change_type, Some(change_value), None),
            "by Percent" => (change_type, None, Some(change_value)),
            _ => return Err(Status::invalid_argument("Invalid change type")),
        };
        let mut notification =
            csb_db_user::models::Notification::by_id(id as i32, user_id, db_connection)
                .await
                .map_err(|_| Status::not_found("Notification not found"))?;
        notification.cryptocurrency = coin_name;
        notification.source = source;
        notification.type_ = type_;
        notification.value_change = value_change;
        notification.percent_change = percent_change;
        notification.current_price = current_price;
        notification.name = name;
        notification
            .update(db_connection)
            .await
            .map_err(|_| Status::internal("Error. Failed to update notification"))?;
        Ok(Response::new(()))
    }
}

impl UserService {
    async fn validate(&self, token: String) -> Result<Claims, Status> {
        let claims: Claims = token
            .verify_with_key(&self.key)
            .map_err(|_| Status::unauthenticated("Invalid token"))?;
        let mut connection = self
            .context
            .db_connection
            .get()
            .await
            .map_err(|_| Status::internal("Error while getting connection from the pool"))?;
        let now = chrono::Utc::now().naive_utc();
        if now.timestamp() > claims.exp {
            csb_db_user::models::Token::remove_outdated(&mut connection, now)
                .await
                .map_err(|_| Status::internal("Error while removing outdated tokens"))?;
            return Err(Status::unauthenticated("Token expired"));
        }
        let token_valid =
            csb_db_user::models::Token::by_user_token(&mut connection, claims.user_id, &token)
                .await
                .map_err(|_| Status::internal("Error while getting token from the database"))?
                .is_some();
        if !token_valid {
            return Err(Status::unauthenticated("Revoked token"));
        }

        Ok(claims)
    }
}

async fn create_token_for_user(
    user_id: i32,
    key: &Hmac<sha2::Sha256>,
    connection: &mut AsyncPgConnection,
) -> Result<LoginResponse, Status> {
    let now = chrono::Utc::now().naive_utc();
    let expiration = now + chrono::Duration::days(1);
    let claims = Claims {
        user_id,
        exp: expiration.timestamp(),
    };

    let token_str = claims
        .sign_with_key(key)
        .map_err(|_| Status::internal("Error while signing token"))?;

    let new_token = csb_db_user::models::NewToken {
        user_id,
        token: token_str.clone(),
        created_at: now,
        expires_at: expiration,
    };

    new_token
        .insert(connection)
        .await
        .map_err(|_| Status::internal("Error while inserting token into the database"))?;

    Ok(LoginResponse {
        token: token_str,
        expires_at: expiration.timestamp(),
    })
}

fn check_if_mail_is_valid(email: &str) -> bool {
    let index = email.find('@');
    match index {
        None => false,
        Some(index) if index == 0 => false,
        Some(index) if index == email.len() - 1 => false,
        Some(_) => true,
    }
}
