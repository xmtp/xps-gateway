extern crate ethers;
extern crate tokio;

pub mod registry;
pub mod types;

use ethers::providers::{Http, Provider};
use std::sync::Arc;
pub struct XpsRegistry {
    provider: Arc<Provider<Http>>,
    registry_contract_address: String,
}
