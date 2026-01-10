#!/bin/bash

# LiveDraft Arena Service - Fly.io Deployment Script
# Deploys directly without Git repository

set -e

echo "🚀 Deploying LiveDraft Arena Service to Fly.io"
echo "=============================================="

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Check if flyctl is installed
if ! command -v flyctl &> /dev/null; then
    echo -e "${RED}❌ flyctl is not installed${NC}"
    echo "Install it from: https://fly.io/docs/hands-on/install-flyctl/"
    exit 1
fi

# Check if logged in to Fly.io
if ! flyctl auth whoami &> /dev/null; then
    echo -e "${YELLOW}⚠️  Not logged in to Fly.io${NC}"
    echo "Please run: flyctl auth login"
    exit 1
fi

echo -e "${BLUE}📋 Deployment Configuration:${NC}"
echo "  App Name: livedraft-arena-api"
echo "  Region: iad (Ashburn, VA)"
echo "  Port: 8080"
echo "  CORS: Allow all origins"
echo ""

# Set required environment variables
echo -e "${BLUE}🔧 Setting environment variables...${NC}"

# Check if LIVEDRAFT_APP_ID is set
if [ -z "$LIVEDRAFT_APP_ID" ]; then
    echo -e "${RED}❌ LIVEDRAFT_APP_ID environment variable is required${NC}"
    echo "Please set it with your deployed Linera application ID:"
    echo "  export LIVEDRAFT_APP_ID=your_app_id_here"
    exit 1
fi

# Set secrets in Fly.io
flyctl secrets set LIVEDRAFT_APP_ID="$LIVEDRAFT_APP_ID"

# Optional: Set chain ID if provided
if [ ! -z "$LIVEDRAFT_CHAIN_ID" ]; then
    echo -e "${GREEN}✅ Setting chain ID: $LIVEDRAFT_CHAIN_ID${NC}"
    flyctl secrets set LIVEDRAFT_CHAIN_ID="$LIVEDRAFT_CHAIN_ID"
fi

# Optional: Set wallet path if provided
if [ ! -z "$LINERA_WALLET_PATH" ]; then
    echo -e "${GREEN}✅ Setting wallet path: $LINERA_WALLET_PATH${NC}"
    flyctl secrets set LINERA_WALLET_PATH="$LINERA_WALLET_PATH"
fi

echo ""
echo -e "${BLUE}🐳 Building and deploying Docker image...${NC}"

# Deploy to Fly.io (this will build the Docker image and deploy)
flyctl deploy --local-only

echo ""
echo -e "${GREEN}✅ Deployment complete!${NC}"
echo ""
echo -e "${BLUE}📊 Service Information:${NC}"
echo "  GraphQL Endpoint: https://livedraft-arena-api.fly.dev/graphql"
echo "  GraphQL Playground: https://livedraft-arena-api.fly.dev/playground"
echo "  Health Check: https://livedraft-arena-api.fly.dev/health"
echo ""
echo -e "${BLUE}🔍 Useful Commands:${NC}"
echo "  Check status: flyctl status"
echo "  View logs: flyctl logs"
echo "  Scale app: flyctl scale count 2"
echo "  Open app: flyctl open"
echo ""
echo -e "${YELLOW}⚠️  Note: Make sure to update your frontend VITE_BACKEND_GRAPHQL_URL to:${NC}"
echo -e "${YELLOW}  https://livedraft-arena-api.fly.dev/graphql${NC}"