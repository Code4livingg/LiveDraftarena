#!/bin/bash

# LiveDraft Arena Service Startup Script
# This script starts the Linera service with proper configuration

set -e

echo "🚀 Starting LiveDraft Arena Service"
echo "=================================="

# Check if required environment variables are set
if [ -z "$LIVEDRAFT_APP_ID" ]; then
    echo "❌ Error: LIVEDRAFT_APP_ID environment variable not set"
    echo "Please set it to your deployed application ID:"
    echo "export LIVEDRAFT_APP_ID=your_app_id_here"
    exit 1
fi

# Optional environment variables with defaults
export PORT=${PORT:-8080}
export RUST_LOG=${RUST_LOG:-info}

# Wallet path (defaults to standard Linera location)
export LINERA_WALLET_PATH=${LINERA_WALLET_PATH:-~/.config/linera/wallet.json}

# Chain ID (optional - will use default from wallet if not set)
# export LIVEDRAFT_CHAIN_ID=your_chain_id_here

echo "📋 Configuration:"
echo "  Application ID: $LIVEDRAFT_APP_ID"
echo "  Port: $PORT"
echo "  Wallet Path: $LINERA_WALLET_PATH"
echo "  Log Level: $RUST_LOG"

if [ -n "$LIVEDRAFT_CHAIN_ID" ]; then
    echo "  Chain ID: $LIVEDRAFT_CHAIN_ID"
else
    echo "  Chain ID: (using default from wallet)"
fi

echo ""

# Check if wallet exists
if [ ! -f "$LINERA_WALLET_PATH" ]; then
    echo "❌ Error: Wallet file not found at $LINERA_WALLET_PATH"
    echo "Please ensure linera CLI is initialized:"
    echo "  linera wallet init --with-new-chain"
    exit 1
fi

echo "✅ Wallet found at $LINERA_WALLET_PATH"

# Build and run the service
echo "🔨 Building service..."
cargo build --release

echo "🌐 Starting service..."
echo "  GraphQL endpoint: http://localhost:$PORT/graphql"
echo "  GraphQL playground: http://localhost:$PORT/playground"
echo "  Health check: http://localhost:$PORT/health"
echo ""

# Run the service
cargo run --release