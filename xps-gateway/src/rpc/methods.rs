//! Interface Implementations for XPS JSON-RPC

use super::api::*;

use async_trait::async_trait;
use jsonrpsee::types::ErrorObjectOwned;

use crate::types::Message;

/// Gateway Methods for XPS
pub struct XpsMethods;

#[async_trait]
impl XpsServer for XpsMethods {
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned> {
        //TODO: Stub for sendMessage, ref: [discussion](https://github.com/xmtp/xps-gateway/discussions/11)
        log::debug!("xps_sendMessage called");
        todo!();
    }
}
