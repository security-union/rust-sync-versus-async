use std::time::Duration;

use tokio::{
    net::{TcpListener, TcpStream},
    time::sleep,
};
use types::{SLEEP_TIME_MS, http_response};

pub struct AsyncTcpServer {
    listener: TcpListener,
}

impl AsyncTcpServer {
    pub async fn new(endpoint: &str) -> anyhow::Result<AsyncTcpServer> {
        Ok(AsyncTcpServer {
            listener: TcpListener::bind(endpoint).await?,
        })
    }

    pub async fn run_forever(&self) -> anyhow::Result<()> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;
            // You do not have to .await the returned JoinHandle to make the provided future start execution. It will start running in the background immediately when spawn is called.
            tokio::spawn(async move { AsyncTcpServer::process(&mut stream).await.unwrap() });
        }
    }

    async fn process(stream: &mut TcpStream) -> anyhow::Result<()> {
        stream.readable().await?;
        let mut buf = vec![0; 4096];
        match stream.try_read_buf(&mut buf) {
            Ok(_) => {
                stream.writable().await?;
                types::permute();
                stream.try_write(&http_response())?;
                Ok(())
            }
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
