use anyhow::Error;
use jsonrpsee::{
    server::Server,
    ws_client::{WsClient, WsClientBuilder},
};

use ethers::{
    abi::Address,
    core::utils::Anvil,
    middleware::Middleware,
    middleware::SignerMiddleware,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer as _},
    utils::AnvilInstance,
};
use futures::future::FutureExt;
use lib_didethresolver::{did_registry::DIDRegistry, Resolver};
use std::{future::Future, sync::Arc, time::Duration};
use tokio::time::timeout as timeout_tokio;

use xps_gateway::{
    rpc::XpsClient,
    types::{GatewayContext, GatewaySigner},
    XpsMethods, XpsServer, SERVER_HOST,
};

const TEST_TIMEOUT: Duration = Duration::from_secs(10);

#[cfg(test)]
mod it {
    use ethers::{abi::Bytes, types::U256};
    use gateway_types::{Message, XmtpAttributeType};

    use super::*;

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
    async fn test_fail_send_message() -> Result<(), Error> {
        with_xps_client(None, |client, _, _, _| async move {
            let message = Message {
                conversation_id: b"abcdefg".iter().map(|c| *c as u8).collect(),
                payload: b"Hello World".iter().map(|c| *c as u8).collect(),
                v: vec![],
                r: vec![],
                s: vec![],
            };
            let result = client.send_message(message).await;
            assert!(result.is_err());
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_revoke_installation() -> Result<(), Error> {
        with_xps_client(None, |client, context, resolver, _| async move {
            let me = context.signer.address();
            let name = *b"did/pub/xmtp/ed25519/inst/hex   ";
            let value = b"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71";
            let validity = U256::from(604_800);
            let tx = context
                .registry
                .set_attribute(me, name, value.into(), validity)
                .tx;
            let signature = context.signer.sign_transaction(&tx, me).await.unwrap();
            let attr = context.registry.set_attribute_signed(
                me,
                signature.v.try_into().unwrap(),
                signature.r.try_into().unwrap(),
                signature.s.try_into().unwrap(),
                name,
                value.into(),
                validity,
            );
            attr.send().await?.await?;

            let tx = context.registry.revoke_attribute(me, name, value.into()).tx;
            let signature = context.signer.sign_transaction(&tx, me).await.unwrap();
            client
                .revoke_installation(
                    format!("0x{}", hex::encode(me)),
                    XmtpAttributeType::InstallationKey,
                    value.to_vec(),
                    signature,
                )
                .await?;

            let doc = resolver.resolve_did(me, None).await.unwrap().document;
            println!("{}", serde_json::to_string_pretty(&doc).unwrap());
            Ok(())
        })
        .await
    }
}

async fn with_xps_client<F, R, T>(timeout: Option<Duration>, f: F) -> Result<T, Error>
where
    F: FnOnce(WsClient, GatewayContext, Resolver<Arc<GatewaySigner>>, &AnvilInstance) -> R
        + 'static
        + Send,
    R: Future<Output = Result<T, Error>> + FutureExt + Send + 'static,
{
    let anvil = Anvil::new().args(vec!["--base-fee", "100"]).spawn();
    log::debug!("Anvil spawned at {}", anvil.ws_endpoint());
    let registry_address = deploy_to_anvil(&anvil).await;
    log::debug!("Contract deployed at {}", registry_address);

    let server = Server::builder().build(SERVER_HOST).await.unwrap();
    let addr = server.local_addr().unwrap();
    let context = GatewayContext::new(anvil.ws_endpoint()).await?;
    let resolver = Resolver::new(context.signer.clone(), registry_address)
        .await
        .unwrap();

    let handle = server.start(XpsMethods::new(&context).into_rpc());
    let client = WsClientBuilder::default()
        .build(&format!("ws://{addr}"))
        .await
        .unwrap();

    let result = timeout_tokio(
        timeout.unwrap_or(TEST_TIMEOUT),
        f(client, context, resolver, &anvil),
    )
    .await;

    handle.stop().unwrap();
    handle.stopped().await;

    match result {
        Ok(v) => v,
        Err(_) => panic!("Test timed out"),
    }
}

async fn deploy_to_anvil(anvil: &AnvilInstance) -> Address {
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let client = client(&anvil, wallet).await;

    let registry = DIDRegistry::deploy(client.clone(), ())
        .unwrap()
        .gas_price(100)
        .send()
        .await
        .unwrap();

    registry.address()
}

async fn client(
    anvil: &AnvilInstance,
    wallet: LocalWallet,
) -> Arc<SignerMiddleware<Provider<Ws>, LocalWallet>> {
    let provider = Provider::<Ws>::connect(anvil.ws_endpoint())
        .await
        .unwrap()
        .interval(std::time::Duration::from_millis(10u64));
    Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(anvil.chain_id()),
    ))
}
