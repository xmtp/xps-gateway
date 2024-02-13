pub mod rpc;
pub mod types;
mod util;

use anyhow::Result;
use ethers::{
    abi::Address,
    providers::{Provider, Ws},
};
use jsonrpsee::server::Server;
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
    let xps_methods = rpc::XpsMethods::new(&context);
    let handle = server.start(xps_methods.into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}