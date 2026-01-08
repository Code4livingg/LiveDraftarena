#!/bin/bash

# LiveDraft Arena Service Startup Script
# Loads configuration and starts the backend service
# Supports both development and production deployment

set -e

echo "🚀 Starting LiveDraft Arena Service"
echo "=================================="

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Detect deployment mode
DEPLOYMENT_MODE=${DEPLOYMENT_MODE:-"development"}
echo -e "${BLUE}🏗️  Deployment Mode: $DEPLOYMENT_MODE${NC}"

# Check if .env file exists and load it
ENV_FILE=".env"
if [ -f "$ENV_FILE" ]; then
    echo -e "${BLUE}📋 Loading configuration from $ENV_FILE${NC}"
    
    # Export variables from .env file
    export $(grep -v '^#' "$ENV_FILE" | xargs)
    
    # Display loaded configuration (without sensitive values)
    if [ ! -z "$LIVEDRAFT_APP_ID" ]; then
        echo -e "${GREEN}✅ Application ID: $LIVEDRAFT_APP_ID${NC}"
    else
        echo -e "${RED}❌ LIVEDRAFT_APP_ID not found in $ENV_FILE${NC}"
    fi
    
    if [ ! -z "$LIVEDRAFT_CHAIN_ID" ]; then
        echo "   Chain ID: $LIVEDRAFT_CHAIN_ID"
    fi
    
    if [ ! -z "$PORT" ]; then
        echo "   Port: $PORT"
    fi
    
    if [ ! -z "$BIND_ADDRESS" ]; then
        echo "   Bind Address: $BIND_ADDRESS"
    fi
    
    if [ ! -z "$CORS_ORIGINS" ]; then
        echo "   CORS Origins: $CORS_ORIGINS"
    fi
else
    echo -e "${YELLOW}⚠️  No .env file found. Using environment variables only.${NC}"
    
    if [ -z "$LIVEDRAFT_APP_ID" ]; then
        echo -e "${RED}❌ Error: LIVEDRAFT_APP_ID not set${NC}"
        echo ""
        echo "Please either:"
        echo "  1. Run deployment: ./scripts/deploy_conway.sh"
        echo "  2. Set environment: export LIVEDRAFT_APP_ID=your-app-id"
        echo "  3. Create .env file with: LIVEDRAFT_APP_ID=your-app-id"
        exit 1
    fi
fi

# Set defaults based on deployment mode
if [ "$DEPLOYMENT_MODE" = "production" ]; then
    # Production defaults
    export PORT=${PORT:-8080}
    export BIND_ADDRESS=${BIND_ADDRESS:-"0.0.0.0"}
    export RUST_LOG=${RUST_LOG:-"info"}
    export CORS_ORIGINS=${CORS_ORIGINS:-"https://your-frontend-domain.com"}
    
    echo -e "${GREEN}🏭 Production Configuration Applied${NC}"
    echo "   - Binding to 0.0.0.0 for public access"
    echo "   - Restricted CORS origins for security"
    echo "   - Info-level logging for performance"
else
    # Development defaults
    export PORT=${PORT:-8080}
    export BIND_ADDRESS=${BIND_ADDRESS:-"0.0.0.0"}
    export RUST_LOG=${RUST_LOG:-"debug"}
    export CORS_ORIGINS=${CORS_ORIGINS:-"*"}
    
    echo -e "${YELLOW}🔧 Development Configuration Applied${NC}"
    echo "   - Binding to 0.0.0.0 for local testing"
    echo "   - Open CORS for development"
    echo "   - Debug-level logging for development"
fi

export LINERA_WALLET_PATH=${LINERA_WALLET_PATH:-~/.config/linera/wallet.json}

echo ""
echo -e "${BLUE}📋 Final Configuration:${NC}"
echo "  Application ID: $LIVEDRAFT_APP_ID"
echo "  Port: $PORT"
echo "  Bind Address: $BIND_ADDRESS"
echo "  CORS Origins: $CORS_ORIGINS"
echo "  Wallet Path: $LINERA_WALLET_PATH"
echo "  Log Level: $RUST_LOG"

if [ -n "$LIVEDRAFT_CHAIN_ID" ]; then
    echo "  Chain ID: $LIVEDRAFT_CHAIN_ID"
else
    echo "  Chain ID: (using default from wallet)"
fi

echo ""

# Verify Linera wallet is accessible
echo -e "${BLUE}🔗 Verifying Linera wallet access...${NC}"
if command -v linera &> /dev/null; then
    if linera wallet show &> /dev/null; then
        echo -e "${GREEN}✅ Linera wallet accessible${NC}"
    else
        echo -e "${YELLOW}⚠️  Linera wallet not initialized or not accessible${NC}"
        echo "   Service will attempt to load wallet from default location"
    fi
else
    echo -e "${YELLOW}⚠️  Linera CLI not found in PATH${NC}"
    echo "   Service will attempt to load wallet from default location"
fi

# Check if wallet file exists
if [ ! -f "$LINERA_WALLET_PATH" ]; then
    echo -e "${RED}❌ Error: Wallet file not found at $LINERA_WALLET_PATH${NC}"
    echo "Please ensure linera CLI is initialized:"
    echo "  linera wallet init --with-new-chain"
    exit 1
fi

echo -e "${GREEN}✅ Wallet found at $LINERA_WALLET_PATH${NC}"
echo ""

# Start the service
echo -e "${BLUE}🚀 Starting LiveDraft Arena service...${NC}"

if [ "$DEPLOYMENT_MODE" = "production" ]; then
    echo -e "${BLUE}  GraphQL endpoint: http://$BIND_ADDRESS:$PORT/graphql${NC}"
    echo -e "${BLUE}  GraphQL playground: http://$BIND_ADDRESS:$PORT/playground${NC}"
    echo -e "${BLUE}  Health check: http://$BIND_ADDRESS:$PORT/health${NC}"
    echo ""
    echo -e "${GREEN}🏭 Running in PRODUCTION mode${NC}"
    
    # Build and run optimized binary for production
    echo -e "${BLUE}📦 Building optimized binary...${NC}"
    cargo build --release
    
    echo -e "${BLUE}🚀 Starting production service...${NC}"
    RUST_LOG=$RUST_LOG ./target/release/livedraft-arena-service
else
    echo -e "${BLUE}  GraphQL endpoint: http://$BIND_ADDRESS:$PORT/graphql${NC}"
    echo -e "${BLUE}  GraphQL playground: http://$BIND_ADDRESS:$PORT/playground${NC}"
    echo -e "${BLUE}  Health check: http://$BIND_ADDRESS:$PORT/health${NC}"
    echo ""
    echo -e "${YELLOW}🔧 Running in DEVELOPMENT mode${NC}"
    
    # Run in development mode with logs
    RUST_LOG=$RUST_LOG cargo run
fi