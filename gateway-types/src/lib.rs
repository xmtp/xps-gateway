//! Shared types between XPS Gateawy and client (libxmtp)
use ethers::types::U256;
use ethers::utils::format_units;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

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
