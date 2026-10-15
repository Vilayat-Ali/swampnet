use swampnet::{error::SwampnetResult, logger::setup_logger};
use tokio::net::UdpSocket;
use tracing::info;

#[tokio::main]
async fn main() -> SwampnetResult<()> {
    setup_logger();

    let addr = "0.0.0.0:8080";
    let socket = UdpSocket::bind(addr).await?;
    info!(
        "Swampnet v{} started on {}",
        env!("CARGO_PKG_VERSION"),
        addr
    );

    let mut buf = vec![0u8; 65535];

    loop {
        let (amt, src) = socket.recv_from(&mut buf).await?;

        let packet = &buf[..amt];
        info!("Received {} bytes from drone at {}", amt, src);

        info!("Packet bytes: {:?}", packet);
    }
}
