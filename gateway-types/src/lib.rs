use lib_didethresolver::types::{Attribute, KeyEncoding, KeyPurpose, KeyType, PublicKey};
use serde::{Deserialize, Serialize};

/// Address of the did:ethr Registry on Sepolia
pub const DID_ETH_REGISTRY: &str = "0xd1D374DDE031075157fDb64536eF5cC13Ae75000";

/// A message sent to a conversation
#[derive(Serialize, Deserialize)]
pub struct Message {
    // Unique identifier for a conversation
    #[serde(rename = "groupId")]
    group_id: Vec<u8>,
    /// message content in bytes
    payload: Vec<u8>,
    signature: Signature,
}

/// A Signature containing V, R S
#[derive(Serialize, Deserialize)]
pub struct Signature {
    /// Signature of V
    pub v: u8,
    /// Signature of R
    pub r: [u8; 32],
    /// Signature of S
    pub s: [u8; 32],
}

/// The XMTP-specific attribute type
#[derive(Serialize, Deserialize)]
pub enum XmtpAttributeType {
    InstallationKey,
}

impl From<XmtpAttributeType> for Attribute {
    fn from(attribute: XmtpAttributeType) -> Self {
        match attribute {
            XmtpAttributeType::InstallationKey => Attribute::PublicKey(PublicKey {
                key_type: KeyType::Ed25519VerificationKey2020,
                purpose: KeyPurpose::Xmtp,
                encoding: KeyEncoding::Hex,
            }),
        }
    }
}
