use anyhow::Error;
use jsonrpsee::{
    server::Server,
    ws_client::{WsClient, WsClientBuilder},
};

use futures::future::FutureExt;
use std::{future::Future, time::Duration};
use tokio::time::timeout as timeout_tokio;

use xps_gateway::{rpc::XpsClient, types::Message, XpsMethods, XpsServer, SERVER_HOST};

const TEST_TIMEOUT: Duration = Duration::from_secs(10);
const TEST_WALLET_ADDRESS: &str = "0x000000000000000000000000000000000000dEaD";

#[cfg(test)]
mod it {
    use super::*;

    #[tokio::test]
    async fn test_say_hello() -> Result<(), Error> {
        with_xps_client(None, |client| async move {
            let result = client.status().await?;
            assert_eq!(result, "OK");
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_fail_send_message() -> Result<(), Error> {
        with_xps_client(None, |client| async move {
            let message = Message {
                conversation_id: b"abcdefg".iter().map(|c| *c as u8).collect(),
                payload: b"Hello World".iter().map(|c| *c as u8).collect(),
                v: vec![],
                r: vec![],
                s: vec![],
            };
            let result = client.send_message(message).await;
            assert!(result.is_err());
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_wallet_address() -> Result<(), Error> {
        with_xps_client(None, |client| async move {
            let result = client.wallet_address().await?;
            assert_eq!(result, TEST_WALLET_ADDRESS);
            Ok(())
        })
        .await
    }
}

async fn with_xps_client<F, R, T>(timeout: Option<Duration>, f: F) -> Result<T, Error>
where
    F: FnOnce(WsClient) -> R + 'static + Send,
    R: Future<Output = Result<T, Error>> + FutureExt + Send + 'static,
{
    let server = Server::builder().build(SERVER_HOST).await.unwrap();
    let addr = server.local_addr().unwrap();
    let handle = server.start(XpsMethods::new(TEST_WALLET_ADDRESS).into_rpc());
    let client = WsClientBuilder::default()
        .build(&format!("ws://{addr}"))
        .await
        .unwrap();
    let result = timeout_tokio(timeout.unwrap_or(TEST_TIMEOUT), f(client)).await;

    handle.stop().unwrap();
    handle.stopped().await;

    match result {
        Ok(v) => v,
        Err(_) => panic!("Test timed out"),
    }
}
