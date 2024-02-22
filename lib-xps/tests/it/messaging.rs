use anyhow::Error;

use ethers::utils::keccak256;
use ethers::types::{Bytes, U256};
use ethers::signers::LocalWallet;
use crate::integration_util::*;
use lib_xps::rpc::XpsClient;
use messaging::ConversationSignerExt;
use xps_types::{Message, Status};



#[tokio::test]
async fn test_send_message() -> Result<(), Error> {
    with_xps_client(None, None, |client, context, _resolver, anvil| async move {
        let wallet: LocalWallet = anvil.keys()[3].clone().into();
        let me = get_user(&anvil, 3).await;

        let conversation_id = keccak256(b"conversation_id");
        let payload = Bytes::from_static(b"payload");

        let signature = wallet
            .sign_xmtp_message(
                &context.conversation,
                conversation_id,
                payload.clone(),
                me.address(),
            )
            .await?;

        let message = Message {
            conversation_id,
            payload,
            identity: me.address(),
            signature,
        };

        let pre_nonce = context.conversation.nonce(me.address()).call().await?;
        assert!(pre_nonce == U256::zero());

        let result = client.send_message(message).await;
        assert!(result.is_ok());
        assert!(result.unwrap().status == Status::Success);

        // post-nonce should be same as pre-nonce + 1
        let post_nonce = context.conversation.nonce(me.address()).call().await?;
        assert!(post_nonce == pre_nonce + 1);
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_send_message_fail() -> Result<(), Error> {
    with_xps_client(None, None, |client, context, _resolver, anvil| async move {
        let wallet: LocalWallet = anvil.keys()[3].clone().into();
        let me = get_user(&anvil, 3).await;

        let conversation_id = keccak256(b"conversation_id");
        let payload = Bytes::from_static(b"payload");

        let signature = wallet
            .sign_xmtp_message(
                &context.conversation,
                keccak256(b"unmatched_conversation_id"),
                payload.clone(),
                me.address(),
            )
            .await?;

        let message = Message {
            conversation_id,
            payload,
            identity: me.address(),
            signature,
        };

        let pre_nonce = context.conversation.nonce(me.address()).call().await?;
        assert!(pre_nonce == U256::zero());

        let result = client.send_message(message).await;
        assert!(result.is_err());
        println!("{:?}", result.err());

        // post-nonce should be same as pre-nonce
        let post_nonce = context.conversation.nonce(me.address()).call().await?;
        assert!(post_nonce == pre_nonce);
        Ok(())
    })
    .await
}

