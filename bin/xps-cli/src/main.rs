//! NOTE: NOT MEANT FOR PRODUCTION USE

use argh::FromArgs;
use color_eyre::{eyre::WrapErr, Result};
use ethers::{
    middleware::{Middleware, SignerMiddleware},
    prelude::{Http, Provider, ProviderExt},
    signers::{LocalWallet, Signer},
    types::Address,
};
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use lib_didethresolver::{
    did_registry::{DIDRegistry, RegistrySignerExt},
    types::{KeyEncoding, XmtpAttribute, XmtpKeyPurpose},
};
use lib_xps::XpsClient;
use std::{path::PathBuf, str::FromStr, sync::Arc};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

/// One Year of Validity
const VALIDITY: u64 = 60 * 60 * 24 * 365;

#[derive(FromArgs)]
/// A simple CLI to interact with the XPS Gateway
pub struct App {
    /// the HTTPS network RPC-URL to interact with
    /// By-default, `https://ethereum-sepolia.publicnode.com`
    #[argh(
        short = 'n',
        option,
        default = "String::from(\"https://ethereum-sepolia.publicnode.com\")"
    )]
    network: String,

    /// address of the DID Registry contract to interact with
    ///(default: Test deployment on Sepolia)
    #[argh(
        short = 'c',
        option,
        default = "String::from(\"0xd1D374DDE031075157fDb64536eF5cC13Ae75000\")"
    )]
    contract: String,

    /// path to a local JSON wallet. Ensure usage of a test wallet, the
    /// security of this binary has not been verified. Use at your own risk. (default:
    /// `./wallet.json`)
    #[argh(short = 'w', option, default = "PathBuf::from(\"./wallet.json\")")]
    wallet: PathBuf,

    /// URL of the XPS gateway. Default `ws://localhost:9944`
    #[argh(
        short = 'g',
        long = "gateway",
        option,
        default = "url::Url::parse(\"ws://localhost:9944\").expect(\"Invalid URL\")"
    )]
    gateway_url: url::Url,

    /// the subcommand to execute
    #[argh(subcommand)]
    subcommand: SubCommand,
}

/// The Revoke SubCommand
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommand {
    Grant(Grant),
    Revoke(Revoke),
    Info(Info),
}

/// The Revoke SubCommand
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "revoke")]
struct Revoke {
    /// the hex-encoded value to revoke
    #[argh(short = 'v', option)]
    value: String,
}

/// The Grant SubCommand
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "grant")]
struct Grant {
    /// the hex-encoded value to revoke
    #[argh(short = 'v', option)]
    value: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "info")]
/// Get information about the gateway, like wallet address and current balance.
struct Info {}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_logging();
    let app: App = argh::from_env();
    log::info!("Network: {}", app.network);
    log::info!("Contract: {}", app.contract);
    log::info!("Wallet: {:?}", app.wallet);
    log::info!("Gateway: {}", app.gateway_url.as_str());

    run(app).await?;
    Ok(())
}

/// optimism addr: 0xd1D374D44050cD04f0C1bDBb22A0d5f76BD21900
async fn run(app: App) -> Result<()> {
    let App {
        network,
        contract,
        ref wallet,
        gateway_url,
        subcommand,
    } = app;

    let xps_gateway = WsClientBuilder::default()
        .build(gateway_url.as_str())
        .await
        .wrap_err("Failed to connect to XPS Gateway at {gateway_url}. Is it running?")?;

    if let SubCommand::Info(_) = subcommand {
        info(xps_gateway).await?;
        return Ok(());
    }

    let provider = Provider::<Http>::connect(&network).await;
    println!(
        "CLI Connected to Chain Id: {}",
        provider.get_chainid().await?
    );
    let prompt = format!(
        "Enter Password to {} ",
        app.wallet.as_path().to_str().unwrap()
    );
    let password =
        rpassword::prompt_password(prompt).wrap_err("I/O Error while inputting password")?;
    let wallet = LocalWallet::decrypt_keystore(wallet, password)
        .wrap_err("Could not decrypt keystore; wrong password?")?;

    let signer = SignerMiddleware::new_with_provider_chain(provider, wallet.clone()).await?;

    execute_subcommand(subcommand, xps_gateway, signer.into(), contract).await?;
    Ok(())
}

async fn execute_subcommand(
    subcommand: SubCommand,
    xps: WsClient,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract: String,
) -> Result<()> {
    let contract_address = Address::from_str(&contract).wrap_err("Invalid contract address")?;
    let registry = DIDRegistry::new(contract_address, signer.clone());
    let wallet = signer.signer();

    match subcommand {
        SubCommand::Grant(ref grant) => {
            let grant_value = grant.value.trim_start_matches("0x");
            let value = hex::decode(grant_value).wrap_err("Invalid hex value")?;
            let attribute = XmtpAttribute {
                purpose: XmtpKeyPurpose::Installation,
                encoding: KeyEncoding::Hex,
            };
            let signature = wallet
                .sign_attribute(
                    &registry,
                    attribute.clone().into(),
                    value.clone(),
                    VALIDITY.into(),
                )
                .await?;
            log::info!(
                "Signature for setAttribute {}",
                hex::encode(signature.to_vec())
            );
            log::info!("Setting: xmtp/installation/hex {:?}", grant);

            xps.grant_installation(hex::encode(wallet.address()), attribute, value, signature)
                .await
                .wrap_err("Failed to grant installation")?;
        }
        SubCommand::Revoke(revoke) => {
            log::info!("Revoking: {:?}", revoke);
            let revoke_value = revoke.value.trim_start_matches("0x");
            let value = hex::decode(revoke_value).wrap_err("Invalid hex value")?;
            let attribute = XmtpAttribute {
                purpose: XmtpKeyPurpose::Installation,
                encoding: KeyEncoding::Hex,
            };
            let signature = wallet
                .sign_revoke_attribute(&registry, attribute.clone().into(), value.clone())
                .await?;
            xps.revoke_installation(hex::encode(wallet.address()), attribute, value, signature)
                .await
                .wrap_err("Failed to revoke installation")?;
        }
        _ => {}
    }
    Ok(())
}

async fn info(xps: WsClient) -> Result<()> {
    let address = hex::encode(xps.wallet_address().await?);
    let balance = xps.balance().await?;
    println!(
        r#" ------------------------- XPS Information ---------------------------
        Wallet Address: 0x{address}
        Balance: {balance} 
        "#
    );
    Ok(())
}

fn init_logging() {
    let fmt = fmt::layer().compact();
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt)
        .init()
}
