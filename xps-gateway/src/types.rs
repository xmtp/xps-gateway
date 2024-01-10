use serde::{Deserialize, Serialize};

/// A message sent to a conversation
#[derive(Serialize, Deserialize)]
pub struct Message {
    // Unique identifier for a conversation
    #[serde(rename = "groupId")]
    group_id: Vec<u8>,
    /// message content in bytes
    payload: Vec<u8>,
    /// Signature of V
    v: Vec<u8>,
    /// Signature of R
    r: Vec<u8>,
    /// Signature of S
    s: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Signature {
    /// Signature of V
    #[serde(rename = "V")]
    v: i64,
    /// Signature of R
    #[serde(rename = "R")]
    r: Vec<u8>,
    /// Signature of S
    #[serde(rename = "S")]
    s: Vec<u8>,
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
