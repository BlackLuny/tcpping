# TCP Ping Tool

A high-performance TCP ping utility written in Rust, designed for measuring network round-trip times (RTT) with support for concurrent testing.

## Features

- Measure TCP connection RTT
- Configurable packet size
- Support for concurrent testing threads
- Detailed logging
- Both client and server components
- Built with async I/O using Tokio

## Prerequisites

- Rust toolchain (1.70.0 or later)
- Cargo package manager

## Building from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/tcpping.git
cd tcpping
```

2. Build the project:
```bash
cargo build --release
```

The compiled binaries will be available in `target/release/`:
- `tcpping-server`: The TCP echo server
- `tcpping-client`: The TCP ping client

## Usage

### Starting the Server

```bash
cargo run --bin tcpping-server -- [OPTIONS]
```

Server options:
- `--host <ADDRESS>`: Listen address (default: "0.0.0.0")
- `-p, --port <PORT>`: Listen port (default: 8080)

Example:
```bash
cargo run --bin tcpping-server -- --host 127.0.0.1 -p 8080
```

### Running the Client

```bash
cargo run --bin tcpping-client -- [OPTIONS]
```

Client options:
- `--host <ADDRESS>`: Target host address
- `-p, --port <PORT>`: Target port (default: 8080)
- `-s, --size <BYTES>`: Packet size in bytes (default: 1024)
- `-t, --threads <COUNT>`: Number of concurrent test threads (default: 1)
- `-c, --count <NUMBER>`: Number of pings to send (default: 0, meaning infinite)

Example:
```bash
# Send 10 pings with 2 concurrent threads and 2048-byte packets
cargo run --bin tcpping-client -- --host 127.0.0.1 -p 8080 -s 2048 -t 2 -c 10
```

## Project Structure

- `tcpping-common`: Shared library containing common types and constants
- `tcpping-server`: TCP echo server implementation
- `tcpping-client`: TCP ping client implementation

## Output Format

The client outputs detailed information for each ping:
```
Thread [ID] - Reply from [HOST]:[PORT]: bytes=[SIZE] time=[RTT]
```

Example output:
```
Thread 0 - Reply from 127.0.0.1:8080: bytes=1024 time=1.23ms
```

## Error Handling

The tool provides detailed error messages through the logging system:
- Connection failures
- I/O errors
- Configuration errors

## Building for Production

For production use, build with optimizations:

```bash
cargo build --release
```

The optimized binaries will be available in:
- `target/release/tcpping-server`
- `target/release/tcpping-client`

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
