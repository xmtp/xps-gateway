pub mod error;
#[cfg(test)]
mod test;

use std::str::FromStr;

use error::ContactOperationError;
use ethers::{
    providers::Middleware,
    types::{Address, Bytes, Signature, H160, U256},
};
use lib_didethresolver::{did_registry::DIDRegistry, types::XmtpAttribute, Resolver};
use xps_types::{GrantInstallationResult, IdentityResult, InstallationId, Status};

pub struct ContactOperations<Middleware> {
    registry: DIDRegistry<Middleware>,
    resolver: Resolver<Middleware>,
}

impl<M> ContactOperations<M>
where
    M: Middleware + 'static,
{
    /// Creates a new ContactOperations instance
    pub fn new(registry: DIDRegistry<M>) -> Self {
        let resolver = registry.clone().into();
        Self { registry, resolver }
    }

    /// Internal function to resolve a DID to an ethereum address
    fn resolve_did_address(&self, did: String) -> Result<H160, ContactOperationError<M>> {
        // for now, we will just assume the DID is a valid ethereum wallet address
        // TODO: Parse or resolve the actual DID

        let address = Address::from_slice(Bytes::from_str(did.as_ref())?.to_vec().as_slice());
        Ok(address)
    }

    /// Fetches key packages for a given DID using [`Resolver::resolve_did`]
    pub async fn get_identity_updates(
        &self,
        did: String,
        start_time_ns: i64,
    ) -> Result<IdentityResult, ContactOperationError<M>> {
        let address = Address::from_str(&did)?;

        let resolution = self
            .resolver
            .resolve_did(address, None)
            .await
            .map_err(|e| ContactOperationError::ResolutionError(e, did))?;

        if resolution.metadata.deactivated {
            return Err(ContactOperationError::DIDDeactivated);
        }

        let document = resolution.document;

        let installations = document
            .verification_method
            .into_iter()
            .filter(|method| {
                method
                    .id
                    .fragment()
                    .map(|f| f.starts_with("xmtp-"))
                    .unwrap_or(false)
                    && method
                        .id
                        .contains_query("meta".into(), "installation".into())
            })
            .filter_map(|method| {
                Some(InstallationId {
                    id: method.verification_properties?.try_into().ok()?,
                    timestamp_ns: method
                        .id
                        .get_query_value("timestamp")?
                        .parse::<u64>()
                        .ok()?,
                })
            })
            .collect::<Vec<InstallationId>>();

        Ok(IdentityResult {
            status: Status::Success,
            message: "Identities retrieved".to_string(),
            installations: installations
                .into_iter()
                // the only way an i64 cannot fit into a u64 if it's negative, so we can safely set
                // to 0
                .filter(|i| i.timestamp_ns > start_time_ns.try_into().unwrap_or(0))
                .collect(),
        })
    }

    /// Grants an XMTP installation via the did:ethr registry.
    pub async fn grant_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
        validity: U256,
    ) -> Result<GrantInstallationResult, ContactOperationError<M>> {
        let address = self.resolve_did_address(did)?;
        let attribute: [u8; 32] = name.into();
        log::debug!(
            "setting attribute {:#?}",
            String::from_utf8_lossy(&attribute)
        );

        let transaction_receipt = self
            .registry
            .set_attribute_signed(
                address,
                signature.v.try_into()?,
                signature.r.into(),
                signature.s.into(),
                attribute,
                value.into(),
                validity,
            )
            .send()
            .await?
            .await?;

        if let Some(ref receipt) = transaction_receipt {
            log::debug!(
                "Gas Used by transaction {}, Gas used in block {}, effective_price {}",
                receipt.gas_used.unwrap_or(0.into()),
                receipt.cumulative_gas_used,
                receipt.effective_gas_price.unwrap_or(0.into())
            );
        }

        Ok(GrantInstallationResult {
            status: Status::Success,
            message: "Installation request complete.".to_string(),
            transaction: transaction_receipt.map(|r| r.transaction_hash),
        })
    }

    /// Revokes an XMTP installation via the did:ethr registry.
    pub async fn revoke_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<(), ContactOperationError<M>> {
        let address = self.resolve_did_address(did)?;
        let attribute: [u8; 32] = name.into();
        log::debug!(
            "Revoking attribute {:#?}",
            String::from_utf8_lossy(&attribute)
        );

        let transaction_receipt = self
            .registry
            .revoke_attribute_signed(
                address,
                signature.v.try_into()?,
                signature.r.into(),
                signature.s.into(),
                attribute,
                value.into(),
            )
            .send()
            .await?
            .await?;

        if let Some(ref receipt) = transaction_receipt {
            log::debug!(
                "Gas Used by transaction {}, Gas used in block {}, effective_price {}",
                receipt.gas_used.unwrap_or(0.into()),
                receipt.cumulative_gas_used,
                receipt.effective_gas_price.unwrap_or(0.into())
            );
        }

        Ok(())
    }

    /// get the nonce for a given address from [`DIDRegistry`]
    pub async fn nonce(&self, did: String) -> Result<U256, ContactOperationError<M>> {
        let address = self.resolve_did_address(did)?;
        let nonce = self.registry.nonce(address).await?;
        Ok(nonce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{
        abi::AbiEncode,
        providers::{MockProvider, Provider},
    };
    use lib_didethresolver::did_registry::NonceReturn;

    impl ContactOperations<Provider<MockProvider>> {
        pub fn mocked() -> (Self, MockProvider) {
            let (mock_provider, mock) = Provider::mocked();
            let registry = DIDRegistry::new(H160::zero(), mock_provider.into());

            (ContactOperations::new(registry), mock)
        }
    }

    #[test]
    fn test_resolve_address_from_hexstr() {
        let addr = "0x0000000000000000000000000000000000000000";
        let (ops, _) = ContactOperations::mocked();
        assert_eq!(
            ops.resolve_did_address(addr.to_string()).unwrap(),
            H160::zero()
        );

        let addr = "0000000000000000000000000000000000000000";
        assert_eq!(
            ops.resolve_did_address(addr.to_string()).unwrap(),
            H160::zero()
        );
    }

    #[tokio::test]
    async fn test_nonce() {
        let (ops, mock) = ContactOperations::mocked();

        mock.push::<String, String>(NonceReturn(U256::from(212)).encode_hex())
            .unwrap();

        let nonce = ops
            .nonce("0x1111111111111111111111111111111111111111".to_string())
            .await
            .unwrap();

        assert_eq!(nonce, U256::from(212));
    }
}
