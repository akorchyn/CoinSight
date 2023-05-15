use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

use error::Error;

pub use diesel;

pub mod error;
pub mod models;
pub mod schema;

pub type DbPool = Pool<AsyncPgConnection>;

pub struct Context {
    pub db_connection: DbPool,
}

impl juniper::Context for Context {}

impl Context {
    pub async fn new(connection_string: String) -> Result<Context, Error> {
        Ok(Context {
            db_connection: establish_pool_connection(connection_string).await?,
        })
    }
}

pub async fn establish_pool_connection(database_url: String) -> Result<DbPool, Error> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder().build(config).await?;
    Ok(pool)
}
