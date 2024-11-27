use anyhow::Result;
use clap::Parser;
use tcpping_common::{DEFAULT_PACKET_SIZE, DEFAULT_PORT};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Listen address
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Listen port
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!("New connection from {}", addr);
                tokio::spawn(handle_connection(socket));
            }
            Err(e) => {
                warn!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buf = vec![0u8; DEFAULT_PACKET_SIZE];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                // Connection closed
                break;
            }
            Ok(n) => {
                if let Err(e) = socket.write_all(&buf[..n]).await {
                    warn!("Failed to write to socket: {}", e);
                    break;
                }
            }
            Err(e) => {
                warn!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}
