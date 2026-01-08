# LiveDraft Arena - Conway Testnet Deployment

This guide covers deploying LiveDraft Arena contracts to the Conway testnet with proper Application ID management.

## Prerequisites

### 1. Install Linera CLI
```bash
# Install Linera CLI (follow official Linera documentation)
# Ensure linera command is available in PATH
linera --version
```

### 2. Setup Wallet and Chain
```bash
# Initialize wallet with new chain
linera wallet init --with-new-chain

# Or connect to existing chain
linera wallet init
linera wallet set-default <chain-id>
```

### 3. Install Rust WASM Target
```bash
rustup target add wasm32-unknown-unknown
```

## Deployment Process

### Automated Deployment (Recommended)
```bash
# Run the complete deployment pipeline
./scripts/deploy_conway.sh

# Verify deployment
./scripts/verify_deployment.sh
```

The deployment script will:
1. Build WASM contract
2. Deploy to Conway testnet with `ContractParameters::Lobby`
3. Create service configuration (`service/.env`)
4. Create frontend configuration (`frontend/src/config.ts`)
5. Generate deployment summary (`deployment_info.json`)

## Application ID Management

### Single Source of Truth
The deployment creates a **single Application ID** used by all components:

```
Deployment Script
    ↓
service/.env (LIVEDRAFT_APP_ID=...)
    ↓
Backend Service (reads from .env)

    ↓
frontend/src/config.ts (APP_ID: '...')
    ↓
Frontend (imports from config)
```

### Configuration Files Created

**Service Configuration** (`service/.env`):
```bash
# LiveDraft Arena Service Configuration
LIVEDRAFT_APP_ID=deployed-app-id-here
# LINERA_WALLET_PATH=/path/to/wallet.json
# LIVEDRAFT_CHAIN_ID=chain-id-here
# PORT=8080
```

**Frontend Configuration** (`frontend/src/config.ts`):
```typescript
export const LIVEDRAFT_CONFIG = {
  APP_ID: 'deployed-app-id-here',
  GRAPHQL_ENDPOINT: 'http://localhost:8080/graphql',
  DEPLOYMENT: {
    BYTECODE_ID: 'bytecode-id-here',
    CHAIN_ID: 'chain-id-here',
    DEPLOYED_AT: '2024-01-08T12:00:00Z',
    NETWORK: 'Conway Testnet'
  }
} as const;
```

## Application Architecture

### Unified Contract Design
- **Single Application ID** for both Lobby and DraftRoom
- **Main Chain**: Runs the Lobby contract (`ContractParameters::Lobby`)
- **Microchains**: Each DraftRoom runs on its own microchain (`ContractParameters::DraftRoom { max_players }`)

### Contract Parameters Used
```rust
// Lobby instance (created by deployment script)
ContractParameters::Lobby

// DraftRoom instances (created by Lobby operations)
ContractParameters::DraftRoom { max_players: 4 }
```

## Post-Deployment

### 1. Start Backend Service
```bash
cd service
./start.sh  # Automatically loads .env configuration
```

The service will:
- Load `LIVEDRAFT_APP_ID` from `.env` file
- Connect to Conway testnet
- Serve GraphQL API on `http://localhost:8080/graphql`

### 2. Start Frontend
```bash
cd frontend
npm install
npm run dev
```

The frontend will:
- Import `APP_ID` from `config.ts`
- Connect to backend service
- Serve UI on `http://localhost:3000`

### 3. Test the Deployment
1. Open `http://localhost:3000`
2. Connect wallet (will use deployed chain)
3. Create rooms in lobby
4. Join rooms and start drafts

## Environment Variables

### Service Environment Variables
```bash
# Required (set by deployment script)
LIVEDRAFT_APP_ID=your-deployed-app-id

# Optional overrides
LINERA_WALLET_PATH=/path/to/wallet.json
LIVEDRAFT_CHAIN_ID=your-chain-id
PORT=8080
```

### Manual Configuration
If needed, you can manually set the Application ID:

```bash
# Set environment variable
export LIVEDRAFT_APP_ID=your-app-id

# Or create service/.env file
echo "LIVEDRAFT_APP_ID=your-app-id" > service/.env

# Update frontend config
# Edit frontend/src/config.ts and set APP_ID
```

## Verification

### Automated Verification
```bash
./scripts/verify_deployment.sh
```

This checks:
- Linera CLI connectivity
- Application deployment status
- Service configuration
- Frontend configuration
- Chain accessibility

### Manual Verification
```bash
# Check application exists
linera query-application $LIVEDRAFT_APP_ID

# Check service health
curl http://localhost:8080/health

# Check frontend config
grep APP_ID frontend/src/config.ts
```

## Troubleshooting

### Application ID Issues
```bash
# Check deployment info
cat deployment_info.json

# Check service config
cat service/.env

# Check frontend config
cat frontend/src/config.ts
```

### Service Won't Start
```bash
# Check if App ID is set
echo $LIVEDRAFT_APP_ID

# Check .env file exists
ls -la service/.env

# Check wallet access
linera wallet show
```

### Frontend Not Connecting
```bash
# Verify config file exists
ls -la frontend/src/config.ts

# Check if App ID is set correctly
grep -n "REPLACE_AFTER_DEPLOY" frontend/src/config.ts

# If found, re-run deployment script
./scripts/deploy_conway.sh
```

## Production Considerations

### Security
- Store private keys securely
- Use hardware wallets for mainnet
- Validate all deployment parameters
- Never commit `.env` files with real App IDs

### Monitoring
- Monitor application state: `linera query-application $APP_ID`
- Monitor chain health: `linera query-balance`
- Monitor service health: `curl http://localhost:8080/health`

### Backup
- Backup wallet files
- Save deployment configurations
- Document all App IDs and Chain IDs
- Keep `deployment_info.json` safe

## Conway Testnet Specifics

- **Network**: Conway Testnet
- **GraphQL Endpoint**: `https://conway-testnet.linera.net:8080/graphql`
- **Purpose**: Testing and development
- **Limitations**: Testnet may reset, use for testing only

## File Structure After Deployment

```
LiveDraftArena/
├── deployment_info.json          # Deployment summary
├── service/
│   ├── .env                      # Service configuration
│   └── start.sh                  # Service startup script
├── frontend/
│   └── src/
│       └── config.ts             # Frontend configuration
└── scripts/
    ├── deploy_conway.sh          # Deployment script
    └── verify_deployment.sh      # Verification script
```

All components use the same deployed Application ID for consistency.