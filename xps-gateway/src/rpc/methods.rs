//! Interface Implementations for XPS JSON-RPC

use crate::types::{GatewayContext, GatewaySigner};

use super::api::*;
use jsonrpsee::types::error::ErrorCode;

use async_trait::async_trait;
use ethers::prelude::*;
use ethers::{core::types::Signature, providers::Middleware};
use gateway_types::GrantInstallationResult;
use jsonrpsee::types::ErrorObjectOwned;
use lib_didethresolver::types::XmtpAttribute;

use rand::{rngs::StdRng, SeedableRng};
use std::sync::Arc;
use thiserror::Error;

use gateway_types::Message;
use registry::{error::ContactOperationError, ContactOperations};

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
        let block_number = self.signer.get_block_number().await.unwrap();
        let validity_period: U64 = U64::from(60 * 60 * 24 * 365 / 5); // number of round in one year, assuming 5-second round.
        let validity = block_number + validity_period;

        let result = self
            .contact_operations
            .grant_installation(did, name, value, signature, U256::from(validity.as_u64()))
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
        println!("{:?}", error);
        match error {
            RpcError::ContactOperation(c) => {
                ErrorObjectOwned::owned(-31999, c.to_string(), None::<()>)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::MockProviderExt;
    use ethers::providers::MockResponse;
    use lib_didethresolver::types::{KeyEncoding, XmtpKeyPurpose};

    use super::*;

    fn type_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[tokio::test]
    async fn test_rpc_wallet_address() {
        let (context, _) = GatewayContext::mocked().await;
        let methods = XpsMethods::new(&context);

        let res = methods.wallet_address().await.unwrap();
        assert_eq!(type_of(res), "primitive_types::H160");
    }

    #[tokio::test]
    async fn test_rpc_revoke_installation() {
        let (context, mut mock) = GatewayContext::mocked().await;

        let methods = XpsMethods::new(&context);

        let attr = XmtpAttribute {
            encoding: KeyEncoding::Hex,
            purpose: XmtpKeyPurpose::Installation,
        };
        let value = vec![0x01, 0x02, 0x03];

        mock.set_contract_response(Default::default());

        let res = methods
            .revoke_installation(
                "0x7e575682a8e450e33eb0493f9972821ae333cd7f".to_string(),
                attr,
                value,
                Signature {
                    r: [0x01; 32].into(),
                    s: [0x02; 32].into(),
                    v: 0x01,
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_rpc_revoke_installation_error() {
        let (context, mut mock) = GatewayContext::mocked().await;

        let methods = XpsMethods::new(&context);

        let attr = XmtpAttribute {
            encoding: KeyEncoding::Hex,
            purpose: XmtpKeyPurpose::Installation,
        };
        let value = vec![0x01, 0x02, 0x03];

        mock.push_response(MockResponse::Error(JsonRpcError {
            code: -32000,
            message: "VM Exception while processing transaction: revert".to_string(),
            data: None,
        }));

        let res = methods
            .revoke_installation(
                "0x7e575682a8e450e33eb0493f9972821ae333cd7f".to_string(),
                attr,
                value,
                Signature {
                    r: [0x01; 32].into(),
                    s: [0x02; 32].into(),
                    v: 0x01,
                },
            )
            .await;
        println!("{:?}", res);
    }
}
