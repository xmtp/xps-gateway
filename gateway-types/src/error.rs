use ethers::{
    abi::EncodePackedError,
    contract::ContractError,
    providers::{Middleware, ProviderError},
    signers::WalletError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtSignerError<M: Middleware> {
    #[error(transparent)]
    Encode(#[from] EncodePackedError),
    #[error("{0}")]
    ContractError(#[from] ContractError<M>),
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error(transparent)]
    Wallet(#[from] WalletError),
}
