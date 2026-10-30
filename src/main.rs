use swampnet::{error::SwampnetResult, logger::init_logger};
use tokio::net::UdpSocket;
use tracing::info;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> SwampnetResult<()> {
    let addr = "0.0.0.0:8004";
    let (_stdout_guard, _file_guard) = init_logger()?;

    let udp_socket = UdpSocket::bind(addr).await?;
    info!(address = %addr, "Swampnet v{} have started", VERSION);

    loop {
        udp_socket.writable().await?;
    }
}
