pub mod error;

use std::str::FromStr;

use error::ContactOperationError;
use ethers::types::{H160, U256};
use ethers::{core::types::Signature, providers::Middleware, types::Address};
use gateway_types::{GrantInstallationResult, KeyPackageResult, Status};
use lib_didethresolver::types::VerificationMethodProperties;
use lib_didethresolver::Resolver;
use lib_didethresolver::{did_registry::DIDRegistry, types::XmtpAttribute};

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
        let address = Address::from_str(&did)?;
        Ok(address)
    }

    /// Fetches key packages for a given DID using [`Resolver::resolve_did`]
    pub async fn fetch_key_packages(
        &self,
        did: String,
    ) -> Result<KeyPackageResult, ContactOperationError<M>> {
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

        let properties = document
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
            .filter_map(|method| method.verification_properties)
            .collect::<Vec<VerificationMethodProperties>>();

        Ok(KeyPackageResult {
            status: Status::Success,
            message: "Key packages retrieved".to_string(),
            installation: properties
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
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
        Ok(GrantInstallationResult {
            status: Status::Success,
            message: "Installation request complete.".to_string(),
            transaction: transaction_receipt.unwrap().transaction_hash.to_string(),
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

        self.registry
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

        Ok(())
    }
}
