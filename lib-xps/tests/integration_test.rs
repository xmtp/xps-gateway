mod integration_util;

use std::str::FromStr;

use anyhow::Error;

use ethers::providers::Middleware;
use ethers::types::{Address, Bytes, TransactionRequest, U256};
use ethers::utils::keccak256;
use ethers::{signers::LocalWallet, signers::Signer};
use integration_util::*;
use jsonrpsee::core::ClientError;
use lib_didethresolver::{
    did_registry::RegistrySignerExt,
    types::{DidUrl, KeyEncoding, XmtpAttribute, XmtpKeyPurpose, NULL_ADDRESS},
};
use lib_xps::rpc::{XpsClient, DEFAULT_ATTRIBUTE_VALIDITY};
use messaging::ConversationSignerExt;
use xps_types::{Message, Status, Unit};

#[tokio::test]
async fn test_say_hello() -> Result<(), Error> {
    with_xps_client(None, None, |client, _, _, _| async move {
        let result = client.status().await?;
        assert_eq!(result, "OK");
        Ok(())
    })
    .await
}

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

#[tokio::test]
async fn test_wallet_address() -> Result<(), Error> {
    with_xps_client(None, None, |client, _, _, _| async move {
        let result = client.wallet_address().await?;
        assert_ne!(result, Address::zero());
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_grant_revoke() -> Result<(), Error> {
    with_xps_client(None, None, |client, context, resolver, anvil| async move {
        for (key_index, key) in anvil.keys().iter().enumerate() {
            let wallet: LocalWallet = key.clone().into();
            let me = get_user(&anvil, key_index).await;
            let name = *b"xmtp/installation/hex           ";
            let value = b"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71";

            let attribute = XmtpAttribute {
                purpose: XmtpKeyPurpose::Installation,
                encoding: KeyEncoding::Hex,
            };
            let signature = wallet
                .sign_attribute(
                    &context.registry,
                    name,
                    value.to_vec(),
                    U256::from(DEFAULT_ATTRIBUTE_VALIDITY),
                )
                .await?;

            client
                .grant_installation(
                    format!("0x{}", hex::encode(me.address())),
                    attribute.clone(),
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

            let signature = wallet
                .sign_revoke_attribute(&context.registry, name, value.to_vec())
                .await?;

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
        }
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_grant_installation() -> Result<(), Error> {
    with_xps_client(None, None, |client, context, resolver, anvil| async move {
        let keys = anvil.keys();
        let wallet: LocalWallet = keys[3].clone().into();
        let me = get_user(&anvil, 3).await;
        let name = *b"xmtp/installation/hex           ";
        let value = b"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71";

        let attribute = XmtpAttribute {
            purpose: XmtpKeyPurpose::Installation,
            encoding: KeyEncoding::Hex,
        };

        let signature = wallet
            .sign_attribute(
                &context.registry,
                name,
                value.to_vec(),
                U256::from(DEFAULT_ATTRIBUTE_VALIDITY),
            )
            .await?;

        client
            .grant_installation(
                format!("0x{}", hex::encode(me.address())),
                attribute.clone(),
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

        // now, try to grant again, and ensure it fails as expected:
        // (due to bad signature)
        match client
            .grant_installation(
                format!("0x{}", hex::encode(me.address())),
                attribute.clone(),
                value.to_vec(),
                signature,
            )
            .await
        {
            Err(jsonrpsee::core::client::error::Error::Call(e)) => assert_eq!(e.code(), -31999),
            _ => panic!("grant_installation call was expected to fail on the second invocation"),
        };

        // calculate the signature again.
        let signature = wallet
            .sign_attribute(
                &context.registry,
                name,
                value.to_vec(),
                U256::from(DEFAULT_ATTRIBUTE_VALIDITY),
            )
            .await?;

        // and invoke again.
        client
            .grant_installation(
                format!("0x{}", hex::encode(me.address())),
                attribute.clone(),
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
                "did:ethr:0x{}?meta=installation#xmtp-1",
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
    with_xps_client(None, None, |client, context, resolver, anvil| async move {
        let me: LocalWallet = anvil.keys()[3].clone().into();
        let name = *b"xmtp/installation/hex           ";
        let value = b"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71";

        set_attribute(name, value.to_vec(), &me, &context.registry).await?;

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

        let signature = me
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

#[tokio::test]
async fn test_balance() -> Result<(), Error> {
    with_xps_client(
        None,
        Some(0.into()),
        |client, context, _resolver, _anvil| async move {
            // by default, we have no balance. verify that.
            let mut balance = client.balance().await?;
            assert_eq!(balance.balance, U256::from(0));
            assert_eq!(balance.unit, Unit::Eth);

            // fund the wallet account.
            let accounts = context.signer.get_accounts().await?;
            let from = accounts[1];
            let tx = TransactionRequest::new()
                .to(client.wallet_address().await?)
                .value(5_000_000_000_000_000_000_000_u128)
                .from(from);
            context.signer.send_transaction(tx, None).await?.await?;

            // check to see if the balance gets updated.
            balance = client.balance().await?;
            assert_eq!(
                balance.balance,
                U256::from(5_000_000_000_000_000_000_000_u128)
            );
            assert_eq!(balance.unit, Unit::Eth);

            Ok(())
        },
    )
    .await
}

#[tokio::test]
async fn test_fetch_key_packages() -> Result<(), Error> {
    with_xps_client(None, None, |client, context, _, anvil| async move {
        let me: LocalWallet = anvil.keys()[3].clone().into();
        let name = *b"xmtp/installation/hex           ";
        let value = b"000000000000000000000000000000000000000000000000000000000000000000";
        set_attribute(name, value.to_vec(), &me, &context.registry).await?;

        let value = b"111111111111111111111111111111111111111111111111111111111111111111";
        set_attribute(name, value.to_vec(), &me, &context.registry).await?;

        let res = client
            .fetch_key_packages(format!("0x{}", hex::encode(me.address())))
            .await?;

        assert_eq!(res.status, Status::Success);
        assert_eq!(&res.message, "Key packages retrieved");
        assert_eq!(
            res.installation,
            vec![
                hex::decode(b"000000000000000000000000000000000000000000000000000000000000000000")
                    .unwrap(),
                hex::decode(b"111111111111111111111111111111111111111111111111111111111111111111")
                    .unwrap()
            ]
        );
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_fetch_key_packages_revoke() -> Result<(), Error> {
    with_xps_client(None, None, |client, context, _, anvil| async move {
        let me: LocalWallet = anvil.keys()[3].clone().into();
        let name = *b"xmtp/installation/hex           ";
        let value = b"000000000000000000000000000000000000000000000000000000000000000000";
        set_attribute(name, value.to_vec(), &me, &context.registry).await?;

        let value = b"111111111111111111111111111111111111111111111111111111111111111111";
        set_attribute(name, value.to_vec(), &me, &context.registry).await?;

        client
            .revoke_installation(
                format!("0x{}", hex::encode(me.address())),
                XmtpAttribute {
                    purpose: XmtpKeyPurpose::Installation,
                    encoding: KeyEncoding::Hex,
                },
                value.to_vec(),
                me.sign_revoke_attribute(&context.registry, name, value.to_vec())
                    .await?,
            )
            .await?;

        let res = client
            .fetch_key_packages(format!("0x{}", hex::encode(me.address())))
            .await?;

        assert_eq!(res.status, Status::Success);
        assert_eq!(&res.message, "Key packages retrieved");
        assert_eq!(
            res.installation,
            vec![hex::decode(
                b"000000000000000000000000000000000000000000000000000000000000000000"
            )
            .unwrap()]
        );

        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_fetch_key_packages_client() -> Result<(), Error> {
    with_xps_client(None, None, |client, context, _, anvil| async move {
        let me: LocalWallet = anvil.keys()[3].clone().into();
        let attribute = XmtpAttribute {
            purpose: XmtpKeyPurpose::Installation,
            encoding: KeyEncoding::Hex,
        };
        let value = b"000000000000000000000000000000000000000000000000000000000000000000";

        client
            .grant_installation(
                format!("0x{}", hex::encode(me.address())),
                attribute.clone(),
                value.to_vec(),
                me.sign_attribute(
                    &context.registry,
                    attribute.into(),
                    value.to_vec(),
                    U256::from(DEFAULT_ATTRIBUTE_VALIDITY),
                )
                .await?,
            )
            .await?;
        let res = client
            .fetch_key_packages(format!("0x{}", hex::encode(me.address())))
            .await?;

        assert_eq!(res.status, Status::Success);
        assert_eq!(&res.message, "Key packages retrieved");
        assert_eq!(
            res.installation,
            vec![hex::decode(
                b"000000000000000000000000000000000000000000000000000000000000000000"
            )
            .unwrap()]
        );

        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_did_deactivation() -> Result<(), Error> {
    with_xps_client(None, None, |client, context, _, anvil| async move {
        let me: LocalWallet = anvil.keys()[3].clone().into();

        let new_owner = Address::from_str(NULL_ADDRESS).unwrap();
        let signature = me.sign_owner(&context.registry, new_owner).await.unwrap();
        let _ = context
            .registry
            .change_owner_signed(
                me.address(),
                signature.v.try_into().unwrap(),
                signature.r.into(),
                signature.s.into(),
                new_owner,
            )
            .send()
            .await?
            .await?;

        let res = client
            .fetch_key_packages(format!("0x{}", hex::encode(me.address())))
            .await
            .unwrap_err();

        assert!(matches!(res, ClientError::Call(_)));
        match res {
            ClientError::Call(err) => {
                assert_eq!(err.code(), -31999);
                assert_eq!(
                    err.message(),
                    "The DID has been deactivated, and no longer valid"
                );
            }
            _ => panic!("Expected a client error. this should never match"),
        }
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_nonce() -> Result<(), Error> {
    with_xps_client(None, None, |client, _, _, anvil| async move {
        let me: LocalWallet = anvil.keys()[3].clone().into();

        let nonce = client.nonce(hex::encode(me.address())).await?;
        assert_eq!(U256::from(0), nonce);

        Ok(())
    })
    .await
}
