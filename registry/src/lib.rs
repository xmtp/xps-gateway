pub mod error;

use std::str::FromStr;

use error::ContactOperationError;
use ethers::types::{H160, U256};
use ethers::{core::types::Signature, providers::Middleware, types::Address};
use gateway_types::GrantInstallationResult;
use lib_didethresolver::{
    did_registry::DIDRegistry,
    types::{Attribute, XmtpAttribute},
};

pub struct ContactOperations<Middleware> {
    registry: DIDRegistry<Middleware>,
}

impl<M> ContactOperations<M>
where
    M: Middleware + 'static,
{
    /// Creates a new ContactOperations instance
    pub fn new(registry: DIDRegistry<M>) -> Self {
        Self { registry }
    }

    fn resolve_did_address(&self, did: String) -> Result<H160, ContactOperationError<M>> {
        // for now, we will just assume the DID is a valid ethereum wallet address
        // TODO: Parse or resolve the actual DID
        let address = Address::from_str(&did)?;
        Ok(address)
    }

    pub async fn grant_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
        validity: U256,
    ) -> Result<GrantInstallationResult, ContactOperationError<M>> {
        let address = self.resolve_did_address(did)?;
        let attribute: [u8; 32] = Attribute::from(name).into();
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
            status: "completed".to_string(),
            message: "Installation request complete.".to_string(),
            transaction: transaction_receipt.unwrap().transaction_hash.to_string(),
        })
    }

    pub async fn revoke_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<(), ContactOperationError<M>> {
        let address = self.resolve_did_address(did)?;
        let attribute: [u8; 32] = Attribute::from(name).into();
        log::debug!(
            "Revoking attribute {:#?}",
            String::from_utf8_lossy(&attribute)
        );

        let res = self.registry.revoke_attribute_signed(
            address,
            signature.v.try_into()?,
            signature.r.into(),
            signature.s.into(),
            attribute,
            value.into(),
        );

        let res = res.send().await;
        println!("{:?}", res);
        res?.await?;

        Ok(())
    }
}
