pub mod rpc;
pub mod types;
mod util;

pub use crate::rpc::{XpsMethods, XpsServer};
use anyhow::Result;
use jsonrpsee::server::Server;

pub const SERVER_HOST: &str = "127.0.0.1:0";

/// Entrypoint for the xps Gateway
pub async fn run() -> Result<()> {
    crate::util::init_logging();

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build(SERVER_HOST).await?;
    let addr = server.local_addr()?;
    let xps_methods = XpsMethods {
        registry: registry::XpsRegistry {},
    };
    let handle = server.start(xps_methods.into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}
