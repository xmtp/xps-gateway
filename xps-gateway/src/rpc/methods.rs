//! Interface Implementations for XPS JSON-RPC

use crate::types::{GatewayContext, GatewaySigner};

use super::api::*;
use jsonrpsee::types::error::ErrorCode;

use async_trait::async_trait;
use ethers::prelude::*;
use ethers::utils::format_units;
use ethers::{core::types::BlockId, core::types::Signature, providers::Middleware};
use gateway_types::{GrantInstallationResult, WalletBalance};
use jsonrpsee::types::ErrorObjectOwned;
use lib_didethresolver::types::XmtpAttribute;
use rand::{rngs::StdRng, SeedableRng};
use std::sync::Arc;
use thiserror::Error;

use gateway_types::Message;
use registry::{error::ContactOperationError, ContactOperations};

// DEFAULT_ATTRIBUTE_VALIDITY is the hard-coded value we use for the validity of the attributes we set.
// This value is interpeted as number of seconds starting from the block where the attribute is being set.
pub const DEFAULT_ATTRIBUTE_VALIDITY: u64 = 60 * 60 * 24 * 365;

/// Gateway Methods for XPS
pub struct XpsMethods<P: Middleware + 'static> {
    contact_operations: ContactOperations<GatewaySigner<P>>,
    pub wallet: LocalWallet,
    pub signer: Arc<GatewaySigner<P>>,
}

impl<P: Middleware> XpsMethods<P> {
    pub fn new(context: &GatewayContext<P>) -> Self {
        Self {
            contact_operations: ContactOperations::new(context.registry.clone()),
            wallet: LocalWallet::new(&mut StdRng::from_entropy()),
            signer: context.signer.clone(),
        }
    }
}

#[async_trait]
impl<P: Middleware + 'static> XpsServer for XpsMethods<P> {
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned> {
        //TODO: Stub for sendMessage, ref: [discussion](https://github.com/xmtp/xps-gateway/discussions/11)
        log::debug!("xps_sendMessage called");
        Err(ErrorCode::MethodNotFound.into())
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
            .get_balance(self.wallet.address(), Option::<BlockId>::None)
            .await
            .unwrap();

        // Convert the balance from wei to Ether, formatting the result as a string.
        let ether_balance =
            format_units(wei_balance, 18) // 18 decimal places for Ether
                .unwrap_or_else(|_| "failed to convert balance".to_string());

        // Return the balance in Ether as a WalletBalance object.
        Ok(WalletBalance {
            balance: format!("{} ETH", ether_balance),
            unit: "ETH".to_string(),
        })
    }
}

/// Error types for DID Registry JSON-RPC
#[derive(Debug, Error)]
enum RpcError<M: Middleware> {
    /// A public key parameter was invalid
    #[error(transparent)]
    ContactOperation(#[from] ContactOperationError<M>),
}

impl<M: Middleware> From<RpcError<M>> for ErrorObjectOwned {
    fn from(error: RpcError<M>) -> Self {
        match error {
            RpcError::ContactOperation(c) => {
                ErrorObjectOwned::owned(-31999, c.to_string(), None::<()>)
            }
        }
    }
}
