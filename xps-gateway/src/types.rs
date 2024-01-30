use std::sync::Arc;

use anyhow::Error;
use ethers::{
    middleware::SignerMiddleware, providers::Middleware, signers::LocalWallet, types::Address,
};
use lib_didethresolver::did_registry::DIDRegistry;
use rand::{rngs::StdRng, SeedableRng};

pub type GatewaySigner<P> = SignerMiddleware<P, LocalWallet>;

pub struct GatewayContext<P: Middleware> {
    pub registry: DIDRegistry<GatewaySigner<P>>,
    pub signer: Arc<GatewaySigner<P>>,
    pub wallet: LocalWallet,
}

impl<P: Middleware + 'static> GatewayContext<P> {
    pub async fn new(registry: Address, provider: P) -> Result<Self, Error> {
        let wallet = LocalWallet::new(&mut StdRng::from_entropy());
        let signer =
            Arc::new(SignerMiddleware::new_with_provider_chain(provider, wallet.clone()).await?);
        let registry = DIDRegistry::new(registry, signer.clone());
        Ok(Self {
            registry,
            signer,
            wallet,
        })
    }
}

#[cfg(test)]
mod tests {
    use ethers::{prelude::MockProvider, providers::Provider, types::U64};
    use std::str::FromStr;

    use super::*;

    impl GatewayContext<Provider<MockProvider>> {
        pub async fn mocked() -> (Self, MockProvider) {
            let (mut provider, mock) = Provider::mocked();
            provider.set_interval(std::time::Duration::from_millis(1));
            mock.push(U64::from(2)).unwrap();

            let gateway = GatewayContext::new(
                Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
                provider,
            )
            .await
            .unwrap();

            (gateway, mock)
        }
    }

    #[tokio::test]
    async fn test_gateway_constructor() {
        let (provider, mock) = Provider::mocked();
        mock.push(U64::from(2)).unwrap();

        let gateway = GatewayContext::new(
            Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            provider,
        )
        .await
        .unwrap();

        assert!(gateway.registry.address().is_zero());
        assert!(gateway.signer.is_signer().await);
    }
}
