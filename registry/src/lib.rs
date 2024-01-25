pub mod error;

use std::str::FromStr;

use ethers::{core::types::Signature, providers::Middleware, types::Address};
use lib_didethresolver::{
    did_registry::DIDRegistry,
    types::{Attribute, XmtpAttribute},
};

use error::ContactOperationError;

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

    pub async fn revoke_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<(), ContactOperationError<M>> {
        // for now, we will just assume the DID is a valid ethereum wallet address
        // TODO: Parse or resolve the actual DID
        let address = Address::from_str(&did)?;
        let attribute: [u8; 32] = Attribute::from(name).into();
        log::debug!(
            "Revoking attribute {:#?}",
            String::from_utf8_lossy(&attribute)
        );
        let res = self
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

        Ok(())
    }
}
