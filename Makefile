.PHONY: build test clean setup deploy verify service

# Build the project
build:
	cargo build

# Build for release
build-release:
	cargo build --release

# Build WASM for deployment
build-wasm:
	cargo build --target wasm32-unknown-unknown --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Setup development environment
setup:
	./scripts/setup.sh

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy linter
lint:
	cargo clippy

# Run all checks
ci: fmt lint test

# Deploy to Conway testnet
deploy:
	./scripts/deploy_conway.sh

# Verify deployment
verify:
	./scripts/verify_deployment.sh

# Full deployment pipeline
deploy-full: build-wasm deploy verify

# Start the service
service:
	cd service && ./start.sh

# Build service
service-build:
	cd service && cargo build --release