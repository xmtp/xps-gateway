pub mod rpc;
pub mod types;
mod util;

use anyhow::Result;
use ethers::abi::Address;
use gateway_types::DID_ETH_REGISTRY;
use jsonrpsee::server::Server;
use std::str::FromStr;

pub use crate::rpc::{XpsMethods, XpsServer};
use crate::types::GatewayContext;

/// Entrypoint for the xps Gateway
pub async fn run(host: String, port: u16) -> Result<()> {
    crate::util::init_logging();

    let server_addr = format!("{}:{}", host, port);

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build(server_addr).await?;
    let addr = server.local_addr()?;

    let registry_contract = Address::from_str(DID_ETH_REGISTRY)?;
    let context =
        GatewayContext::new(registry_contract, "wss://ethereum-sepolia.publicnode.com").await?;
    let xps_methods = rpc::XpsMethods::new(&context);
    let handle = server.start(xps_methods.into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}
