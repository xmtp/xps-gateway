use anyhow::Error;
use jsonrpsee::{
    server::Server,
    ws_client::{WsClient, WsClientBuilder},
};

use ethers::{
    abi::AbiDecode,
    abi::Address,
    core::{types::TransactionRequest, utils::Anvil},
    middleware::Middleware,
    middleware::SignerMiddleware,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer as _},
    utils::AnvilInstance,
};
use futures::future::FutureExt;
use lib_didethresolver::{
    did_registry::{DIDRegistry, RegistrySignerExt},
    Resolver,
};
use std::{
    future::Future,
    sync::{Arc, Once},
    time::Duration,
};
use tokio::time::timeout as timeout_tokio;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

use xps_gateway::{
    rpc::XpsClient,
    types::{GatewayContext, GatewaySigner},
    XpsMethods, XpsServer, SERVER_HOST,
};

const TEST_TIMEOUT: Duration = Duration::from_secs(20);

#[cfg(test)]
mod it {
    use ethers::types::U256;
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
        with_xps_client(None, |client, context, resolver, anvil| async move {
            let wallet: LocalWallet = anvil.keys()[3].clone().into();
            let me = get_user(&anvil, 3).await;
            let name = *b"did/pub/ed25519/veriKey/hex     ";
            let value = b"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71";
            let validity = U256::from(604_800);
            let signature = wallet
                .sign_attribute(&context.registry, name, value.to_vec(), validity)
                .await?;

            log::debug!("Me: {}", hex::encode(me.address()));
            log::debug!("Wallet {}", hex::encode(wallet.address()));
            log::debug!("Registry: {}", hex::encode(context.registry.address()));
            log::debug!("Signer: {}", hex::encode(context.signer.address()));
            let attr = context.registry.set_attribute_signed(
                me.address(),
                signature.v.try_into().unwrap(),
                signature.r.try_into().unwrap(),
                signature.s.try_into().unwrap(),
                name,
                value.into(),
                validity,
            );
            let res = attr.send().await;
            if let Err(e) = res {
                let rev = e.decode_revert::<String>();
                // let rev_bytes = e.as_revert().map(|b| hex::encode(b)).unwrap();
                // let rev = BadSignature::decode_hex(rev_bytes);
                println!("{:?}", rev);
            } else {
                println!("IT WORKED???");
            }

            let doc = resolver
                .resolve_did(me.address(), None)
                .await
                .unwrap()
                .document;

            log::debug!("{}", serde_json::to_string_pretty(&doc).unwrap());
            /*
            client
                .revoke_installation(
                    format!("0x{}", hex::encode(me.address())),
                    XmtpAttributeType::InstallationKey,
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
            */
            Ok(())
        })
        .await
    }
}

async fn with_xps_client<F, R, T>(timeout: Option<Duration>, f: F) -> Result<T, Error>
where
    F: FnOnce(WsClient, GatewayContext, Resolver<Arc<GatewaySigner>>, Arc<AnvilInstance>) -> R
        + 'static
        + Send,
    R: Future<Output = Result<T, Error>> + FutureExt + Send + 'static,
{
    init_test_logging();
    let anvil = Anvil::new().args(vec!["--base-fee", "100"]).spawn();
    log::debug!("Anvil spawned at {}", anvil.ws_endpoint());
    let registry_address = deploy_to_anvil(&anvil).await;
    log::debug!("Contract deployed at {}", registry_address);

    let context = GatewayContext::new(registry_address, anvil.ws_endpoint()).await?;

    let accounts = context.signer.get_accounts().await?;
    let from = accounts[0];
    let tx = TransactionRequest::new()
        .to(context.signer.address())
        .value(5_000_000_000_000_000_000_000_u128)
        .from(from);
    context.signer.send_transaction(tx, None).await?.await?;
    let balance = context
        .signer
        .get_balance(context.signer.address(), None)
        .await?;
    log::debug!("Gateway Balance is {}", balance);

    let resolver = Resolver::new(context.signer.clone(), registry_address)
        .await
        .unwrap();

    let server = Server::builder().build(SERVER_HOST).await.unwrap();
    let addr = server.local_addr().unwrap();
    let handle = server.start(XpsMethods::new(&context).into_rpc());
    let client = WsClientBuilder::default()
        .build(&format!("ws://{addr}"))
        .await
        .unwrap();
    let anvil = Arc::new(anvil);
    let result = timeout_tokio(
        timeout.unwrap_or(TEST_TIMEOUT),
        f(client, context, resolver, anvil.clone()),
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

async fn get_user(
    anvil: &AnvilInstance,
    index: usize,
) -> Arc<SignerMiddleware<Provider<Ws>, LocalWallet>> {
    let wallet: LocalWallet = anvil.keys()[index].clone().into();
    client(&anvil, wallet).await
}

#[cfg(test)]
static INIT: Once = Once::new();

#[cfg(test)]
fn init_test_logging() {
    INIT.call_once(|| {
        let fmt = fmt::layer().compact();
        Registry::default()
            .with(EnvFilter::from_default_env())
            .with(fmt)
            .init()
    })
}
