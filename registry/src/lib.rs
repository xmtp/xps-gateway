pub mod error;

use std::str::FromStr;

use error::ContactOperationError;
use ethers::types::{Bytes, U256};
use ethers::{core::types::Signature, providers::Middleware, types::Address};
use lib_didethresolver::Resolver;
use lib_didethresolver::{did_registry::DIDRegistry, types::XmtpAttribute};
use openmls_rust_crypto::RustCrypto;
use xmtp_mls::verified_key_package::VerifiedKeyPackage;
use xps_types::{GrantInstallationResult, InstallationId, KeyPackageResult, Status};

pub struct ContactOperations<Middleware> {
    registry: DIDRegistry<Middleware>,
    resolver: Resolver<Middleware>,
}

struct ValidateKeyPackageResult {
    installation_id: Vec<u8>,
    account_address: String,
    #[allow(dead_code)]
    credential_identity_bytes: Vec<u8>,
    #[allow(dead_code)]
    expiration: u64,
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
    fn resolve_did_address<S: AsRef<str>>(
        &self,
        did: &S,
    ) -> Result<Address, ContactOperationError<M>> {
        // for now, we will just assume the DID is a valid ethereum wallet address
        // TODO: Parse or resolve the actual DID
        let address = Address::from_slice(Bytes::from_str(did.as_ref())?.to_vec().as_slice());
        Ok(address)
    }

    /// Fetches key packages for a given DID using [`Resolver::resolve_did`]
    pub async fn fetch_key_packages(
        &self,
        did: String,
        start_time_ns: i64,
    ) -> Result<KeyPackageResult, ContactOperationError<M>> {
        let address = self.resolve_did_address(&did)?;

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

        Ok(KeyPackageResult {
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
        kp_bytes: Vec<u8>,
        signature: Signature,
        validity: U256,
    ) -> Result<GrantInstallationResult, ContactOperationError<M>> {
        let _address = self.resolve_did_address(&did)?;
        let attribute: [u8; 32] = name.into();
        log::debug!(
            "setting attribute {:#?}",
            String::from_utf8_lossy(&attribute)
        );

        let validated_key_package = Self::validate_key_package(kp_bytes)?;
        log::debug!(
            "Installation Id {:?}",
            validated_key_package.installation_id
        );

        let transaction_receipt = self
            .registry
            .set_attribute_signed(
                Address::from_str(&validated_key_package.account_address)?,
                signature.v.try_into()?,
                signature.r.into(),
                signature.s.into(),
                attribute,
                validated_key_package.installation_id.into(),
                validity, // we can also possibly use the expiration on the keypackage
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

    fn validate_key_package(
        key_package_bytes: Vec<u8>,
    ) -> Result<ValidateKeyPackageResult, ContactOperationError<M>> {
        let rust_crypto = RustCrypto::default();
        let verified_key_package =
            VerifiedKeyPackage::from_bytes(&rust_crypto, key_package_bytes.as_slice())?;

        Ok(ValidateKeyPackageResult {
            installation_id: verified_key_package.installation_id(),
            account_address: verified_key_package.account_address,
            credential_identity_bytes: verified_key_package
                .inner
                .leaf_node()
                .credential()
                .identity()
                .to_vec(),
            expiration: verified_key_package.inner.life_time().not_after(),
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
        let address = self.resolve_did_address(&did)?;
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
}
