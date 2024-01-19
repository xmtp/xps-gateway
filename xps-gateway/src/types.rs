use serde::{Deserialize, Serialize};

// The types defined here are paret of the model for RPC calls. These are different from the underlying crates implementation data structure.
// underlying types, for instance, won't be serialized.

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

#[derive(Serialize, Deserialize)]
pub struct Signature {
    /// Signature of V
    #[serde(rename = "V")]
    pub v: i64,
    /// Signature of R
    #[serde(rename = "R")]
    pub r: Vec<u8>,
    /// Signature of S
    #[serde(rename = "S")]
    pub s: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GrantInstallationResult {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "tx")]
    pub transaction: String,
}
