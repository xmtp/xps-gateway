use anyhow::Error;
use jsonrpsee::{
    server::Server,
    ws_client::{WsClient, WsClientBuilder},
};

use ethers::{
    abi::Address,
    core::{types::TransactionRequest, utils::Anvil},
    middleware::Middleware,
    middleware::SignerMiddleware,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer as _},
    types::U256,
    utils::AnvilInstance,
};
use futures::future::FutureExt;
use lib_didethresolver::{
    did_registry::{DIDRegistry, RegistrySignerExt},
    Resolver,
};
use messaging::Conversation;
use std::{
    future::Future,
    sync::{Arc, Once},
    time::Duration,
};
use tokio::time::timeout as timeout_tokio;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

use lib_xps::{
    types::{GatewayContext, GatewaySigner},
    XpsMethods, XpsServer,
};

const TEST_TIMEOUT: Duration = Duration::from_secs(20);
pub const SERVER_HOST: &str = "127.0.0.1";

/// Run a test with an XPS client
/// If `with_balance` is provided, the gateway signer will be funded with the given balance
/// If `with_balance` is not provided, the gateway signer will be funded with 5_000_000_000_000_000_000_000 wei
pub async fn with_xps_client<F, R, T>(
    timeout: Option<Duration>,
    with_balance: Option<U256>,
    f: F,
) -> Result<T, Error>
where
    F: FnOnce(
            WsClient,
            GatewayContext<Provider<Ws>>,
            Resolver<Arc<GatewaySigner<Provider<Ws>>>>,
            Arc<AnvilInstance>,
        ) -> R
        + 'static
        + Send,
    R: Future<Output = Result<T, Error>> + FutureExt + Send + 'static,
{
    init_test_logging();
    let anvil = Anvil::new().args(vec!["--base-fee", "100"]).spawn();
    log::debug!("Anvil spawned at {}", anvil.ws_endpoint());
    let (registry_address, conversation_address) = deploy_to_anvil(&anvil).await;
    log::debug!("Registry contract deployed at {}", registry_address);
    log::debug!("Conversation contract deployed at {}", conversation_address);
    let provider = Provider::<Ws>::connect(anvil.ws_endpoint())
        .await
        .unwrap()
        .interval(std::time::Duration::from_millis(10u64));

    let context = GatewayContext::new(registry_address, conversation_address, provider).await?;

    // transfer balance to the gateway signer so that we may be able to send & pay for transactions to anvil
    let accounts = context.signer.get_accounts().await?;
    let from = accounts[0];
    let tx = TransactionRequest::new()
        .to(context.signer.address())
        .value(with_balance.unwrap_or(U256::from(5_000_000_000_000_000_000_000_u128)))
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

    let server = Server::builder()
        .build(SERVER_HOST.to_string() + ":0")
        .await
        .unwrap();
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

async fn deploy_to_anvil(anvil: &AnvilInstance) -> (Address, Address) {
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let client = client(anvil, wallet).await;

    let registry = DIDRegistry::deploy(client.clone(), ())
        .unwrap()
        .gas_price(100)
        .send()
        .await
        .unwrap();

    let conversation = Conversation::deploy(client.clone(), ())
        .unwrap()
        .gas_price(100)
        .send()
        .await
        .unwrap();

    (registry.address(), conversation.address())
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

pub async fn get_user(
    anvil: &AnvilInstance,
    index: usize,
) -> Arc<SignerMiddleware<Provider<Ws>, LocalWallet>> {
    let wallet: LocalWallet = anvil.keys()[index].clone().into();
    client(anvil, wallet).await
}

static INIT: Once = Once::new();

fn init_test_logging() {
    INIT.call_once(|| {
        let fmt = fmt::layer().compact();
        Registry::default()
            .with(EnvFilter::from_default_env())
            .with(fmt)
            .init()
    })
}

pub async fn set_attribute(
    name: [u8; 32],
    value: Vec<u8>,
    wallet: &LocalWallet,
    registry: &DIDRegistry<GatewaySigner<Provider<Ws>>>,
) -> Result<(), Error> {
    let validity = U256::from(604_800);
    let signature = wallet
        .sign_attribute(registry, name, value.to_vec(), validity)
        .await?;

    let attr = registry.set_attribute_signed(
        wallet.address(),
        signature.v.try_into().unwrap(),
        signature.r.into(),
        signature.s.into(),
        name,
        value.into(),
        validity,
    );
    attr.send().await?.await?;
    Ok(())
}
