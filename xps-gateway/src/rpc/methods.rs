//! Interface Implementations for XPS JSON-RPC

use super::api::*;
use jsonrpsee::types::error::ErrorCode;

use async_trait::async_trait;
use ethers::prelude::*;
use jsonrpsee::types::ErrorObjectOwned;
use rand::{rngs::StdRng, SeedableRng};

use crate::types::{GrantInstallationResult, Message, Signature};
use ethers::providers::{Http, Provider};
use registry::XpsRegistry;

/// Gateway Methods for XPS
pub struct XpsMethods {
    pub registry: XpsRegistry,
    pub wallet: LocalWallet,
}

impl XpsMethods {
    /// Create a new instance of the XpsMethods struct
    pub fn new(provider: Provider<Http>, registry_address: String) -> Self {
        Self {
            registry: registry::XpsRegistry::new(provider, registry_address),
            wallet: LocalWallet::new(&mut StdRng::from_entropy()),
        }
    }
}

/*impl Default for XpsMethods {
    fn default() -> Self {
        Self::new()
    }
}*/

#[async_trait]
impl XpsServer for XpsMethods {
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned> {
        //TODO: Stub for sendMessage, ref: [discussion](https://github.com/xmtp/xps-gateway/discussions/11)
        log::debug!("xps_sendMessage called");
        Err(ErrorCode::MethodNotFound.into())
    }

    async fn status(&self) -> Result<String, ErrorObjectOwned> {
        log::debug!("xps_status called");
        Ok("OK".to_string())
    }

    /// Asynchronously grants installation access in the registry.
    ///
    /// This function is responsible for granting a new installation based on the provided
    /// decentralized identifier (DID), name, value, and signature. It performs validation
    /// checks on the input parameters (name and value lengths) and then invokes the
    /// registry call to grant installation access.
    ///
    /// # Arguments
    /// * `did` - A `String` representing the decentralized identifier.
    /// * `name` - A `String` representing the name of the installation, limited to 32 bytes.
    /// * `value` - A `String` representing the value associated with the installation, limited to 4096 bytes.
    /// * `signature` - A `Signature` struct representing the signature for the operation.
    ///
    /// # Returns
    /// This function returns a `Result` which, on success, includes a `GrantInstallationResult`
    /// containing the status, message, and transaction details of the operation.
    ///
    /// # Errors
    /// Returns `ErrorObjectOwned` if input validation fails (e.g., if the name or value exceeds their respective length limits).
    ///
    /// # Examples
    /// ```
    /// // Assume the function is part of an implementation and proper context is set
    /// let result = obj.grant_installation(
    ///     "did:example:123".to_string(),
    ///     "installName".to_string(),
    ///     "installationValue".to_string(),
    ///     signature,
    /// ).await;
    /// ```
    async fn grant_installation(
        &self,
        did: String,
        name: String,
        value: String,
        signature: Signature,
    ) -> Result<GrantInstallationResult, ErrorObjectOwned> {
        // perform data validation on the request parameters.
        if name.len() > 32 {
            return Err(ErrorObjectOwned::owned(
                -31001,
                "name field was longer than 32 bytes",
                None::<()>,
            ));
        };
        if value.len() > 4096 {
            return Err(ErrorObjectOwned::owned(
                -31002,
                "value field was longer than 4096 bytes",
                None::<()>,
            ));
        }

        // if all good, invoke the call to the registry.
        let result = self
            .registry
            .grant_installation(
                did,
                name,
                value,
                registry::types::Signature {
                    v: signature.v,
                    r: signature.r,
                    s: signature.s,
                },
            )
            .await
            .map_err(into_error_object)?;

        Ok(GrantInstallationResult {
            status: result.status,
            message: result.message,
            transaction: result.transaction,
        })
    }

    async fn wallet_address(&self) -> Result<Address, ErrorObjectOwned> {
        Ok(self.wallet.address())
    }
}

/// Convenience function to convert an anyhow::Error into an ErrorObjectOwned.
fn into_error_object(error: anyhow::Error) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(-31000, error.to_string(), None::<()>)
}
