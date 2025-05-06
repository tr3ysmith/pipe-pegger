# Pipe Pegger
Pipe Pegger 🚰 - The FIFO Stress Test Tool That Never Sleeps

A high-performance, low-latency named pipe writer that'll make your FIFOs sweat. Perfect for when you need to:
- Test pipe performance with surgical precision
- Keep your pipes busy (they get lonely too)
- Show your system who's boss with CPU core pinning
- Write bytes faster than your ex can write bad tweets

Because sometimes you just need to peg a pipe. 🎯

## Installation

### Prerequisites
- Rust toolchain (install via [rustup](https://rustup.rs/))
- Linux system (this tool uses Linux-specific features)

### Building
```bash
# Clone the repository
git clone https://github.com/yourusername/pipe-pegger.git
cd pipe-pegger

# Build in release mode
cargo build --release

# The binary will be available at target/release/pipe-pegger
```

## Usage

> The tool will automatically create a new FIFO pipe if it doesn't already exist.

### Basic Usage
```bash
# Write 0x00 bytes to the pipe for 10 seconds
./pipe-pegger -n /tmp/my_pipe
```

### Advanced Options
```bash
# Write 100 bytes of 0xFF every 5ms for 30 seconds on CPU core 0
./pipe-pegger -n /tmp/my_pipe -c 100 -d 5 -b 255 -t 30 --core 0

# Write single 0x42 bytes every 1ms for 60 seconds
./pipe-pegger -n /tmp/my_pipe -b 66 -t 60
```

### Command Line Arguments
- `-n, --name`: Path to the named pipe (required)
- `-c, --count`: Number of bytes to write per iteration (default: 1)
- `-d, --delay`: Delay between writes in microseconds (default: 1000µs / 1ms)
- `-b, --byte-value`: Byte value to write (0-255, default: 0)
- `-t, --duration`: Duration to run in seconds (default: 10)
- `--core`: CPU core to run on (optional)

## Example Use Cases

### Testing Pipe Performance
```bash
# Write 1024 bytes every 1ms for 5 seconds
./pipe-pegger -n /tmp/test_pipe -c 1024 -t 5
```

### Stress Testing
```bash
# Write 4096 bytes of 0xFF every 10µs for 1 minute
./pipe-pegger -n /tmp/stress_pipe -c 4096 -d 10 -b 255 -t 60
```

### CPU Core Pinning
```bash
# Pin to CPU core 0 and write 100 bytes every 1ms
./pipe-pegger -n /tmp/pinned_pipe -c 100 --core 0
```