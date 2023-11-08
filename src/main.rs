use std::net::SocketAddr;
use std::time::Duration;

use http::header::HeaderName;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod inbox;

use inbox::v1::proto::inbox_server::InboxServer;

use clap::Parser;

/// XMTP RPC node
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on for public gRPC
    #[arg(short, long, default_value_t = 50051)]
    port: u16,
}

const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
    ["grpc-status", "grpc-message", "grpc-status-details-bin"];
const DEFAULT_ALLOW_HEADERS: [&str; 4] =
    ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let inbox_v1 = inbox::v1::InboxService::default();

    let addr: SocketAddr = format!("[::1]:{}", args.port).parse()?;
    let server = Server::builder()
        .accept_http1(true)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::mirror_request())
                .allow_credentials(true)
                .max_age(DEFAULT_MAX_AGE)
                .expose_headers(
                    DEFAULT_EXPOSED_HEADERS
                        .iter()
                        .cloned()
                        .map(HeaderName::from_static)
                        .collect::<Vec<HeaderName>>(),
                )
                .allow_headers(
                    DEFAULT_ALLOW_HEADERS
                        .iter()
                        .cloned()
                        .map(HeaderName::from_static)
                        .collect::<Vec<HeaderName>>(),
                ),
        )
        .layer(GrpcWebLayer::new())
        .add_service(InboxServer::new(inbox_v1));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on {}", listener.local_addr().unwrap());
    let stream = tokio_stream::wrappers::TcpListenerStream::new(listener);
    server.serve_with_incoming(stream).await?;

    Ok(())
}
