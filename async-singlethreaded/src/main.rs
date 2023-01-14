use async_multithreaded::AsyncTcpServer;
use types::ENDPOINT;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server = AsyncTcpServer::new(ENDPOINT).await.unwrap();
    server.run_forever().await.unwrap();
}
