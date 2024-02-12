use ethers::{
    contract::ContractError,
    providers::{Middleware, ProviderError},
};
use thiserror::Error;

use std::num::TryFromIntError;

#[derive(Error, Debug)]
pub enum MessagingOperationError<M: Middleware> {
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error("Error converting from int: {0}")]
    IntConversion(#[from] TryFromIntError),
}
