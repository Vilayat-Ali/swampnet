use swampnet::{error::SwampnetResult, logger::init_logger};
use tokio::net::UdpSocket;
use tracing::{info, warn};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> SwampnetResult<()> {
    let addr = "0.0.0.0:8004";
    let (_stdout_guard, _file_guard) = init_logger()?;

    let udp_socket = UdpSocket::bind(addr).await?;
    info!(address = %addr, "Swampnet v{} has started", VERSION);

    let mut buffer: [u8; 2048] = [0; 2048];

    loop {
        match udp_socket.recv_from(&mut buffer).await {
            Ok((size, peer)) => {
                info!(%peer, bytes = %size, "Received data packet");
            }
            Err(e) => {
                warn!(error = %e, "Failed recieved UDP packet");
            }
        }
    }
}
