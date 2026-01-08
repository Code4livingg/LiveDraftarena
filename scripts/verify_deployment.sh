#!/bin/bash

# LiveDraft Arena - Deployment Verification Script
# This script verifies the deployed contracts are working correctly

set -e

echo "🔍 LiveDraft Arena - Deployment Verification"
echo "============================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check if deployment info exists
DEPLOY_INFO_FILE="deployment_info.txt"
if [ ! -f "$DEPLOY_INFO_FILE" ]; then
    echo -e "${RED}❌ Error: Deployment info file not found: $DEPLOY_INFO_FILE${NC}"
    echo "Please run deploy_conway.sh first"
    exit 1
fi

# Extract deployment info
APP_ID=$(grep "Application ID:" "$DEPLOY_INFO_FILE" | awk '{print $3}')
CHAIN_ID=$(grep "Chain ID:" "$DEPLOY_INFO_FILE" | awk '{print $3}')

echo -e "${BLUE}📋 Verifying Deployment:${NC}"
echo "  Application ID: $APP_ID"
echo "  Chain ID: $CHAIN_ID"
echo ""

# Step 1: Verify application exists
echo -e "${YELLOW}🔍 Step 1: Verifying application exists...${NC}"

# Check if application is registered
if linera wallet show | grep -q "$APP_ID"; then
    echo -e "${GREEN}✅ Application found in wallet${NC}"
else
    echo -e "${RED}❌ Application not found in wallet${NC}"
    exit 1
fi

# Step 2: Test GraphQL service
echo -e "${YELLOW}🔍 Step 2: Testing GraphQL service...${NC}"

# Try to query the lobby service
echo "  Testing lobby rooms query..."
GRAPHQL_QUERY='{"query": "{ rooms { chain_id metadata { room_name max_players status } } }"}'

# Note: This would require the GraphQL service to be running
# For now, we'll just show what the query would be
echo "  GraphQL Query: $GRAPHQL_QUERY"
echo "  Endpoint: https://conway-testnet.linera.net:8080/graphql"
echo -e "${YELLOW}⚠️  Manual verification required: Test GraphQL endpoint with frontend${NC}"

# Step 3: Verify frontend configuration
echo -e "${YELLOW}🔍 Step 3: Verifying frontend configuration...${NC}"

FRONTEND_FILE="frontend/src/linera.ts"
if [ -f "$FRONTEND_FILE" ]; then
    if grep -q "$APP_ID" "$FRONTEND_FILE"; then
        echo -e "${GREEN}✅ Frontend configuration updated correctly${NC}"
    else
        echo -e "${RED}❌ Frontend configuration not updated${NC}"
        exit 1
    fi
else
    echo -e "${RED}❌ Frontend configuration file not found${NC}"
    exit 1
fi

# Step 4: Show next steps
echo ""
echo -e "${GREEN}🎉 Verification Complete!${NC}"
echo ""
echo -e "${BLUE}🚀 Next Steps for Testing:${NC}"
echo "  1. Start frontend development server:"
echo "     cd frontend && npm run dev"
echo ""
echo "  2. Test the application:"
echo "     • Connect wallet with chain ID: $CHAIN_ID"
echo "     • Create a new room in the lobby"
echo "     • Join the room and start a draft"
echo ""
echo "  3. Monitor transactions:"
echo "     linera wallet show"
echo "     linera query-application $APP_ID"
echo ""
echo -e "${BLUE}📊 Deployment Summary:${NC}"
cat "$DEPLOY_INFO_FILE"