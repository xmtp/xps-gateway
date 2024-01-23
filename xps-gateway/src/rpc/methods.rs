//! Interface Implementations for XPS JSON-RPC

use super::api::*;
use jsonrpsee::types::error::ErrorCode;

use async_trait::async_trait;
use ethers::prelude::*;
use jsonrpsee::types::ErrorObjectOwned;
use rand::{rngs::StdRng, SeedableRng};

use crate::types::Message;

/// Gateway Methods for XPS
pub struct XpsMethods {
    pub wallet: LocalWallet,
}

impl XpsMethods {
    /// Create a new instance of the XpsMethods struct
    pub fn new() -> Self {
        Self {
            wallet: LocalWallet::new(&mut StdRng::from_entropy()),
        }
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

    async fn wallet_address(&self) -> Result<Address, ErrorObjectOwned> {
        Ok(self.wallet.address())
    }
}
