use tokio::net::TcpListener;
use tracing::{error, info};

use crate::connection::handle_client;

pub async fn run_listener(addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!("client connected: {}", peer_addr);

        tokio::spawn(async move {
            if let Err(err) = handle_client(stream).await {
                error!("client {} error: {:?}", peer_addr, err);
            }
        });
    }
}
