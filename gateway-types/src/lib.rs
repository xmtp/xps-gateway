//! Shared types between XPS Gateawy and client (libxmtp)
pub mod error;

use ethers::types::{Address, Bytes as EthersBytes, Signature};
use std::fmt;

use serde::{Deserialize, Serialize};

/// Address of the did:ethr Registry on Sepolia
pub const DID_ETH_REGISTRY: &str = "0xd1D374DDE031075157fDb64536eF5cC13Ae75000";
// Address of the Converstion on Sepolia
pub const CONVERSATION: &str = "0x15aE865d0645816d8EEAB0b7496fdd24227d1801";

/// A message sent to a conversation
#[derive(Serialize, Deserialize)]
pub struct Message {
    // Unique identifier for a conversation
    #[serde(rename = "conversationId")]
    pub conversation_id: [u8; 32],
    /// message content in bytes
    pub payload: EthersBytes,
    // Sender's identity
    pub identity: Address,
    // Signature by sender
    pub signature: Signature,
}

pub type Bytes = Vec<u8>;

/// GrantInstallationResult represents the result of a grant installation operation in the DID registry.
///
/// This struct encapsulates the outcome of an attempt to grant an installation,
/// providing details about the operation's status, a descriptive message, and the
/// transaction identifier associated with the blockchain transaction.
///
/// # Fields
/// * `status` - One of [`Status::Success`] or [`Status::Failed`], indicating the outcome of the
/// operation.
/// * `message` - A `String` providing more detailed information about the operation. This
///   can be a success message, error description, or any other relevant information.
/// * `transaction` - A `String` representing the unique identifier of the transaction on the
///   blockchain. This can be used to track the transaction in a blockchain explorer.
///
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GrantInstallationResult {
    pub status: Status,
    pub message: String,
    pub transaction: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SendMessageResult {
    pub status: Status,
    pub message: String,
    pub transaction: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct KeyPackageResult {
    /// Status of the operation
    pub status: Status,
    /// A message relating to the operation
    pub message: String,
    /// A list of key packages
    pub installation: Vec<Bytes>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Success,
    Failed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Success => write!(f, "success"),
            Status::Failed => write!(f, "failed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", Status::Success), "success");
        assert_eq!(format!("{}", Status::Failed), "failed");
    }
}
