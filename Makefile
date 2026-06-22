# OrbitNFT Makefile - Common development commands

.PHONY: build test clean deploy fmt lint

# Build the contract
build:
	cargo build --target wasm32v1-none --release

# Run all tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Run linter
lint:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Deploy to Stellar Testnet
deploy:
	soroban contract deploy \
		--wasm target/wasm32v1-none/release/orbitnft.wasm \
		--network testnet

# Build and deploy in one step
all: build deploy

# Show help
help:
	@echo "Available commands:"
	@echo "  make build   - Build the smart contract"
	@echo "  make test    - Run all tests"
	@echo "  make fmt     - Format code with cargo fmt"
	@echo "  make lint    - Run cargo clippy linter"
	@echo "  make clean   - Clean build artifacts"
	@echo "  make deploy  - Deploy to Stellar Testnet"
	@echo "  make all     - Build and deploy"
