use crate::types::{GrantInstallationResult, Signature};

impl super::XpsRegistry {
    pub async fn grant_installation(
        &self,
        _did: String,
        _name: String,
        _value: String,
        _signature: Signature,
    ) -> Result<GrantInstallationResult, anyhow::Error> {
        // if all good, invoke the call to the registry.
        // stub -
        let result = GrantInstallationResult {
            status: "my-status".to_string(),
            message: "my message".to_string(),
            transaction: "my transaction".to_string(),
        };
        Ok(result)
    }
}
