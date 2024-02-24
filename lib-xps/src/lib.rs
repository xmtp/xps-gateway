pub mod rpc;
pub mod types;
#[cfg(test)]
mod util;

use anyhow::Result;
use ethers::{abi::Address, providers::Middleware};
use jsonrpsee::{server::Server, RpcModule};
use std::str::FromStr;
use xps_types::{CONVERSATION, DID_ETH_REGISTRY};

pub use crate::rpc::{XpsClient, XpsMethods, XpsServer};
use crate::types::GatewayContext;

/// Entrypoint for the xps Gateway
pub async fn run<P>(host: String, port: u16, provider: P) -> Result<()>
where
    P: Middleware + 'static,
{
    let server_addr = format!("{}:{}", host, port);
    let server = Server::builder().build(server_addr).await?;
    let addr = server.local_addr()?;

    let registry_contract = Address::from_str(DID_ETH_REGISTRY)?;
    let conversation_contract = Address::from_str(CONVERSATION)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{prelude::Provider, types::U64};
    use jsonrpsee::{core::client::ClientT, ws_client::WsClientBuilder};

    #[tokio::test]
    async fn test_run() -> Result<()> {
        let (provider, mock) = Provider::mocked();
        // chainID
        mock.push(U64::from(0x1)).unwrap();
        let port = 43594;
        let handle = tokio::spawn(async move {
            match run("127.0.0.1".to_string(), 43594, provider).await {
                Err(e) => log::error!("Error running server: {e}"),
                Ok(_) => log::info!("Server Stopped"),
            }
        });

        // give the server some time to start
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let client = WsClientBuilder::default()
            .build(&format!("ws://127.0.0.1:{port}"))
            .await?;

        #[derive(Debug, serde::Deserialize)]
        struct Methods {
            methods: Vec<String>,
        }

        let methods = client
            .request::<Methods, Vec<()>>("rpc_methods", vec![])
            .await?;

        assert_eq!(
            methods.methods,
            vec![
                "rpc_methods",
                "xps_balance",
                "xps_fetchKeyPackages",
                "xps_grantInstallation",
                "xps_nonce",
                "xps_revokeInstallation",
                "xps_sendMessage",
                "xps_status",
                "xps_walletAddress",
            ]
        );

        handle.abort();
        Ok(())
    }

    #[test]
    fn test_build_api() {
        let methods = RpcModule::new(());
        let methods = build_rpc_api(methods);
        let methods: Vec<String> = methods.method_names().map(String::from).collect();
        assert_eq!(methods, vec!["rpc_methods",]);
    }
}
