pub mod mutation;
pub mod query;
pub mod subscription;

pub struct Context {
    pub db_connection: csb_db::DbPool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(connection_string: String) -> Result<Context, csb_db::error::Error> {
        Ok(Context {
            db_connection: csb_db::establish_pool_connection(connection_string)?,
        })
    }
}

pub type Schema =
    juniper::RootNode<'static, query::Query, mutation::Mutation, subscription::Subscription>;
