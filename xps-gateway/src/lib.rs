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
        abi::AbiEncode,
        prelude::{Transaction, TransactionReceipt},
        providers::MockProvider,
        types::{Block, FeeHistory, TxHash, U256, U64},
    };
    use serde::Serialize;

    pub trait MockProviderExt {
        /// Set the response for a call to a contract
        /// This must be called for each transaction that a function might send.
        ///
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
        /// mock.set_transaction_response(TransactionReceipt::default());
        /// let pending = provider.send_transaction(tx, None).await.unwrap().await.unwrap();
        /// ```
        fn set_transaction_response(&mut self, response: TransactionReceipt);

        /// Set the response for a transaction to a Contract
        ///
        /// # Example
        /// ```
        ///  
        /// let (context, mut mock) = GatewayContext::mocked().await;
        /// let methods = XpsMethods::new(&context);
        /// let attr = XmtpAttribute {
        ///     encoding: KeyEncoding::Hex,
        ///     purpose: XmtpKeyPurpose::Installation,
        /// };
        /// let value = vec![0x01, 0x02, 0x03];
        /// mock.set_contract_response(Default::default());
        /// let res = methods
        ///     .revoke_installation(
        ///         Address::default().to_string(),
        ///         attr,
        ///         value,
        ///         Signature {
        ///             r: [0x01; 32].into(),
        ///             s: [0x02; 32].into(),
        ///             v: 0x01,
        ///         },
        ///     )
        ///     .await;
        /// ```
        fn set_contract_response(&mut self, response: TransactionReceipt);

        /// Set the response for a call to a contract
        ///
        /// # Example
        ///
        /// ```
        /// let (context, mut mock) = GatewayContext::mocked().await;
        /// let registry = DIDRegistry::new(Address::default(), context.signer.clone());
        /// mock.set_call_response(ChangedReturn(U256::zero()));
        /// registry.changed(Address::default()).call().await.unwrap();
        ///
        /// ```
        ///
        fn set_call_response<T: Serialize + Send + Sync + AbiEncode>(&mut self, response: T);
    }

    impl MockProviderExt for MockProvider {
        fn set_transaction_response(&mut self, response: TransactionReceipt) {
            self.push(response).unwrap();
            self.push(Transaction {
                block_number: Some(1.into()),
                ..Transaction::default()
            })
            .unwrap(); // eth_getTransaction
            self.push(TxHash::default()).unwrap(); // eth_sendTransaction
            self.push(U64::from(0)).unwrap(); // eth_estimateGas
            self.push(U64::from(0)).unwrap(); // eth_GasPrice
        }

        fn set_contract_response(&mut self, response: TransactionReceipt) {
            self.push(response).unwrap();
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

        fn set_call_response<T: Serialize + Send + Sync + AbiEncode>(&mut self, response: T) {
            self.push::<String, &String>(&response.encode_hex())
                .unwrap();
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
