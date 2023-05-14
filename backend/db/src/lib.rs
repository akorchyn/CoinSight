use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use error::Error;

pub use diesel;

pub mod error;
pub mod models;
pub mod schema;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Context {
    pub db_connection: DbPool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(connection_string: String) -> Result<Context, Error> {
        Ok(Context {
            db_connection: establish_pool_connection(connection_string)?,
        })
    }
}

pub fn establish_pool_connection(database_url: String) -> Result<DbPool, Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(Pool::builder().build(manager)?)
}
