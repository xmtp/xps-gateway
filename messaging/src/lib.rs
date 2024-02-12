pub mod error;

use error::MessagingOperationError;
use ethers::{
    abi::{Address, Token},
    contract::abigen,
    core::abi::encode_packed,
    core::types::{Bytes, Signature},
    providers::Middleware,
    signers::LocalWallet,
    types::H256,
    utils::keccak256,
};
use xps_types::{error::ExtSignerError, Message, SendMessageResult, Status};

abigen!(
    Conversation,
    "./abi/Conversation.json",
    derives(serde::Serialize, serde::Deserialize)
);

pub struct MessagingOperations<Middleware> {
    contract: Conversation<Middleware>,
}

impl<M> MessagingOperations<M>
where
    M: Middleware + 'static,
{
    /// Creates a new MessagingOperations instance
    pub fn new(contract: Conversation<M>) -> Self {
        Self { contract }
    }

    pub async fn send_message(
        &self,
        m: Message,
    ) -> Result<SendMessageResult, MessagingOperationError<M>> {
        let transaction_receipt = self
            .contract
            .send_message_signed(
                m.conversation_id,
                m.payload,
                m.identity,
                m.signature.v.try_into()?,
                m.signature.r.into(),
                m.signature.s.into(),
            )
            .send()
            .await?
            .await?;
        Ok(SendMessageResult {
            status: Status::Success,
            message: "Message sent.".to_string(),
            transaction: transaction_receipt.unwrap().transaction_hash.to_string(),
        })
    }
}

/// Signer for data that is externally signed to be processed by the Conversation Contract.
#[async_trait::async_trait]
pub trait ConversationSignerExt {
    /// Sign hash of the data for [`Conversation::send_message_signed`]
    async fn sign_xmtp_message<M: Middleware>(
        &self,
        conversation: &Conversation<M>,
        conversation_id: [u8; 32],
        payload: Bytes,
        identity: Address,
    ) -> Result<Signature, ExtSignerError<M>>;
}

#[async_trait::async_trait]
impl ConversationSignerExt for LocalWallet {
    async fn sign_xmtp_message<M: Middleware>(
        &self,
        conversation: &Conversation<M>,
        conversation_id: [u8; 32],
        payload: Bytes,
        identity: Address,
    ) -> Result<Signature, ExtSignerError<M>> {
        let nonce = conversation.nonce(identity).call().await?;
        let mut nonce_bytes = [0; 32];
        nonce.to_big_endian(&mut nonce_bytes);
        let tokens = vec![
            Token::FixedBytes(vec![0x19]),
            Token::FixedBytes(vec![0x0]),
            Token::FixedBytes(conversation_id[0..32].to_vec()),
            Token::Bytes(payload.to_vec()),
            Token::Address(identity),
            Token::Bytes(nonce_bytes[0..32].to_vec()),
        ];

        let encoded = encode_packed(tokens.as_slice())?;
        let digest = H256(keccak256(encoded));
        let signature = self.sign_hash(digest)?;
        Ok(signature)
    }
}
