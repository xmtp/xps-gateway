use crate::types::{GrantInstallationResult, Signature};
use ethers::contract::abigen;
use ethers::contract::Contract;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use std::sync::Arc;

abigen!(
    DIDRegistry,
    "./src/abi/DIDRegistry.json",
    derives(serde::Serialize, serde::Deserialize)
);

impl super::XpsRegistry {
    /// Constructs a new instance with a specified Ethereum provider and registry contract address.
    ///
    /// This function initializes the object with a given Ethereum provider and the address of
    /// a DID registry contract. The provider is used for interacting with the Ethereum blockchain,
    /// and the registry address specifies the location of the DID registry contract that this
    /// object will interact with.
    ///
    /// # Arguments
    /// * `provider` - A `Provider<Http>` instance used for blockchain interactions.
    /// * `registry_address` - A `String` representing the Ethereum address of the DID registry contract.
    ///
    /// # Returns
    /// Returns an instance of the object configured with the given provider and registry address.
    ///
    /// # Examples
    /// ```
    /// let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    /// let registry_address = "0x...".to_string();
    /// let obj = YourStruct::new(provider, registry_address);
    /// ```
    ///
    /// Ensure that the `registry_address` is a valid Ethereum address of the deployed DID registry contract.
    pub fn new(provider: Provider<Http>, registry_address: String) -> Self {
        Self {
            provider: Arc::new(provider),
            registry_contract_address: registry_address,
        }
    }

    /// Asynchronously grants an installation on the DID registry.
    ///
    /// This method allows you to associate a set of data (name and value) with a decentralized identifier (DID)
    /// and record this association on the blockchain. The operation is authenticated using a digital signature.
    ///
    /// # Arguments
    /// * `did` - A `String` representing the decentralized identifier to which the data will be linked.
    /// * `name` - A `String` representing the name attribute of the installation.
    /// * `value` - A `String` containing the data or value to be associated with the DID.
    /// * `signature` - A `Signature` struct representing the cryptographic signature authenticating the operation.
    ///
    /// # Returns
    /// Returns a `Result` that, on success, encapsulates a `GrantInstallationResult` containing details
    /// about the operation, including its status, a message, and the transaction information.
    /// On failure, it returns an `anyhow::Error`.
    ///
    /// # Errors
    /// This function may return an error if there are issues with the blockchain interaction,
    /// such as network problems, invalid arguments, or failed transaction submissions.
    ///
    /// # Examples
    /// ```
    /// // Assuming 'client' is an instance of the object containing this method
    /// let result = client.grant_installation(
    ///     "did:example:123".to_string(),
    ///     "attributeName".to_string(),
    ///     "attributeValue".to_string(),
    ///     signature, // A valid Signature struct
    /// ).await;
    /// ```
    ///
    /// Ensure that the provided `did`, `name`, `value`, and `signature` are valid and conform
    /// to the expected formats and standards for the operation.
    pub async fn grant_installation(
        &self,
        did: String,
        name: String,
        value: String,
        signature: Signature,
    ) -> Result<GrantInstallationResult, anyhow::Error> {
        // Setup contract instance
        let contract_address = self.registry_contract_address.parse::<Address>()?;
        let contract = Contract::new(
            contract_address,
            DIDREGISTRY_ABI.clone(),
            self.provider.clone(),
        );

        // Prepare the setAttributeSigned transaction data
        let tx_data = contract
            .method::<_, H256>(
                "setAttributeSigned",
                (
                    did.clone(), // identity
                    signature.v, // sigV
                    signature.r, // sigR
                    signature.s, // sigS
                    name,        // name
                    value,       // value
                    3600,        // validity
                ),
            )?
            .tx
            .as_legacy_ref()
            .unwrap()
            .data
            .clone()
            .expect("Failed to construct transaction data");

        // Send the transaction
        let tx = self
            .provider
            .send_transaction(
                TransactionRequest {
                    to: Some(ethers::types::NameOrAddress::Address(contract_address)),
                    data: Some(tx_data),
                    // Set other fields as needed (gas, gas_price, etc.)
                    ..Default::default()
                },
                None,
            )
            .await?;

        // Wait for transaction confirmation
        let receipt = tx.await?;

        // Handle result
        let result = GrantInstallationResult {
            status: "Success".to_string(),
            message: "Installation granted".to_string(),
            transaction: format!("{:?}", receipt.unwrap().transaction_hash),
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::XpsRegistry;
    use ethers::types::H256;
    use tokio;

    /// The address of the DID Registry contract on the Ethereum Sepolia Testnet
    pub const DID_ETH_REGISTRY: &str = "0xd1D374DDE031075157fDb64536eF5cC13Ae75000";
    pub(crate) const DEFAULT_PROVIDER: &str = "http://127.0.0.1:8545";

    #[tokio::test]
    async fn test_grant_installation() {
        let provider = Provider::<Http>::try_from(DEFAULT_PROVIDER).unwrap();

        let registry = XpsRegistry::new(provider, DID_ETH_REGISTRY.to_string()); // Modify as per your constructor
        let signature = Signature {
            v: 27, // Example values
            r: H256::zero().0.to_vec(),
            s: H256::zero().0.to_vec(),
        };

        let result = registry
            .grant_installation(
                "did:example:123".to_string(),
                "did:xmtp:v3:installkey".to_string(),
                "installation_key_value".to_string(),
                signature,
            )
            .await;

        assert!(result.is_ok());
        // Further assertions based on expected results
    }
}
