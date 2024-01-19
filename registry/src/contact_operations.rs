//! Contact Operations

use std::str::FromStr;

use ethers::{core::types::Signature, providers::Middleware, types::Address};
use gateway_types::XmtpAttributeType;
use lib_didethresolver::{did_registry::DIDRegistry, types::Attribute};

use crate::error::ContactOperationError;

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
        name: XmtpAttributeType,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<(), ContactOperationError<M>> {
        // for now, we will just assume the DID is a valid ethereum wallet address
        // TODO: Parse or resolve the actual DID
        // TODO: Remove unwraps
        let address = Address::from_str(&did)?;
        self.registry
            .revoke_attribute_signed(
                address,
                signature.v.try_into().unwrap(),
                signature.r.try_into().unwrap(),
                signature.s.try_into().unwrap(),
                Attribute::from(name).into(),
                value.into(),
            )
            .send()
            .await?
            .await?;
        Ok(())
    }
}
