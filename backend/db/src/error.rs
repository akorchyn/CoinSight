#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DbConnectionError(#[from] diesel::ConnectionError),
    #[error(transparent)]
    PoolCreationError(#[from] diesel_async::pooled_connection::PoolError),
}
