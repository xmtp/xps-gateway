mod integration_util;

use anyhow::Error;

use ethers::{signers::LocalWallet, types::Bytes, utils::keccak256};
use lib_didethresolver::{
    did_registry::RegistrySignerExt,
    types::{DidUrl, KeyEncoding, XmtpAttribute, XmtpKeyPurpose},
};
use messaging::ConversationSignerExt;
use xps_gateway::rpc::XpsClient;

use ethers::middleware::Middleware;
use ethers::types::{Address, U256, U64};
use gateway_types::Message;

use integration_util::*;

#[tokio::test]
async fn test_say_hello() -> Result<(), Error> {
    with_xps_client(None, |client, _, _, _| async move {
        let result = client.status().await?;
        assert_eq!(result, "OK");
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_send_message() -> Result<(), Error> {
    with_xps_client(None, |client, context, _resolver, anvil| async move {
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
            conversation_id: conversation_id,
            payload: payload,
            identity: me.address(),
            signature: signature,
        };

        let pre_nonce = context.conversation.nonce(me.address()).call().await?;
        assert!(pre_nonce == U256::zero());

        let result = client.send_message(message).await;
        assert!(result.is_ok());

        let post_nonce = context.conversation.nonce(me.address()).call().await?;
        assert!(post_nonce == pre_nonce + 1);
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_wallet_address() -> Result<(), Error> {
    with_xps_client(None, |client, _, _, _| async move {
        let result = client.wallet_address().await?;
        assert_ne!(result, Address::zero());
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_grant_installation() -> Result<(), Error> {
    with_xps_client(None, |client, context, resolver, anvil| async move {
        let wallet: LocalWallet = anvil.keys()[3].clone().into();
        let me = get_user(&anvil, 3).await;
        let name = *b"xmtp/installation/hex           ";
        let value = b"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71";

        let attribute = XmtpAttribute {
            purpose: XmtpKeyPurpose::Installation,
            encoding: KeyEncoding::Hex,
        };

        let block_number = context.signer.get_block_number().await.unwrap();
        let validity_period: U64 = U64::from(60 * 60 * 24 * 365 / 5); // number of round in one year, assuming 5-second round.
        let validity = block_number + validity_period;

        let signature = wallet
            .sign_attribute(
                &context.registry,
                name,
                value.to_vec(),
                U256::from(validity.as_u64()),
            )
            .await?;

        client
            .grant_installation(
                format!("0x{}", hex::encode(me.address())),
                attribute,
                value.to_vec(),
                signature,
            )
            .await?;

        let doc = resolver
            .resolve_did(me.address(), None)
            .await
            .unwrap()
            .document;

        assert_eq!(doc.verification_method.len(), 2);
        assert_eq!(
            doc.verification_method[0].id,
            DidUrl::parse(format!(
                "did:ethr:0x{}#controller",
                hex::encode(me.address())
            ))
            .unwrap()
        );
        assert_eq!(
            doc.verification_method[1].id,
            DidUrl::parse(format!(
                "did:ethr:0x{}?meta=installation#xmtp-0",
                hex::encode(me.address())
            ))
            .unwrap()
        );
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_revoke_installation() -> Result<(), Error> {
    with_xps_client(None, |client, context, resolver, anvil| async move {
        let wallet: LocalWallet = anvil.keys()[3].clone().into();
        let me = get_user(&anvil, 3).await;
        let name = *b"xmtp/installation/hex           ";
        let value = b"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71";
        let validity = U256::from(604_800);
        let signature = wallet
            .sign_attribute(&context.registry, name, value.to_vec(), validity)
            .await?;

        let attr = context.registry.set_attribute_signed(
            me.address(),
            signature.v.try_into().unwrap(),
            signature.r.into(),
            signature.s.into(),
            name,
            value.into(),
            validity,
        );
        attr.send().await?.await?;

        let doc = resolver
            .resolve_did(me.address(), None)
            .await
            .unwrap()
            .document;
        assert_eq!(
            doc.verification_method[1].id,
            DidUrl::parse(format!(
                "did:ethr:0x{}?meta=installation#xmtp-0",
                hex::encode(me.address())
            ))
            .unwrap()
        );

        let signature = wallet
            .sign_revoke_attribute(&context.registry, name, value.to_vec())
            .await?;

        let attribute = XmtpAttribute {
            purpose: XmtpKeyPurpose::Installation,
            encoding: KeyEncoding::Hex,
        };

        client
            .revoke_installation(
                format!("0x{}", hex::encode(me.address())),
                attribute,
                value.to_vec(),
                signature,
            )
            .await?;

        let doc = resolver
            .resolve_did(me.address(), None)
            .await
            .unwrap()
            .document;

        log::debug!("{}", serde_json::to_string_pretty(&doc).unwrap());

        assert_eq!(
            doc.verification_method[0].id,
            DidUrl::parse(format!(
                "did:ethr:0x{}#controller",
                hex::encode(me.address())
            ))
            .unwrap()
        );
        assert_eq!(doc.verification_method.len(), 1);

        Ok(())
    })
    .await
}
