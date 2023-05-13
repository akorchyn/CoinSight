#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DbConnectionError(#[from] diesel::ConnectionError),
    #[error(transparent)]
    PoolCreationError(#[from] diesel::r2d2::PoolError),
}
