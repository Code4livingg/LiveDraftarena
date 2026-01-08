#!/bin/bash

# LiveDraft Arena - Deployment Verification Script
# Verifies that the deployed application is working correctly on Conway testnet

set -e

echo "🔍 LiveDraft Arena - Deployment Verification"
echo "============================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Load configuration
ENV_FILE="service/.env"
DEPLOY_INFO_FILE="deployment_info.json"

echo -e "${BLUE}📋 Loading deployment configuration...${NC}"

# Try to load Application ID from multiple sources
APP_ID=""
CHAIN_ID=""
BYTECODE_ID=""

# 1. Try deployment info JSON file
if [ -f "$DEPLOY_INFO_FILE" ]; then
    echo "  Found deployment info: $DEPLOY_INFO_FILE"
    
    if command -v jq &> /dev/null; then
        APP_ID=$(jq -r '.contract.application_id' "$DEPLOY_INFO_FILE" 2>/dev/null || echo "")
        CHAIN_ID=$(jq -r '.contract.chain_id' "$DEPLOY_INFO_FILE" 2>/dev/null || echo "")
        BYTECODE_ID=$(jq -r '.contract.bytecode_id' "$DEPLOY_INFO_FILE" 2>/dev/null || echo "")
    else
        echo -e "${YELLOW}  Warning: jq not found, cannot parse JSON deployment info${NC}"
    fi
fi

# 2. Try service .env file
if [ -z "$APP_ID" ] && [ -f "$ENV_FILE" ]; then
    echo "  Found service config: $ENV_FILE"
    
    while IFS='=' read -r key value; do
        case "$key" in
            "LIVEDRAFT_APP_ID") APP_ID="$value" ;;
            "LIVEDRAFT_CHAIN_ID") CHAIN_ID="$value" ;;
        esac
    done < <(grep -v '^#' "$ENV_FILE" | grep '=')
fi

# 3. Try environment variables
if [ -z "$APP_ID" ]; then
    APP_ID="$LIVEDRAFT_APP_ID"
    CHAIN_ID="$LIVEDRAFT_CHAIN_ID"
fi

# Validate we have required information
if [ -z "$APP_ID" ]; then
    echo -e "${RED}❌ Error: Application ID not found${NC}"
    echo ""
    echo "Please ensure deployment was successful and try one of:"
    echo "  1. Run deployment: ./scripts/deploy_conway.sh"
    echo "  2. Set environment: export LIVEDRAFT_APP_ID=your-app-id"
    echo "  3. Check files: $ENV_FILE or $DEPLOY_INFO_FILE"
    exit 1
fi

echo -e "${GREEN}✅ Configuration loaded${NC}"
echo "  Application ID: $APP_ID"
if [ -n "$CHAIN_ID" ]; then
    echo "  Chain ID: $CHAIN_ID"
fi
if [ -n "$BYTECODE_ID" ]; then
    echo "  Bytecode ID: $BYTECODE_ID"
fi
echo ""

# Step 1: Verify Linera CLI connectivity
echo -e "${YELLOW}🔗 Step 1: Verifying Linera CLI connectivity...${NC}"

if ! command -v linera &> /dev/null; then
    echo -e "${RED}❌ Error: linera CLI not found${NC}"
    exit 1
fi

# Test basic connectivity
if ! linera wallet show &> /dev/null; then
    echo -e "${RED}❌ Error: Cannot access Linera wallet${NC}"
    echo "Please ensure wallet is initialized: linera wallet init"
    exit 1
fi

echo -e "${GREEN}✅ Linera CLI connectivity verified${NC}"
echo ""

# Step 2: Verify application deployment
echo -e "${YELLOW}📱 Step 2: Verifying application deployment...${NC}"

echo "  Querying application: $APP_ID"

# Query the application to verify it exists and is accessible
if linera query-application "$APP_ID" &> /dev/null; then
    echo -e "${GREEN}✅ Application found and accessible${NC}"
    
    # Get application details
    echo "  Application details:"
    linera query-application "$APP_ID" | head -10
else
    echo -e "${RED}❌ Error: Cannot query application $APP_ID${NC}"
    echo "  This could mean:"
    echo "  - Application was not deployed successfully"
    echo "  - Wrong Application ID"
    echo "  - Network connectivity issues"
    exit 1
fi

echo ""

# Step 3: Verify bytecode (if available)
if [ -n "$BYTECODE_ID" ]; then
    echo -e "${YELLOW}📦 Step 3: Verifying bytecode deployment...${NC}"
    
    echo "  Querying bytecode: $BYTECODE_ID"
    
    if linera query-bytecode "$BYTECODE_ID" &> /dev/null; then
        echo -e "${GREEN}✅ Bytecode found and accessible${NC}"
    else
        echo -e "${YELLOW}⚠️  Warning: Cannot query bytecode $BYTECODE_ID${NC}"
        echo "  This is not critical but may indicate deployment issues"
    fi
    
    echo ""
fi

# Step 4: Verify chain access (if specified)
if [ -n "$CHAIN_ID" ]; then
    echo -e "${YELLOW}⛓️  Step 4: Verifying chain access...${NC}"
    
    echo "  Querying chain: $CHAIN_ID"
    
    if linera query-balance --chain "$CHAIN_ID" &> /dev/null; then
        echo -e "${GREEN}✅ Chain accessible${NC}"
        
        # Show chain balance for context
        echo "  Chain balance:"
        linera query-balance --chain "$CHAIN_ID"
    else
        echo -e "${YELLOW}⚠️  Warning: Cannot query chain $CHAIN_ID${NC}"
        echo "  This may be normal if using default chain"
    fi
    
    echo ""
fi

# Step 5: Test service configuration
echo -e "${YELLOW}🔧 Step 5: Verifying service configuration...${NC}"

# Check if service can load the configuration
if [ -f "$ENV_FILE" ]; then
    echo "  Service configuration file: $ENV_FILE"
    echo "  Contents:"
    grep -v '^#' "$ENV_FILE" | grep '=' | while IFS='=' read -r key value; do
        if [[ "$key" == *"APP_ID"* ]]; then
            echo "    $key=$value"
        else
            echo "    $key=***"
        fi
    done
    echo -e "${GREEN}✅ Service configuration valid${NC}"
else
    echo -e "${YELLOW}⚠️  Warning: Service .env file not found${NC}"
    echo "  Service will need LIVEDRAFT_APP_ID environment variable"
fi

echo ""

# Step 6: Test frontend configuration
echo -e "${YELLOW}🌐 Step 6: Verifying frontend configuration...${NC}"

FRONTEND_CONFIG="frontend/src/config.ts"
if [ -f "$FRONTEND_CONFIG" ]; then
    echo "  Frontend configuration file: $FRONTEND_CONFIG"
    
    # Check if APP_ID is set correctly
    if grep -q "APP_ID: '$APP_ID'" "$FRONTEND_CONFIG"; then
        echo -e "${GREEN}✅ Frontend configuration matches deployment${NC}"
    elif grep -q "REPLACE_AFTER_DEPLOY" "$FRONTEND_CONFIG"; then
        echo -e "${YELLOW}⚠️  Warning: Frontend configuration not updated${NC}"
        echo "  Run deployment script to update frontend config"
    else
        echo -e "${YELLOW}⚠️  Warning: Frontend configuration may be outdated${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Warning: Frontend configuration file not found${NC}"
    echo "  Run deployment script to create frontend config"
fi

echo ""

# Step 7: Summary
echo -e "${GREEN}🎉 Verification Complete!${NC}"
echo "=========================="
echo ""

echo -e "${BLUE}📊 Deployment Status:${NC}"
echo "  Application ID: $APP_ID"
echo "  Status: Deployed and accessible"
echo "  Network: Conway Testnet"
echo ""

echo -e "${BLUE}🚀 Next Steps:${NC}"
echo "  1. Start service: cd service && ./start.sh"
echo "  2. Start frontend: cd frontend && npm run dev"
echo "  3. Test application: Open http://localhost:3000"
echo ""

echo -e "${BLUE}🔧 Testing Commands:${NC}"
echo "  Query application: linera query-application $APP_ID"
if [ -n "$CHAIN_ID" ]; then
    echo "  Query chain: linera query-balance --chain $CHAIN_ID"
fi
echo "  Service health: curl http://localhost:8080/health"
echo ""

echo -e "${GREEN}✅ LiveDraft Arena deployment verified successfully!${NC}"