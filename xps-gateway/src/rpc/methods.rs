//! Interface Implementations for XPS JSON-RPC

use super::api::*;

use async_trait::async_trait;
use jsonrpsee::types::ErrorObjectOwned;

use crate::types::{GrantInstallationResult, Message, Signature};

/// Gateway Methods for XPS
pub struct XpsMethods;

#[async_trait]
impl XpsServer for XpsMethods {
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned> {
        //TODO: Stub for sendMessage, ref: [discussion](https://github.com/xmtp/xps-gateway/discussions/11)
        log::debug!("xps_sendMessage called");
        todo!();
    }

    async fn grant_installation(
        &self,
        _did: String,
        _name: String,
        _value: String,
        _signature: Signature,
    ) -> Result<GrantInstallationResult, ErrorObjectOwned> {
        /*if name.len() > 32 {
            Err(GrantInstallationResult {})
        }*/

        let result = GrantInstallationResult {
            status: "my-status".to_string(),
            message: "my message".to_string(),
            transaction: "my transaction".to_string(),
        };
        Ok(result)
    }
}
