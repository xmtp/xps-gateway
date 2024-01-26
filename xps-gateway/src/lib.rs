pub mod rpc;
pub mod types;
mod util;

use anyhow::Result;
use jsonrpsee::server::Server;

pub use crate::rpc::{XpsMethods, XpsServer};

/// Entrypoint for the xps Gateway
pub async fn run(host: String, port: u16) -> Result<()> {
    crate::util::init_logging();

    let server_addr = format!("{}:{}", host, port);

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build(server_addr).await?;
    let addr = server.local_addr()?;
    let handle = server.start(XpsMethods::new().into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use ethers::{
        providers::{MockProvider, Provider},
        types::{Address, U64, H256, TxHash},
        prelude::{PendingTransaction, MockResponse, TransactionReceipt, Transaction}
    };
    use serde_json::Value;
    use std::borrow::Borrow;
    use serde::Serialize;
/*
    use crate::types::GatewayContext;

    pub async fn create_mock_context() -> (GatewayContext<Provider<MockProvider>>, MockProvider) {
        let (provider, mock) = Provider::mocked();
        mock.push(U64::from(2)).unwrap();

        let gateway = GatewayContext::new(
            Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            provider,
        )
        .await
        .unwrap();

        (gateway, mock)
    }
    */

    fn setup_mock_tx(mock: &mut MockProvider) {
        mock.push(TransactionReceipt::default()).unwrap(); // eth_getTransactionReceipt
        mock.push(Transaction { block_number: Some(1.into()), ..Transaction::default()}).unwrap(); // eth_getTransaction
        mock.push(TxHash::default()).unwrap(); // eth_sendTransaction
        mock.push(U64::from(0)).unwrap(); // eth_estimateGas
        mock.push(U64::from(0)).unwrap(); // eth_GasPrice
    }

    pub trait MockProviderExt {
        /// Set the response for a call to a contract
        /// This must be called for each transaction that a function might send.
        ///
        /// # Example
        /// ```
        /// use ethers::{
        ///     providers::{MockProvider, Provider},
        ///     prelude::TransactionRequest,
        ///     types::{Address, Transaction}
        /// };
        /// use std::time::Duration;
        ///
        /// let (mut provider, mut mock) = Provider::mocked();
        /// provider.set_interval(Duration::from_millis(1));
        ///
        /// let to = Address::from_str("0x7e575682a8e450e33eb0493f9972821ae333cd7f").unwrap();
        /// let from = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
        /// let tx = TransactionRequest::new().to(to).value(1000).from(from);
        /// mock.set_transaction_response(None::<()>);
        /// let pending = provider.send_transaction(tx, None).await.unwrap().await.unwrap(); 
        /// ```
        fn set_transaction_response<T: Serialize + Send + Sync, R: Borrow<T>>(&mut self, response: Option<R>);
    }

    impl MockProviderExt for MockProvider {
        fn set_transaction_response<T: Serialize + Send + Sync, R: Borrow<T>>(&mut self, response: Option<R>) {
            if let Some(r) = response {
                self.push(r).unwrap();
            }
            setup_mock_tx(self);
        }
    }
}
