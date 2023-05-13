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

pub fn establish_pool_connection(database_url: String) -> Result<DbPool, Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(Pool::builder().build(manager)?)
}
