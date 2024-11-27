use std::time::Duration;

pub const DEFAULT_PACKET_SIZE: usize = 1024;
pub const DEFAULT_PORT: u16 = 8080;
pub const DEFAULT_THREADS: usize = 1;

#[derive(Debug, Clone)]
pub struct PingConfig {
    pub packet_size: usize,
    pub threads: usize,
    pub host: String,
    pub port: u16,
    pub keep_alive: bool,
}

#[derive(Debug)]
pub struct PingResult {
    pub rtt: Duration,
    pub bytes: usize,
}

impl Default for PingConfig {
    fn default() -> Self {
        Self {
            packet_size: DEFAULT_PACKET_SIZE,
            threads: DEFAULT_THREADS,
            host: "127.0.0.1".to_string(),
            port: DEFAULT_PORT,
            keep_alive: false,
        }
    }
}
