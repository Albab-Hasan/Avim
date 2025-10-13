# Build Instructions

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Building

```bash
cd Avim
cargo build --release
```

The compiled binary will be located at `target/release/avim`.

## Running

```bash
# Run directly with cargo
cargo run --release

# Run with a file
cargo run --release -- path/to/file.txt

# Or use the compiled binary
./target/release/avim
./target/release/avim path/to/file.txt
```

## Development Build

For faster compilation during development:

```bash
cargo build
cargo run -- path/to/file.txt
```

## Testing

```bash
cargo test
```

## Installing

To install the binary to your system:

```bash
cargo install --path .
```

Then you can run `avim` from anywhere.

