use juniper::graphql_value;

pub mod mutation;
pub mod query;
pub mod types;

pub struct UserService {
    url: String,
}

impl UserService {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn client(
        &self,
    ) -> Result<
        csb_comm::user_service_client::UserServiceClient<tonic::transport::Channel>,
        Box<dyn std::error::Error>,
    > {
        Ok(csb_comm::user_service_client::UserServiceClient::connect(self.url.clone()).await?)
    }
}

pub struct Context {
    pub user_db: csb_db_user::Db,
    pub crypto_db: csb_db_crypto::Db,
    pub user_service: UserService,
}

impl juniper::Context for Context {}

pub type Schema = juniper::RootNode<
    'static,
    query::Query,
    mutation::Mutation,
    juniper::EmptySubscription<Context>,
>;

fn grpc_error_to_field_error(e: tonic::Status) -> juniper::FieldError {
    juniper::FieldError::new(
        e.message().to_string(),
        graphql_value!({
            "code": e.code().to_string()
        }),
    )
}
