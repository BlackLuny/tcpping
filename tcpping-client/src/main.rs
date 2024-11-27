use anyhow::Result;
use clap::Parser;
use std::time::Instant;
use tcpping_common::{PingConfig, PingResult, DEFAULT_PACKET_SIZE, DEFAULT_PORT, DEFAULT_THREADS};
use tokio::net::TcpStream;
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target host
    #[arg(long)]
    host: String,

    /// Target port
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,

    /// Packet size in bytes
    #[arg(short, long, default_value_t = DEFAULT_PACKET_SIZE)]
    size: usize,

    /// Number of concurrent threads
    #[arg(short, long, default_value_t = DEFAULT_THREADS)]
    threads: usize,

    /// Number of pings to send (0 for infinite)
    #[arg(short, long, default_value_t = 0)]
    count: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let config = PingConfig {
        packet_size: args.size,
        threads: args.threads,
        host: args.host,
        port: args.port,
    };

    let mut handles = vec![];
    for i in 0..config.threads {
        let config = config.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = run_ping(config, i, args.count).await {
                warn!("Thread {} error: {}", i, e);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

async fn run_ping(config: PingConfig, thread_id: usize, count: u32) -> Result<()> {
    let addr = format!("{}:{}", config.host, config.port);
    let mut ping_count = 0;

    loop {
        if count > 0 && ping_count >= count {
            break;
        }

        match ping_once(&addr, config.packet_size).await {
            Ok(result) => {
                info!(
                    "Thread {} - Reply from {}: bytes={} time={:?}",
                    thread_id, addr, result.bytes, result.rtt
                );
            }
            Err(e) => {
                warn!("Thread {} - Failed to ping {}: {}", thread_id, addr, e);
            }
        }

        ping_count += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn ping_once(addr: &str, size: usize) -> Result<PingResult> {
    let start = Instant::now();
    let stream = TcpStream::connect(addr).await?;

    let data = vec![1u8; size];
    stream.try_write(&data)?;

    let mut buf = vec![0u8; size];
    let n = stream.try_read(&mut buf)?;

    Ok(PingResult {
        rtt: start.elapsed(),
        bytes: n,
    })
}
