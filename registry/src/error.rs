use ethers::{
    contract::ContractError,
    providers::{Middleware, ProviderError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContactOperationError<M: Middleware> {
    #[error("Invalid address {0}")]
    BadAddress(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}
