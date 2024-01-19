//! Interface Implementations for XPS JSON-RPC

use crate::types::{GatewayContext, GatewaySigner};

use super::api::*;
use jsonrpsee::types::error::ErrorCode;

use async_trait::async_trait;
use ethers::{core::types::Signature, providers::Middleware};
use jsonrpsee::types::ErrorObjectOwned;
use thiserror::Error;

use gateway_types::{Message, XmtpAttributeType};
use registry::{error::ContactOperationError, ContactOperations};

/// Gateway Methods for XPS
pub struct XpsMethods {
    contact_operations: ContactOperations<GatewaySigner>,
}

impl XpsMethods {
    pub fn new(context: &GatewayContext) -> Self {
        Self {
            contact_operations: ContactOperations::new(context.registry.clone()),
        }
    }
}

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

    async fn revoke_installation(
        &self,
        did: String,
        name: XmtpAttributeType,
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
