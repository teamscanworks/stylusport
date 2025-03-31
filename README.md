# StylusPort

`StylusPort` is a toolkit to convert Solana [Anchor](https://www.anchor-lang.com/docs) programs to Arbitrum [Stylus](https://arbitrum.io/stylus).

## Documentation

- [Specifications](docs/specs/)
- [Developer Documentation](docs/dev.md)

## Prerequisites

- Rust
- Cargo (included with Rust)
- Just - Command runner for project tasks

## Installation

After cloning the repository:
```shell
cd stylusport

# Install just command runner (if not already installed)
cargo install just

# Install required development tools
just setup
```

## Building and Testing

Below are the main commands:
```shell
# List all available commands
just --list

# Build all crates
just build

# Run all tests
just test

# Format code
just fmt

# Check for linting issues
just lint
```
