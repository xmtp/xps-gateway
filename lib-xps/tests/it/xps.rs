use anyhow::Error;

use crate::integration_util::*;
use ethers::providers::Middleware;
use ethers::types::{TransactionRequest, U256};

use lib_xps::rpc::XpsClient;
use xps_types::Unit;

#[tokio::test]
async fn test_say_hello() -> Result<(), Error> {
    with_xps_client(None, None, |client, _, _, _| async move {
        let result = client.status().await?;
        assert_eq!(result, "OK");
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_balance() -> Result<(), Error> {
    with_xps_client(
        None,
        Some(0.into()),
        |client, context, _resolver, _anvil| async move {
            // by default, we have no balance. verify that.
            let mut balance = client.balance().await?;
            assert_eq!(balance.balance, U256::from(0));
            assert_eq!(balance.unit, Unit::Eth);

            // fund the wallet account.
            let accounts = context.signer.get_accounts().await?;
            let from = accounts[1];
            let tx = TransactionRequest::new()
                .to(client.wallet_address().await?)
                .value(5_000_000_000_000_000_000_000_u128)
                .from(from);
            context.signer.send_transaction(tx, None).await?.await?;

            // check to see if the balance gets updated.
            balance = client.balance().await?;
            assert_eq!(
                balance.balance,
                U256::from(5_000_000_000_000_000_000_000_u128)
            );
            assert_eq!(balance.unit, Unit::Eth);

            Ok(())
        },
    )
    .await
}
