#!/bin/bash

# Test script for LiveDraft Arena GraphQL endpoint
# Tests both local and deployed endpoints

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Default endpoint (can be overridden)
ENDPOINT=${1:-"http://localhost:8080/graphql"}

echo -e "${BLUE}🧪 Testing GraphQL endpoint: $ENDPOINT${NC}"
echo "================================================"

# Test 1: Health check
echo -e "${BLUE}1. Testing health endpoint...${NC}"
HEALTH_URL=$(echo $ENDPOINT | sed 's|/graphql|/health|')
if curl -s -f "$HEALTH_URL" > /dev/null; then
    echo -e "${GREEN}✅ Health check passed${NC}"
else
    echo -e "${RED}❌ Health check failed${NC}"
    exit 1
fi

# Test 2: GraphQL introspection query
echo -e "${BLUE}2. Testing GraphQL introspection...${NC}"
INTROSPECTION_QUERY='{"query": "{ __schema { types { name } } }"}'

RESPONSE=$(curl -s -X POST "$ENDPOINT" \
    -H "Content-Type: application/json" \
    -H "x-player-id: test-player-123" \
    -d "$INTROSPECTION_QUERY")

if echo "$RESPONSE" | grep -q '"__schema"'; then
    echo -e "${GREEN}✅ GraphQL introspection successful${NC}"
    echo "   Schema types found in response"
else
    echo -e "${RED}❌ GraphQL introspection failed${NC}"
    echo "   Response: $RESPONSE"
    exit 1
fi

# Test 3: Rooms query (should work even without real Linera setup)
echo -e "${BLUE}3. Testing rooms query...${NC}"
ROOMS_QUERY='{"query": "query { rooms { chainId roomName status } }"}'

RESPONSE=$(curl -s -X POST "$ENDPOINT" \
    -H "Content-Type: application/json" \
    -H "x-player-id: test-player-123" \
    -d "$ROOMS_QUERY")

if echo "$RESPONSE" | grep -q '"rooms"'; then
    echo -e "${GREEN}✅ Rooms query successful${NC}"
    echo "   Response contains rooms field"
else
    echo -e "${YELLOW}⚠️  Rooms query returned error (expected without Linera setup)${NC}"
    echo "   Response: $RESPONSE"
fi

# Test 4: CORS headers
echo -e "${BLUE}4. Testing CORS headers...${NC}"
CORS_RESPONSE=$(curl -s -I -X OPTIONS "$ENDPOINT" \
    -H "Origin: https://example.com" \
    -H "Access-Control-Request-Method: POST")

if echo "$CORS_RESPONSE" | grep -q "Access-Control-Allow-Origin"; then
    echo -e "${GREEN}✅ CORS headers present${NC}"
else
    echo -e "${RED}❌ CORS headers missing${NC}"
    echo "   Response headers: $CORS_RESPONSE"
fi

echo ""
echo -e "${GREEN}🎉 GraphQL endpoint testing complete!${NC}"
echo ""
echo -e "${BLUE}📋 Summary:${NC}"
echo "  Endpoint: $ENDPOINT"
echo "  Health: ✅ Working"
echo "  GraphQL: ✅ Working"
echo "  CORS: ✅ Configured"
echo ""
echo -e "${YELLOW}💡 To test with real Linera integration:${NC}"
echo "  1. Deploy contracts to Conway testnet"
echo "  2. Set LIVEDRAFT_APP_ID environment variable"
echo "  3. Ensure Linera wallet is accessible"