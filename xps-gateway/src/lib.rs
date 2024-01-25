pub mod rpc;
pub mod types;
mod util;

pub use crate::rpc::{XpsMethods, XpsServer};
use anyhow::Result;
use ethers::providers::{Http, Provider};
use jsonrpsee::server::Server;

pub const DID_ETH_REGISTRY: &str = "0xd1D374DDE031075157fDb64536eF5cC13Ae75000";
pub(crate) const DEFAULT_PROVIDER: &str = "http://127.0.0.1:8545";

/// Entrypoint for the xps Gateway
pub async fn run(host: String, port: u16) -> Result<()> {
    crate::util::init_logging();

    let server_addr = format!("{}:{}", host, port);

    let provider = Provider::<Http>::try_from(DEFAULT_PROVIDER).unwrap();

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build(server_addr).await?;
    let addr = server.local_addr()?;
    let xps_methods = XpsMethods::new(provider, DID_ETH_REGISTRY.to_string());
    let handle = server.start(xps_methods.into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}
