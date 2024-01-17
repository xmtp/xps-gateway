//! Trait Interface Definitions for XPS JSON-RPC

use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};

use crate::types::{GrantInstallationResult, Message, Signature};

/// XPS JSON-RPC Interface Methods
#[rpc(server, client, namespace = "xps")]
pub trait Xps {
    // Placeholder for send_message, see [the discussion](https://github.com/xmtp/xps-gateway/discussions/11)
    #[method(name = "sendMessage")]
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned>;

    #[method(name = "grantInstallation")]
    async fn grant_installation(
        &self,
        did: String,
        name: String,
        value: String,
        signature: Signature,
    ) -> Result<GrantInstallationResult, ErrorObjectOwned>;
}
