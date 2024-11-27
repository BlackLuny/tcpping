use anyhow::Result;
use clap::Parser;
use std::time::Instant;
use tcpping_common::{PingConfig, PingResult, DEFAULT_PACKET_SIZE, DEFAULT_PORT, DEFAULT_THREADS};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;
use std::time::Duration;
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
    
    // Set a 5-second timeout for the entire operation
    let result = timeout(Duration::from_secs(5), async {
        let mut stream = TcpStream::connect(addr).await?;
        
        // Set TCP_NODELAY to disable Nagle's algorithm
        stream.set_nodelay(true)?;
        
        let data = vec![1u8; size];
        stream.write_all(&data).await?;
        
        let mut buf = vec![0u8; size];
        stream.read_exact(&mut buf).await?;
        
        Ok::<_, anyhow::Error>(())
    }).await;
    
    match result {
        Ok(Ok(())) => Ok(PingResult {
            rtt: start.elapsed(),
            bytes: size,
        }),
        Ok(Err(e)) => Err(e),
        Err(_) => anyhow::bail!("connection timed out"),
    }
}
