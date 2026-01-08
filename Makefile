# LiveDraft Arena - Conway Testnet Deployment Makefile

.PHONY: help build-wasm deploy verify start-service start-frontend clean

# Default target
help:
	@echo "LiveDraft Arena - Conway Testnet Deployment"
	@echo "==========================================="
	@echo ""
	@echo "Available targets:"
	@echo "  deploy          - Deploy to Conway testnet (full pipeline)"
	@echo "  verify          - Verify deployment status"
	@echo "  build-wasm      - Build WASM contract only"
	@echo "  start-service   - Start backend service"
	@echo "  start-frontend  - Start frontend development server"
	@echo "  clean           - Clean build artifacts"
	@echo "  help            - Show this help message"
	@echo ""
	@echo "Prerequisites:"
	@echo "  - Linera CLI installed and wallet initialized"
	@echo "  - Rust with wasm32-unknown-unknown target"
	@echo "  - Node.js for frontend development"

# Deploy to Conway testnet (full pipeline)
deploy:
	@echo "🚀 Deploying LiveDraft Arena to Conway testnet..."
	./scripts/deploy_conway.sh

# Verify deployment
verify:
	@echo "🔍 Verifying deployment..."
	./scripts/verify_deployment.sh

# Build WASM contract only
build-wasm:
	@echo "🔨 Building WASM contract..."
	@cd contracts/livedraft-arena && cargo build --target wasm32-unknown-unknown --release

# Start backend service
start-service:
	@echo "🚀 Starting backend service..."
	@cd service && ./start.sh

# Start frontend development server
start-frontend:
	@echo "🌐 Starting frontend development server..."
	@cd frontend && npm run dev

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@rm -f deployment_info.json
	@rm -f service/.env
	@echo "✅ Clean complete"

# Development workflow targets
dev-setup: deploy
	@echo "🔧 Setting up development environment..."
	@echo "✅ Deployment complete. Ready for development!"
	@echo ""
	@echo "Next steps:"
	@echo "  1. Start service: make start-service"
	@echo "  2. Start frontend: make start-frontend"
	@echo "  3. Open http://localhost:3000"

# Quick test deployment
test-deploy: clean deploy verify
	@echo "✅ Test deployment complete!"

# Show deployment status
status:
	@echo "📊 LiveDraft Arena Deployment Status"
	@echo "===================================="
	@echo ""
	@if [ -f "deployment_info.json" ]; then \
		echo "📄 Deployment Info:"; \
		if command -v jq >/dev/null 2>&1; then \
			jq '.' deployment_info.json; \
		else \
			cat deployment_info.json; \
		fi; \
	else \
		echo "❌ No deployment found. Run 'make deploy' first."; \
	fi
	@echo ""
	@if [ -f "service/.env" ]; then \
		echo "🔧 Service Configuration:"; \
		grep -v '^#' service/.env | head -5; \
	else \
		echo "❌ Service not configured."; \
	fi
	@echo ""
	@if [ -f "frontend/src/config.ts" ]; then \
		echo "🌐 Frontend Configuration:"; \
		grep "APP_ID:" frontend/src/config.ts | head -1; \
	else \
		echo "❌ Frontend not configured."; \
	fi