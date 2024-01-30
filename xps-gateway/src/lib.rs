pub mod rpc;
pub mod types;
mod util;

use anyhow::Result;
use ethers::{
    abi::Address,
    providers::{Provider, Ws},
};
use gateway_types::DID_ETH_REGISTRY;
use jsonrpsee::server::Server;
use std::str::FromStr;

pub use crate::{
    rpc::{XpsMethods, XpsServer},
    types::GatewayContext,
};

/// Entrypoint for the xps Gateway
pub async fn run(host: String, port: u16) -> Result<()> {
    crate::util::init_logging();

    let server_addr = format!("{}:{}", host, port);

    // a port of 0 allows the OS to choose an open port
    let server = Server::builder().build(server_addr).await?;
    let addr = server.local_addr()?;

    let registry_contract = Address::from_str(DID_ETH_REGISTRY)?;
    let provider = Provider::<Ws>::connect("wss://ethereum-sepolia.publicnode.com")
        .await
        .unwrap();

    let context = GatewayContext::new(registry_contract, provider).await?;
    let xps_methods = rpc::XpsMethods::new(&context);
    let handle = server.start(xps_methods.into_rpc());

    log::info!("Server Started at {addr}");
    handle.stopped().await;
    Ok(())
}

#[cfg(test)]
mod test {
    use ethers::{
        prelude::{Transaction, TransactionReceipt},
        providers::MockProvider,
        types::{Block, FeeHistory, TxHash, U256, U64},
    };
    use serde::Serialize;
    use std::borrow::Borrow;

    fn setup_mock_tx(mock: &mut MockProvider) {
        mock.push(TransactionReceipt::default()).unwrap(); // eth_getTransactionReceipt
        mock.push(Transaction {
            block_number: Some(1.into()),
            ..Transaction::default()
        })
        .unwrap(); // eth_getTransaction
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
        fn set_transaction_response<T: Serialize + Send + Sync, R: Borrow<T>>(
            &mut self,
            response: Option<R>,
        );

        /// Set the response for a transaction to a Contract
        fn set_contract_response<T: Serialize + Send + Sync, R: Borrow<T>>(
            &mut self,
            response: Option<R>,
        );

        /// Set the response for a call to a contract
        fn set_call_response<T: Serialize + Send + Sync, R: Borrow<T>>(
            &mut self,
            response: Option<R>,
        ) {
            todo!()
        }
    }

    impl MockProviderExt for MockProvider {
        fn set_transaction_response<T: Serialize + Send + Sync, R: Borrow<T>>(
            &mut self,
            response: Option<R>,
        ) {
            if let Some(r) = response {
                self.push(r).unwrap();
            }
            setup_mock_tx(self);
        }

        fn set_contract_response<T: Serialize + Send + Sync, R: Borrow<T>>(
            &mut self,
            response: Option<R>,
        ) {
            self.push(TransactionReceipt::default()).unwrap(); // eth_getTransactionReceipt
            self.push(Transaction {
                block_number: Some(1.into()),
                ..Transaction::default()
            })
            .unwrap(); // eth_getTransaction
            self.push(TxHash::default()).unwrap(); // eth_sendRawTransaction
            self.push(U64::from(0)).unwrap(); // estimateGas
            self.push(fee_history()).unwrap(); // eth_feeHistory
            self.push(Block {
                //eth_getBlockByNumber
                // base_fee_per_gas needs to be Some() for EIP-1559
                base_fee_per_gas: Some(U256::zero()),
                ..Block::<TxHash>::default()
            })
            .unwrap();
            self.push(U64::from(0)).unwrap(); // transactionCount
        }

        fn set_call_response<T: Serialize + Send + Sync, R: Borrow<T>>(
            &mut self,
            response: Option<R>,
        ) {
            todo!()
        }
    }

    // internal fn to return an empty fee history
    fn fee_history() -> FeeHistory {
        FeeHistory {
            base_fee_per_gas: vec![U256::zero()],
            gas_used_ratio: vec![0.0],
            oldest_block: U256::zero(),
            reward: Vec::new(),
        }
    }
}
