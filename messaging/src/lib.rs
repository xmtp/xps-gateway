pub mod error;

use error::MessagingOperationError;
use ethers::contract::abigen;
use ethers::core::types::Bytes;
use ethers::providers::Middleware;

abigen!(
    Conversation,
    "./abi/Conversation.json",
    derives(serde::Serialize, serde::Deserialize)
);

pub struct MessagingOperations<Middleware> {
    messaging: Conversation<Middleware>,
}

impl<M> MessagingOperations<M>
where
    M: Middleware + 'static,
{
    /// Creates a new MessagingOperations instance
    pub fn new(messaging: Conversation<M>) -> Self {
        Self { messaging }
    }

    pub async fn send_message(
        &self,
        conversation_id: [u8; 32],
        payload: Bytes,
    ) -> Result<(), MessagingOperationError<M>> {
        self.messaging
            .send_message(conversation_id, payload)
            .send()
            .await?
            .await?;
        Ok(())
    }
}
