#!/bin/bash

# LiveDraft Arena - Conway Testnet Deployment Script
# This script builds and deploys the LiveDraft Arena contracts to Conway testnet

set -e  # Exit on any error

echo "🚀 LiveDraft Arena - Conway Testnet Deployment"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CONTRACT_PATH="contracts/livedraft-arena"
FRONTEND_LINERA_FILE="frontend/src/linera.ts"
WASM_OUTPUT_DIR="target/wasm32-unknown-unknown/release"

echo -e "${BLUE}📋 Deployment Configuration:${NC}"
echo "  Contract Path: $CONTRACT_PATH"
echo "  Target: Conway Testnet"
echo "  Frontend Config: $FRONTEND_LINERA_FILE"
echo ""

# Step 1: Verify prerequisites
echo -e "${YELLOW}🔍 Step 1: Verifying prerequisites...${NC}"

# Check if linera CLI is available
if ! command -v linera &> /dev/null; then
    echo -e "${RED}❌ Error: linera CLI not found. Please install Linera CLI first.${NC}"
    exit 1
fi

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Error: cargo not found. Please install Rust first.${NC}"
    exit 1
fi

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}⚠️  Installing wasm32-unknown-unknown target...${NC}"
    rustup target add wasm32-unknown-unknown
fi

echo -e "${GREEN}✅ Prerequisites verified${NC}"
echo ""

# Step 2: Build WASM contract
echo -e "${YELLOW}🔨 Step 2: Building WASM contract...${NC}"

# Clean previous builds
echo "  Cleaning previous builds..."
cargo clean

# Build the contract for WASM target
echo "  Building contract for wasm32-unknown-unknown..."
cd $CONTRACT_PATH
cargo build --target wasm32-unknown-unknown --release

# Verify WASM file was created
WASM_FILE="../../$WASM_OUTPUT_DIR/livedraft_arena.wasm"
if [ ! -f "$WASM_FILE" ]; then
    echo -e "${RED}❌ Error: WASM file not found at $WASM_FILE${NC}"
    echo "Expected file: livedraft_arena.wasm"
    echo "Available files:"
    ls -la "../../$WASM_OUTPUT_DIR/" || echo "Directory not found"
    exit 1
fi

cd ../..
echo -e "${GREEN}✅ WASM contract built successfully${NC}"
echo "  WASM file: $WASM_FILE"
echo ""

# Step 3: Check Linera wallet and chain status
echo -e "${YELLOW}🔗 Step 3: Checking Linera wallet and chain status...${NC}"

# Get current chain ID
CHAIN_ID=$(linera wallet show | grep "Default chain" | awk '{print $3}' || echo "")
if [ -z "$CHAIN_ID" ]; then
    echo -e "${RED}❌ Error: No default chain found. Please initialize your wallet and chain first.${NC}"
    echo "Run: linera wallet init --with-new-chain"
    exit 1
fi

echo "  Default chain ID: $CHAIN_ID"

# Check wallet balance (optional, for info)
echo "  Checking wallet status..."
linera wallet show

echo -e "${GREEN}✅ Wallet and chain verified${NC}"
echo ""

# Step 4: Deploy application to Conway testnet
echo -e "${YELLOW}🚀 Step 4: Deploying application to Conway testnet...${NC}"

# Deploy the application
echo "  Publishing application bytecode..."
BYTECODE_ID=$(linera publish-bytecode "$WASM_FILE")
if [ -z "$BYTECODE_ID" ]; then
    echo -e "${RED}❌ Error: Failed to publish bytecode${NC}"
    exit 1
fi

echo "  Bytecode ID: $BYTECODE_ID"

# Create the application (Lobby instance)
echo "  Creating Lobby application..."
APP_ID=$(linera create-application "$BYTECODE_ID" --json-argument '{"Lobby": null}' --json-parameters 'null')
if [ -z "$APP_ID" ]; then
    echo -e "${RED}❌ Error: Failed to create application${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Application deployed successfully${NC}"
echo "  Application ID: $APP_ID"
echo ""

# Step 5: Update frontend configuration
echo -e "${YELLOW}📝 Step 5: Updating frontend configuration...${NC}"

# Backup original file
if [ -f "$FRONTEND_LINERA_FILE" ]; then
    cp "$FRONTEND_LINERA_FILE" "$FRONTEND_LINERA_FILE.backup"
    echo "  Backup created: $FRONTEND_LINERA_FILE.backup"
fi

# Update the LOBBY_APP_ID in the frontend configuration
echo "  Updating LOBBY_APP_ID in $FRONTEND_LINERA_FILE..."

# Use sed to replace the LOBBY_APP_ID value
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS sed syntax
    sed -i '' "s/LOBBY_APP_ID: '[^']*'/LOBBY_APP_ID: '$APP_ID'/" "$FRONTEND_LINERA_FILE"
else
    # Linux sed syntax
    sed -i "s/LOBBY_APP_ID: '[^']*'/LOBBY_APP_ID: '$APP_ID'/" "$FRONTEND_LINERA_FILE"
fi

echo -e "${GREEN}✅ Frontend configuration updated${NC}"
echo ""

# Step 6: Deployment summary
echo -e "${GREEN}🎉 Deployment Complete!${NC}"
echo "================================"
echo ""
echo -e "${BLUE}📊 Deployment Summary:${NC}"
echo "  Bytecode ID:     $BYTECODE_ID"
echo "  Application ID:  $APP_ID"
echo "  Chain ID:        $CHAIN_ID"
echo "  Network:         Conway Testnet"
echo ""
echo -e "${BLUE}📋 Application Details:${NC}"
echo "  Lobby App ID:    $APP_ID (main chain)"
echo "  DraftRoom ID:    $APP_ID (microchains created by lobby)"
echo "  Contract Type:   Unified (Lobby + DraftRoom)"
echo ""
echo -e "${BLUE}🔧 Frontend Integration:${NC}"
echo "  Config File:     $FRONTEND_LINERA_FILE"
echo "  Status:          Updated with deployed App ID"
echo ""
echo -e "${BLUE}🚀 Next Steps:${NC}"
echo "  1. Build frontend: cd frontend && npm run build"
echo "  2. Test lobby:     Create rooms via frontend"
echo "  3. Test drafting:  Join rooms and start drafts"
echo ""
echo -e "${YELLOW}💡 Usage Notes:${NC}"
echo "  • The same App ID is used for both Lobby and DraftRoom"
echo "  • Lobby runs on the main chain ($CHAIN_ID)"
echo "  • DraftRooms run on microchains created by the lobby"
echo "  • Each room gets its own microchain for isolated state"
echo ""
echo -e "${GREEN}✅ LiveDraft Arena is now live on Conway testnet!${NC}"

# Save deployment info to file
DEPLOY_INFO_FILE="deployment_info.txt"
cat > "$DEPLOY_INFO_FILE" << EOF
LiveDraft Arena - Conway Testnet Deployment
==========================================
Deployment Date: $(date)
Bytecode ID: $BYTECODE_ID
Application ID: $APP_ID
Chain ID: $CHAIN_ID
Network: Conway Testnet

Frontend Configuration:
- File: $FRONTEND_LINERA_FILE
- LOBBY_APP_ID: $APP_ID

Usage:
- Lobby runs on chain: $CHAIN_ID
- DraftRooms run on microchains created by lobby operations
- Frontend connects to: https://conway-testnet.linera.net:8080/graphql
EOF

echo "📄 Deployment info saved to: $DEPLOY_INFO_FILE"