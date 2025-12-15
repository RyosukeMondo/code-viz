# Justfile for code-viz

set dotenv-load

# List available recipes
default:
    @just --list

# Run all checks (format, clippy, test)
check: fmt clippy test

# Format code
fmt:
    cargo fmt --all

# Run clippy
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test --all-features

# Build release binaries
release:
    cargo build --release

# Run the CLI (wrapper)
run *args:
    cargo run -p code-viz-cli -- {{args}}
