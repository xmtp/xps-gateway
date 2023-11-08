use std::{pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status};

use proto::inbox_server::Inbox;
use proto::{Envelope, GetMessagesRequest, SendMessageRequest, SendMessageResponse};

pub mod proto {
    tonic::include_proto!("xmtp.gateway.inbox.v1");
}

#[derive(Debug, Default)]
pub struct InboxService {}

#[tonic::async_trait]
impl Inbox for InboxService {
    type GetMessagesStream = Pin<Box<dyn Stream<Item = Result<Envelope, Status>> + Send>>;

    async fn send_message(
        &self,
        _request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        // TODO: Push message contents to IPFS and get CID
        // TODO: Build and submit blockchain smart contract transaction.
        let resp = SendMessageResponse {
            transaction_id: "TODO".to_string(),
        };
        Ok(Response::new(resp))
    }

    async fn get_messages(
        &self,
        _request: Request<GetMessagesRequest>,
    ) -> Result<Response<Self::GetMessagesStream>, Status> {
        let repeat = std::iter::repeat(Envelope {
            inbox: "inbox".to_string(),
            message: Vec::new(),
        });
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // TODO: stream events from smart contract based on recipient

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be sent to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::GetMessagesStream
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::inbox::v1::proto::inbox_server::Inbox;
    use crate::inbox::v1::proto::{Envelope, GetMessagesRequest, SendMessageRequest};
    use crate::inbox::v1::InboxService;
    use tokio_stream::StreamExt;
    use tonic::Request;

    fn build_service() -> InboxService {
        InboxService {}
    }

    #[tokio::test]
    async fn test_send_message() {
        let service = build_service();
        let resp = service
            .send_message(Request::new(SendMessageRequest {
                envelopes: Vec::new(),
            }))
            .await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_get_messages() {
        let service = build_service();
        let resp = service
            .get_messages(Request::new(GetMessagesRequest {
                inbox: "inbox".to_string(),
                follow: true,
                start_at: 0,
                end_at: 0,
            }))
            .await;
        assert!(resp.is_ok());
        let mut resp = resp.unwrap().into_inner();
        let envelope1 = resp
            .next()
            .await
            .expect("streamed response is Some")
            .expect("response is ok");
        assert_eq!(
            envelope1,
            Envelope {
                inbox: "inbox".to_string(),
                message: Vec::new(),
            }
        );
        let envelope2 = resp
            .next()
            .await
            .expect("streamed response is Some")
            .expect("response is ok");
        assert_eq!(
            envelope2,
            Envelope {
                inbox: "inbox".to_string(),
                message: Vec::new(),
            }
        );
    }
}
