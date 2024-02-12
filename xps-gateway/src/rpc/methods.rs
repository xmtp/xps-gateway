//! Interface Implementations for XPS JSON-RPC

use crate::types::{GatewayContext, GatewaySigner};

use super::api::*;

use async_trait::async_trait;
use ethers::prelude::*;
use ethers::{
    core::types::Signature,
    providers::{Middleware, ProviderError},
};
use gateway_types::{
    GrantInstallationResult, KeyPackageResult, SendMessageResult, Unit, WalletBalance,
};
use jsonrpsee::types::ErrorObjectOwned;
use lib_didethresolver::types::XmtpAttribute;
use messaging::MessagingOperations;
use rand::{rngs::StdRng, SeedableRng};
use std::sync::Arc;
use thiserror::Error;

use gateway_types::Message;
use messaging::error::MessagingOperationError;
use registry::{error::ContactOperationError, ContactOperations};

// DEFAULT_ATTRIBUTE_VALIDITY is the hard-coded value we use for the validity of the attributes we set.
// This value is interpeted as number of seconds starting from the block where the attribute is being set.
pub const DEFAULT_ATTRIBUTE_VALIDITY: u64 = 60 * 60 * 24 * 365;

/// Gateway Methods for XPS
pub struct XpsMethods<P: Middleware + 'static> {
    message_operations: MessagingOperations<GatewaySigner<P>>,
    contact_operations: ContactOperations<GatewaySigner<P>>,
    pub wallet: LocalWallet,
    pub signer: Arc<GatewaySigner<P>>,
}

impl<P: Middleware> XpsMethods<P> {
    pub fn new(context: &GatewayContext<P>) -> Self {
        Self {
            message_operations: MessagingOperations::new(context.conversation.clone()),
            contact_operations: ContactOperations::new(context.registry.clone()),
            wallet: LocalWallet::new(&mut StdRng::from_entropy()),
            signer: context.signer.clone(),
        }
    }
}

#[async_trait]
impl<P: Middleware + 'static> XpsServer for XpsMethods<P> {
    async fn send_message(&self, message: Message) -> Result<SendMessageResult, ErrorObjectOwned> {
        let result = self
            .message_operations
            .send_message(message)
            .await
            .map_err(RpcError::from)?;

        Ok(result)
    }

    async fn status(&self) -> Result<String, ErrorObjectOwned> {
        log::debug!("xps_status called");
        Ok("OK".to_string())
    }

    async fn grant_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<GrantInstallationResult, ErrorObjectOwned> {
        log::debug!("xps_grantInstallation called");

        let result = self
            .contact_operations
            .grant_installation(
                did,
                name,
                value,
                signature,
                U256::from(DEFAULT_ATTRIBUTE_VALIDITY),
            )
            .await
            .map_err(RpcError::from)?;

        Ok(result)
    }

    async fn revoke_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<(), ErrorObjectOwned> {
        log::debug!("xps_revokeInstallation called");
        self.contact_operations
            .revoke_installation(did, name, value, signature)
            .await
            .map_err(RpcError::from)?;

        Ok(())
    }

    async fn wallet_address(&self) -> Result<Address, ErrorObjectOwned> {
        Ok(self.wallet.address())
    }

    /// Fetches the current balance of the wallet in Ether.
    ///
    /// This asynchronous method queries the Ethereum blockchain to get the current balance
    /// of the associated wallet address, converting the result from wei (the smallest unit
    /// of Ether) to Ether for more understandable reading.
    ///
    /// # Returns
    /// - `Ok(WalletBalance)`: On success, returns a `WalletBalance` struct containing the
    ///   wallet's balance formatted as a string in Ether, along with the unit "ETH".
    /// - `Err(ErrorObjectOwned)`: On failure, returns an error object detailing why the
    ///   balance could not be fetched or converted.
    ///
    async fn balance(&self) -> Result<WalletBalance, ErrorObjectOwned> {
        // Fetch the balance in wei (the smallest unit of Ether) from the blockchain.
        let wei_balance: U256 = self
            .signer
            .provider()
            .get_balance(self.wallet.address(), None)
            .await
            .map_err::<RpcError<P>, _>(RpcError::from)?;

        // Return the balance in Ether as a WalletBalance object.
        Ok(WalletBalance {
            balance: wei_balance,
            unit: Unit::Eth,
        })
    }

    async fn fetch_key_packages(&self, did: String) -> Result<KeyPackageResult, ErrorObjectOwned> {
        log::debug!("xps_fetchKeyPackages called");
        let result = self
            .contact_operations
            .fetch_key_packages(did)
            .await
            .map_err(RpcError::from)?;
        Ok(result)
    }
}

/// Error types for DID Registry JSON-RPC
#[derive(Debug, Error)]
enum RpcError<M: Middleware> {
    /// A public key parameter was invalid
    #[error(transparent)]
    ContactOperation(#[from] ContactOperationError<M>),
    /// error occured while querying the balance.
    #[error(transparent)]
    BalanceOperation(#[from] ProviderError),
    #[error(transparent)]
    MessagingOperation(#[from] MessagingOperationError<M>),
}

impl<M: Middleware> From<RpcError<M>> for ErrorObjectOwned {
    fn from(error: RpcError<M>) -> Self {
        match error {
            RpcError::ContactOperation(c) => {
                ErrorObjectOwned::owned(-31999, c.to_string(), None::<()>)
            }
            RpcError::BalanceOperation(c) => {
                ErrorObjectOwned::owned(-31999, c.to_string(), None::<()>)
            }
            RpcError::MessagingOperation(m) => {
                ErrorObjectOwned::owned(-31999, m.to_string(), None::<()>)
            }
        }
    }
}
