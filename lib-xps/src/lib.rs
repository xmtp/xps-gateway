pub mod rpc;
pub mod types;
mod util;

use anyhow::Result;
use ethers::{
    abi::Address,
    providers::{Provider, Ws},
};
use jsonrpsee::{server::Server, RpcModule};
use std::str::FromStr;
use xps_types::{CONVERSATION, DID_ETH_REGISTRY};

pub use crate::rpc::{XpsClient, XpsMethods, XpsServer};
use crate::types::GatewayContext;

/// Entrypoint for the xps Gateway
pub async fn run<P: AsRef<str>>(host: String, port: u16, provider: P) -> Result<()> {
    crate::util::init_logging();

    let server_addr = format!("{}:{}", host, port);

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build(server_addr).await?;
    let addr = server.local_addr()?;

    let registry_contract = Address::from_str(DID_ETH_REGISTRY)?;
    let conversation_contract = Address::from_str(CONVERSATION)?;
    let provider = Provider::<Ws>::connect(provider.as_ref()).await.unwrap();

    let context = GatewayContext::new(registry_contract, conversation_contract, provider).await?;
    let mut methods = RpcModule::new(());
    methods.merge(rpc::XpsMethods::new(&context).into_rpc())?;
    let methods = build_rpc_api(methods);

    let handle = server.start(methods);

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}

// create an endpoint that lists all the methods available on the server, at the
// endpoint `/rpc_methods`
fn build_rpc_api<M: Send + Sync + 'static>(mut rpc_api: RpcModule<M>) -> RpcModule<M> {
    let mut available_methods = rpc_api.method_names().collect::<Vec<_>>();
    // The "rpc_methods" is defined below and we want it to be part of the reported methods.
    available_methods.push("rpc_methods");
    available_methods.sort();

    rpc_api
        .register_method("rpc_methods", move |_, _| {
            serde_json::json!({
                "methods": available_methods,
            })
        })
        .expect("infallible, all other methods have their own address space");

    rpc_api
}
