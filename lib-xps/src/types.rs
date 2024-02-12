use std::sync::Arc;

use anyhow::Error;
use ethers::{
    middleware::SignerMiddleware, providers::Middleware, signers::LocalWallet, types::Address,
};
use lib_didethresolver::did_registry::DIDRegistry;
use messaging::Conversation;
use rand::{rngs::StdRng, SeedableRng};

pub type GatewaySigner<P> = SignerMiddleware<P, LocalWallet>;

pub struct GatewayContext<P: Middleware> {
    pub registry: DIDRegistry<GatewaySigner<P>>,
    pub conversation: Conversation<GatewaySigner<P>>,
    pub signer: Arc<GatewaySigner<P>>,
}

impl<P: Middleware + 'static> GatewayContext<P> {
    pub async fn new(registry: Address, conversation: Address, provider: P) -> Result<Self, Error> {
        let wallet = LocalWallet::new(&mut StdRng::from_entropy());
        let signer =
            Arc::new(SignerMiddleware::new_with_provider_chain(provider, wallet.clone()).await?);
        let registry = DIDRegistry::new(registry, signer.clone());
        let conversation = Conversation::new(conversation, signer.clone());
        Ok(Self {
            registry,
            conversation,
            signer,
        })
    }
}

#[cfg(test)]
mod tests {
    use ethers::{providers::Provider, types::U64};
    use std::str::FromStr;

    use super::*;

    #[tokio::test]
    async fn test_gateway_constructor() {
        let (provider, mock) = Provider::mocked();
        mock.push(U64::from(2)).unwrap();

        let gateway = GatewayContext::new(
            Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            provider,
        )
        .await
        .unwrap();

        assert!(gateway.registry.address().is_zero());
        assert!(gateway.conversation.address().is_zero());
        assert!(gateway.signer.is_signer().await);
    }
}
