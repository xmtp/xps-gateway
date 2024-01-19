mod rpc;
mod types;
mod util;

use anyhow::Result;
use jsonrpsee::server::Server;

pub use crate::rpc::{XpsMethods, XpsServer};
use crate::types::GatewayContext;

/// Entrypoint for the xps Gateway
pub async fn run() -> Result<()> {
    crate::util::init_logging();

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build("127.0.0.1:0").await?;
    let addr = server.local_addr()?;

    let context = GatewayContext::new("wss://ethereum-sepolia.publicnode.com").await?;
    let xps_methods = rpc::XpsMethods::new(&context);
    let handle = server.start(xps_methods.into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}
