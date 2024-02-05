//! Shared types between XPS Gateawy and client (libxmtp)

use serde::{Deserialize, Serialize};

/// Address of the did:ethr Registry on Sepolia
pub const DID_ETH_REGISTRY: &str = "0xd1D374DDE031075157fDb64536eF5cC13Ae75000";

/// A message sent to a conversation
#[derive(Serialize, Deserialize)]
pub struct Message {
    // Unique identifier for a conversation
    #[serde(rename = "conversationId")]
    pub conversation_id: Vec<u8>,
    /// message content in bytes
    pub payload: Vec<u8>,
    /// Signature of V
    pub v: Vec<u8>,
    /// Signature of R
    pub r: Vec<u8>,
    /// Signature of S
    pub s: Vec<u8>,
}

/// GrantInstallationResult represents the result of a grant installation operation in the DID registry.
///
/// This struct encapsulates the outcome of an attempt to grant an installation,
/// providing details about the operation's status, a descriptive message, and the
/// transaction identifier associated with the blockchain transaction.
///
/// # Fields
/// * `status` - A `String` indicating the outcome status of the operation. Typically, this
///   would be values like "Success" or "Failure".
/// * `message` - A `String` providing more detailed information about the operation. This
///   can be a success message, error description, or any other relevant information.
/// * `transaction` - A `String` representing the unique identifier of the transaction on the
///   blockchain. This can be used to track the transaction in a blockchain explorer.
///
#[derive(Serialize, Deserialize, Clone)]
pub struct GrantInstallationResult {
    pub status: String,
    pub message: String,
    pub transaction: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletBalance {
    // Unique identifier for a conversation
    #[serde(rename = "balance")]
    pub balance: String,
    /// message content in bytes
    #[serde(rename = "unit")]
    pub unit: String,
}
