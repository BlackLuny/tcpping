# TCPping - A Modern TCP Ping Tool

A cross-platform TCP ping utility written in Rust, supporting multi-threaded network connectivity testing.

## Features

- TCP-based connectivity testing
- Multi-threaded parallel pings
- Configurable packet size and interval
- Keep-alive connection support
- Cross-platform support (Linux, macOS, Windows)
- Static builds available for Linux (MUSL)

## Quick Start

### Download and Install

1. Visit the [Releases](https://github.com/BlackLuny/tcpping/releases) page
2. Download the appropriate binary for your platform:
   - Linux (GNU): `tcpping-x86_64-linux-gnu.tar.gz`
   - Linux (Static/MUSL): `tcpping-x86_64-linux-musl.tar.gz`
   - macOS (Intel): `tcpping-x86_64-darwin.tar.gz`
   - macOS (Apple Silicon): `tcpping-aarch64-darwin.tar.gz`
   - Windows: `tcpping-x86_64-windows.zip`

3. Extract the archive:
```bash
# Linux/macOS
tar xzf tcpping-*.tar.gz

# Windows
# Use Windows Explorer to extract the zip file
```

4. (Optional) Move to a directory in your PATH:
```bash
# Linux/macOS
sudo mv tcpping-client /usr/local/bin/tcpping
sudo mv tcpping-server /usr/local/bin/tcpping-server

# Windows
# Copy to C:\Windows\System32 or add to PATH
```

### Basic Usage

1. Start the server (optional, only if you want to test against your own server):
```bash
# Listen on default port 8080
tcpping-server

# Listen on specific port
tcpping-server -p 4001
```

2. Run TCP ping tests:
```bash
# Basic ping test
tcpping-client --host example.com -p 80

# Specify packet size
tcpping-client --host example.com -p 80 -s 1300

# Use multiple threads
tcpping-client --host example.com -p 80 -t 4

# Limit number of pings
tcpping-client --host example.com -p 80 -c 10

# Keep TCP connection alive between pings
tcpping-client --host example.com -p 80 -k
```

### Example Test Scenarios

1. Test web server connectivity:
```bash
# Test HTTP
tcpping-client --host google.com -p 80

# Test HTTPS
tcpping-client --host google.com -p 443
```

2. Test database connectivity:
```bash
# MySQL/MariaDB
tcpping-client --host db.example.com -p 3306

# PostgreSQL
tcpping-client --host db.example.com -p 5432

# Redis
tcpping-client --host cache.example.com -p 6379
```

3. Load testing with multiple threads:
```bash
# 10 concurrent connections
tcpping-client --host api.example.com -p 443 -t 10
```

4. Connection stability test:
```bash
# Keep connection alive, run 100 pings
tcpping-client --host api.example.com -p 443 -k -c 100
```

## Build from Source

1. Install Rust (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone and build:
```bash
git clone https://github.com/BlackLuny/tcpping.git
cd tcpping
cargo build --release
```

The optimized binaries will be available in:
- `target/release/tcpping-server`
- `target/release/tcpping-client`

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

## Release Process

To create a new release, use the provided release script:

```bash
# Create a new release with version x.y.z (e.g., 1.2.3)
./scripts/release.sh 1.2.3

# Force create/recreate a release
./scripts/release.sh -f 1.2.3

# Use a different version prefix (default is 'v')
./scripts/release.sh -p r 2.0.0
```

The script will:
1. Validate the version format (must be x.y.z)
2. Check for uncommitted changes
3. Create and push a Git tag
4. Trigger the GitHub Actions workflow to:
   - Build all platform binaries
   - Create a GitHub release
   - Upload the binaries as release assets

Available platforms:
- Linux (x86_64, GNU libc)
- Linux (x86_64, MUSL libc, static)
- macOS (x86_64, Intel)
- macOS (aarch64, Apple Silicon)
- Windows (x86_64)

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
