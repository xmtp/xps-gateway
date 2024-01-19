//! Contact Operations

use std::str::FromStr;

use ethers::{providers::Middleware, types::Address};
use gateway_types::{Signature, XmtpAttributeType};
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
        let address = Address::from_str(&did)?;
        self.registry
            .revoke_attribute_signed(
                address,
                signature.v,
                signature.r,
                signature.s,
                Attribute::from(name).into(),
                value.into(),
            )
            .send()
            .await?
            .await?;
        Ok(())
    }
}
