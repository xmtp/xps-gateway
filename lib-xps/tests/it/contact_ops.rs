use std::str::FromStr;

use anyhow::Error;

use crate::integration_util::*;
use ethers::types::{Address, U256};
use ethers::{signers::LocalWallet, signers::Signer};
use jsonrpsee::core::ClientError;
use lib_didethresolver::{
    did_registry::RegistrySignerExt,
    types::{DidUrl, KeyEncoding, XmtpAttribute, XmtpKeyPurpose, NULL_ADDRESS},
};
use lib_xps::rpc::{XpsClient, DEFAULT_ATTRIBUTE_VALIDITY};
use xps_types::Status;

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
