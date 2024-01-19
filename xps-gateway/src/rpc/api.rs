//! Trait Interface Definitions for XPS JSON-RPC

use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};

use gateway_types::{Message, Signature, XmtpAttributeType};

/// XPS JSON-RPC Interface Methods
#[rpc(server, client, namespace = "xps")]
pub trait Xps {
    // Placeholder for send_message, see [the discussion](https://github.com/xmtp/xps-gateway/discussions/11)
    #[method(name = "sendMessage")]
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned>;

    /// removes the contact bundle for the XMTP device installation. Request must be made to a
    /// valid DID with an XMTP profile.
    ///
    /// # Arguments
    ///
    /// * `did` - the DID of the XMTP device installation
    /// * `name` - the name of the contact bundle
    /// * `value` - the value of the contact bundle
    /// * `signature` - the signature of the contact bundle
    #[method(name = "revokeInstallation")]
    async fn revoke_installation(
        &self,
        did: String,
        name: XmtpAttributeType,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<(), ErrorObjectOwned>;
}
