//! Trait Interface Definitions for XPS JSON-RPC

use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};

use crate::types::Message;

/// XPS JSON-RPC Interface Methods
#[rpc(server, client, namespace = "xps")]
pub trait Xps {
    // Placeholder for send_message, see [the discussion](https://github.com/xmtp/xps-gateway/discussions/11)
    #[method(name = "sendMessage")]
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned>;
}
