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

#[cfg(test)]
mod tests {
    use ethers::types::{Block, Transaction, U64};
    use lib_didethresolver::types::{KeyEncoding, XmtpKeyPurpose};

    use super::*;

    #[tokio::test]
    async fn test_rpc_revoke_installation() {
        let (context, mock) = crate::test::create_mock_context().await;

        mock.push(U64::from(0)).unwrap(); // transactioncount
        mock.push(Block::<Transaction>::default()).unwrap(); // latest block

        let methods = XpsMethods::new(&context);

        let attr = XmtpAttribute {
            encoding: KeyEncoding::Hex,
            purpose: XmtpKeyPurpose::Installation,
        };
        let value = vec![0x01, 0x02, 0x03];
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
        if let Err(e) = res {
            println!("{:?}", e);
            println!("{}", e);
        }
    }
}
