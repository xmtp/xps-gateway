//! Shared types between XPS Gateawy and client (libxmtp)

pub mod error;

pub use ethers::types::Bytes;
use ethers::{
    types::{Address, Signature, H256, U256},
    utils::format_units,
};
use std::fmt;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Address of the did:ethr Registry on Sepolia
pub const DID_ETH_REGISTRY: &str = "0xd1D374DDE031075157fDb64536eF5cC13Ae75000";
// Address of the Conversation on Sepolia
pub const CONVERSATION: &str = "0x15aE865d0645816d8EEAB0b7496fdd24227d1801";

/// A message sent to a conversation
#[derive(Serialize, Deserialize)]
pub struct Message {
    // Unique identifier for a conversation
    #[serde(rename = "conversationId")]
    pub conversation_id: [u8; 32],
    /// message content in bytes
    pub payload: Bytes,
    // Sender's identity
    pub identity: Address,
    // Signature by sender
    pub signature: Signature,
}

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
    pub transaction: Option<H256>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    Eth,
    Other(String),
}

/// WalletBalance used as the return value for the balance rpc endpoint.
#[derive(Serialize, Deserialize, Clone)]
pub struct WalletBalance {
    /// The balance for the wallet
    #[serde(rename = "balance")]
    pub balance: U256,
    /// The unit used for the balance
    #[serde(rename = "unit")]
    pub unit: Unit,
}

impl Display for WalletBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.unit {
            Unit::Eth => {
                let ether_balance =
                    format_units(self.balance, 18) // 18 decimal places for Ether
                    .unwrap_or_else(|_| "failed to convert balance".to_string());
                write!(f, "{} ETH", ether_balance)
            }
            Unit::Other(unit_name) => write!(f, "{} {}", self.balance, unit_name),
        }
    }
}

// Assuming you have a Display implementation for Unit as well
impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Eth => write!(f, "ETH"),
            Unit::Other(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SendMessageResult {
    pub status: Status,
    pub message: String,
    pub transaction: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IdentityResult {
    /// Status of the operation
    pub status: Status,
    /// A message relating to the operation
    pub message: String,
    /// A list of key packages
    pub installations: Vec<InstallationId>,
}

/// A single InstallationID
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InstallationId {
    // installation id
    pub id: Vec<u8>,
    /// Timestamp in nanoseconds of the block which the operation took place
    pub timestamp_ns: u64,
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

    #[test]
    fn test_wallet_balance_display() {
        assert_eq!(
            format!(
                "{}",
                WalletBalance {
                    balance: U256::from(123456789),
                    unit: Unit::Eth
                }
            ),
            "0.000000000123456789 ETH"
        );
        assert_eq!(
            format!(
                "{}",
                WalletBalance {
                    balance: U256::from(987654321),
                    unit: Unit::Eth
                }
            ),
            "0.000000000987654321 ETH"
        );
        assert_eq!(
            format!(
                "{}",
                WalletBalance {
                    balance: U256::from(500),
                    unit: Unit::Other("BTC".to_string())
                }
            ),
            "500 BTC"
        );
    }

    #[test]
    fn test_unit_display() {
        assert_eq!(format!("{}", Unit::Eth), "ETH");
        assert_eq!(format!("{}", Unit::Other("ABC".to_string())), "ABC");
    }
}
