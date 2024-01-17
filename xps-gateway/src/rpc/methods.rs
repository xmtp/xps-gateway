//! Interface Implementations for XPS JSON-RPC

use super::api::*;

use async_trait::async_trait;
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};

use crate::types::{GrantInstallationResult, Message, Signature};
use registry::XpsRegistry;

/// Gateway Methods for XPS
pub struct XpsMethods {
    pub registry: XpsRegistry,
}

#[async_trait]
impl XpsServer for XpsMethods {
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned> {
        //TODO: Stub for sendMessage, ref: [discussion](https://github.com/xmtp/xps-gateway/discussions/11)
        log::debug!("xps_sendMessage called");
        todo!();
    }

    async fn grant_installation(
        &self,
        did: String,
        name: String,
        value: String,
        signature: Signature,
    ) -> Result<GrantInstallationResult, ErrorObjectOwned> {
        // perform data validation on the request parameters.
        if name.len() > 32 {
            return Err(ErrorObject::borrowed(
                -31001,
                "name field was longer than 32 bytes",
                None,
            ));
        };
        if value.len() > 4096 {
            return Err(ErrorObject::borrowed(
                -31002,
                "value field was longer than 4096 bytes",
                None,
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
}

/// Convenience function to convert an anyhow::Error into an ErrorObjectOwned.
fn into_error_object(error: anyhow::Error) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(-31000, error.to_string(), None::<()>)
}
