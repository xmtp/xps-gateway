use ethers::{
    contract::ContractError,
    providers::{Middleware, ProviderError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessagingOperationError<M: Middleware> {
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}
