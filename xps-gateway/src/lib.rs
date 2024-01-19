pub mod rpc;
pub mod types;
mod util;

use anyhow::Result;
use jsonrpsee::server::Server;
use serde::Deserialize;

pub use crate::rpc::{XpsMethods, XpsServer};

pub const SERVER_HOST: &str = "127.0.0.1:0";
pub const DEFAULT_WALLET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Deserialize)]
struct XpsGatewayOptions {
    /// wallet address
    #[serde(default = "default_wallet_address")]
    wallet_address: String,
}

fn default_wallet_address() -> String {
    DEFAULT_WALLET_ADDRESS.to_string()
}

/// Entrypoint for the xps Gateway
pub async fn run() -> Result<()> {
    crate::util::init_logging();
    match dotenvy::dotenv() {
        Ok(path) => {
            // .env file successfully loaded.
            log::debug!("Env file {} was loaded successfully", path.display());
        }
        Err(err) => {
            // Error handling for the case where dotenv() fails
            log::info!("Unable to load env file(s) : {err}");
        }
    }
    let opts: XpsGatewayOptions = envy::from_env()?;

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build(SERVER_HOST).await?;
    let addr = server.local_addr()?;
    let handle = server.start(XpsMethods::new(opts.wallet_address.as_str()).into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}
