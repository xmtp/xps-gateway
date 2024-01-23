use std::sync::Arc;

use anyhow::Error;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Provider, Ws},
    signers::LocalWallet,
    types::Address,
};
use lib_didethresolver::did_registry::DIDRegistry;
use rand::{rngs::StdRng, SeedableRng};

pub type GatewaySigner = SignerMiddleware<Provider<Ws>, LocalWallet>;

pub struct GatewayContext {
    pub registry: DIDRegistry<GatewaySigner>,
    pub signer: Arc<GatewaySigner>,
}

impl GatewayContext {
    pub async fn new<Endpoint: AsRef<str>>(
        registry: Address,
        provider_endpoint: Endpoint,
    ) -> Result<Self, Error> {
        let wallet = LocalWallet::new(&mut StdRng::from_entropy());
        let provider = Provider::<Ws>::connect(provider_endpoint).await?;
        let signer =
            Arc::new(SignerMiddleware::new_with_provider_chain(provider, wallet.clone()).await?);
        let registry = DIDRegistry::new(registry, signer.clone());
        Ok(Self { registry, signer })
    }
}
