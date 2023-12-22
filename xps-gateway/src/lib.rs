mod rpc;
mod types;

use anyhow::Result;
use jsonrpsee::server::Server;
use tracing_subscriber::{fmt, Registry, util::SubscriberInitExt, EnvFilter, layer::SubscriberExt};

pub use crate::rpc::{XpsMethods, XpsServer};

/// Entrypoint for the xps Gateway
pub async fn run() -> Result<()> {
    init_logging();
    
    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build("127.0.0.1:0").await?;
    let addr = server.local_addr()?;
    let handle = server.start(rpc::XpsMethods.into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}

/// Start [`tracing`] logging
pub(crate) fn init_logging() {
    let fmt = fmt::layer().compact();
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt)
        .init()
}
