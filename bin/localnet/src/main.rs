use anyhow::Error;
use ethers::{
    abi::Address,
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer as _},
    utils::AnvilInstance,
};
use lib_didethresolver::did_registry::DIDRegistry;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let anvil = deploy().await?;

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    drop(anvil);
    println!("Shutting down...");
    Ok(())
}

async fn deploy() -> Result<AnvilInstance, Error> {
    let anvil = Anvil::new()
        .port(8545_u16)
        .args(vec![
            "--base-fee",
            "35",
            "--gas-price",
            "50",
            "--disable-block-gas-limit",
        ])
        .spawn();
    let registry_address = deploy_to_anvil(&anvil).await?;
    println!(
        "Registry deployed at {}, at endpoint {}",
        hex::encode(registry_address),
        anvil.ws_endpoint()
    );

    println!("Chain ID: {}", anvil.chain_id());
    println!("Endpoint: {}", anvil.endpoint());
    println!("WS Endpoint: {}", anvil.ws_endpoint());

    println!("\n\n");
    println!("Private Keys -------------------------------------");
    for key in anvil.keys() {
        println!("0x{}", hex::encode(key.to_bytes()));
    }
    println!("\n\n");
    println!("Addresses -------------------------------------");
    for address in anvil.addresses() {
        println!("0x{}", hex::encode(address));
    }

    Ok(anvil)
}

async fn deploy_to_anvil(anvil: &AnvilInstance) -> Result<Address, Error> {
    println!("Deploying Registry to local anvil");

    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let client = client(anvil, wallet).await;

    let registry = DIDRegistry::deploy(client.clone(), ())
        .unwrap()
        .gas_price(100)
        .send()
        .await
        .unwrap();

    Ok(registry.address())
}

async fn client(
    anvil: &AnvilInstance,
    wallet: LocalWallet,
) -> Arc<SignerMiddleware<Provider<Ws>, LocalWallet>> {
    let provider = Provider::<Ws>::connect(anvil.ws_endpoint())
        .await
        .unwrap()
        .interval(std::time::Duration::from_millis(10u64));
    Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(anvil.chain_id()),
    ))
}
