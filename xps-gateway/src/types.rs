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
