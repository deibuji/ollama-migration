# Ollama Migrator - Just commands
# https://github.com/casey/just

# Default recipe - show available commands
default:
    @just --list

# Build debug binary
build:
    cargo build

# Build release binary (optimized)
release:
    cargo build --release

# Run all tests
test:
    cargo test --lib

# Run tests with output
verbose-test:
    cargo test --lib -- --nocapture

# Run clippy lints
clippy:
    cargo clippy -- -D warnings

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Check formatting without modifying
fmt-check:
    cargo fmt -- --check

# Generate documentation
doc:
    cargo doc --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# Run the CLI (debug build)
run *ARGS:
    cargo run -- {{ARGS}}

# Run with --help
help:
    cargo run -- --help

# Quick dev cycle: format, check, test
dev: fmt check test

# Full CI check: format-check, clippy, test
ci: fmt-check clippy test

# Install locally (cargo install --path .)
install:
    cargo install --path .

# Uninstall local install
uninstall:
    cargo uninstall ollama-migrator

# Build and run release binary
run-release *ARGS:
    cargo build --release
    ./target/release/ollama-migrator {{ARGS}}

# List installed Ollama models (requires Ollama installation)
list:
    cargo run -- list

# List with JSON output
list-json:
    cargo run -- list --format json

# Example: export a model (dry-run path)
export-example:
    @echo "Example: just export llama3.2:latest ./output.gguf"

# Export command
export MODEL OUTPUT:
    cargo run -- export {{MODEL}} --output {{OUTPUT}}

# Export all models
export-all OUTPUT_DIR="./exported":
    cargo run -- export-all --output {{OUTPUT_DIR}}

# Verify a GGUF file
verify FILE:
    cargo run -- verify {{FILE}}

# Show model info
info MODEL:
    cargo run -- info {{MODEL}}

# Profile build size
size:
    cargo build --release
    ls -lh target/release/ollama-migrator
    @echo "Binary size:"
    du -h target/release/ollama-migrator

# Run benchmarks (if criterion added in future)
bench:
    cargo bench

# Tail cargo build output
watch:
    cargo watch -x build

# Development server with auto-reload on changes
watch-test:
    cargo watch -x "test --lib"
