pub mod rpc;
pub mod types;
mod util;

pub use crate::rpc::{XpsMethods, XpsServer};
use anyhow::Result;
use jsonrpsee::server::Server;

/// Entrypoint for the xps Gateway
pub async fn run(host: String, port: u16) -> Result<()> {
    crate::util::init_logging();

    let server_addr = format!("{}:{}", host, port);

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build(server_addr).await?;
    let addr = server.local_addr()?;
    let xps_methods = XpsMethods {
        registry: registry::XpsRegistry {},
    };
    let handle = server.start(xps_methods.into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}
