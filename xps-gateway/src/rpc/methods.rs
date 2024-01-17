//! Interface Implementations for XPS JSON-RPC

use super::api::*;
use jsonrpsee::types::error::ErrorCode;

use async_trait::async_trait;
use jsonrpsee::types::ErrorObjectOwned;

use crate::types::Message;

/// Gateway Methods for XPS
pub struct XpsMethods;

impl XpsMethods {
    /// Create a new instance of the XpsMethods struct
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for XpsMethods {
    fn default() -> Self {
        Self::new()
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
}
