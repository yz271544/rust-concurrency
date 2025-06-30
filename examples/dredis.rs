use anyhow::Result;
use std::{io, net::SocketAddr};
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // build a listener
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: Listening on {}", addr);

    loop {
        // accept a new connection
        let (stream, raddr) = listener.accept().await?;
        info!("Dredis: Accepted a new connection from: {}", raddr);

        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("Error processing conn with {}: {:?}", raddr, e);
            }
        });
    }
}

async fn process_redis_conn(mut stream: tokio::net::TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Dredis: Connection closed by client: {}", raddr);
    // For now, just close the connection
    Ok(())
}
