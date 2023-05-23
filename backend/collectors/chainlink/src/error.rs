use ethers::{
    prelude::{ContractError, Http},
    providers::{Provider, ProviderError},
};

/// The error type for the oracle.
#[derive(thiserror::Error, Debug)]
pub enum CollectorError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    ContractError(#[from] ContractError<Provider<Http>>),

    #[error("Latest price is negative for {0} token")]
    NegativeLatestPrice(String),

    #[error(transparent)]
    DbError(#[from] csb_db_crypto::diesel::result::Error),
}
