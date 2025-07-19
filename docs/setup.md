# Setup Guide

## Prerequisites

- **Rust**: Version 1.70 or higher
- **Cargo**: Included with Rust installation
- **Git**: For version control

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/DScudeler/gh_actions.git
cd gh_actions
```

### 2. Build the Project

```bash
# Debug build (development)
cargo build

# Release build (production)
cargo build --release
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test user_manager::tests
```

### 4. Run Benchmarks

```bash
cargo bench
```

This will generate HTML reports in `target/criterion/` directory.

## Development Setup

### 1. Install Development Tools

```bash
# Install rustfmt for code formatting
rustup component add rustfmt

# Install clippy for linting
rustup component add clippy

# Install cargo-watch for auto-rebuilding
cargo install cargo-watch
```

### 2. Verify Installation

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Auto-rebuild on changes
cargo watch -x build
```

### 3. IDE Setup

For **VS Code**:
- Install the "rust-analyzer" extension
- Install "CodeLLDB" extension for debugging

For **CLion/IntelliJ**:
- Install the Rust plugin

## Configuration

### Rust Toolchain

The project uses a specific Rust toolchain defined in `rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
```

### Dependencies

Key dependencies are managed in `Cargo.toml`:

- **clap**: CLI argument parsing
- **serde**: Serialization/deserialization
- **serde_json**: JSON handling

Development dependencies:
- **criterion**: Benchmarking
- **assert_cmd**: CLI testing
- **tempfile**: Temporary file handling

## Troubleshooting

### Common Issues

1. **Rust not found**: Ensure Rust is installed via [rustup.rs](https://rustup.rs/)
2. **Build errors**: Try `cargo clean` then `cargo build`
3. **Test failures**: Check if you have the latest dependencies with `cargo update`

### Performance Issues

- Use release builds for performance testing: `cargo build --release`
- Run benchmarks to identify bottlenecks: `cargo bench`
- Check memory usage with tools like `valgrind` or `heaptrack`

### Getting Help

- Check [GitHub Issues](https://github.com/DScudeler/gh_actions/issues)
- Review test files in `tests/` directory for usage examples
- Consult Rust documentation at [doc.rust-lang.org](https://doc.rust-lang.org/)