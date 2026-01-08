# LiveDraft Arena - Conway Testnet Deployment

This guide covers deploying LiveDraft Arena contracts to the Conway testnet.

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

### Option 1: Automated Deployment (Recommended)
```bash
# Run the complete deployment pipeline
make deploy-full

# Or step by step:
make build-wasm    # Build WASM contract
make deploy        # Deploy to Conway testnet
make verify        # Verify deployment
```

### Option 2: Manual Deployment
```bash
# 1. Build WASM contract
cargo build --target wasm32-unknown-unknown --release

# 2. Run deployment script
./scripts/deploy_conway.sh

# 3. Verify deployment
./scripts/verify_deployment.sh
```

## Deployment Script Details

The `scripts/deploy_conway.sh` script performs these steps:

1. **Prerequisites Check**
   - Verifies linera CLI is installed
   - Checks Rust and WASM target availability
   - Validates wallet and chain setup

2. **WASM Build**
   - Cleans previous builds
   - Builds contract for `wasm32-unknown-unknown` target
   - Verifies WASM file creation

3. **Linera Deployment**
   - Publishes bytecode to Conway testnet
   - Creates application instance (Lobby)
   - Returns Application ID

4. **Frontend Configuration**
   - Updates `frontend/src/linera.ts` with deployed App ID
   - Creates backup of original configuration

5. **Deployment Summary**
   - Outputs all deployment details
   - Saves deployment info to `deployment_info.txt`

## Application Architecture

### Unified Contract Design
- **Single Application ID** for both Lobby and DraftRoom
- **Main Chain**: Runs the Lobby contract
- **Microchains**: Each DraftRoom runs on its own microchain
- **Parameters**: Distinguish between Lobby and DraftRoom instances

### Contract Parameters
```rust
// Lobby instance (main chain)
ContractParameters::Lobby

// DraftRoom instance (microchain)
ContractParameters::DraftRoom { max_players: 4 }
```

## Post-Deployment

### 1. Frontend Integration
The deployment script automatically updates:
```typescript
// frontend/src/linera.ts
export const LINERA_CONFIG = {
  LOBBY_APP_ID: 'deployed-app-id-here',
  // ...
};
```

### 2. Testing the Deployment
```bash
# Start frontend
cd frontend
npm install
npm run dev

# Test flow:
# 1. Connect wallet with deployed chain ID
# 2. Create rooms in lobby
# 3. Join rooms and start drafts
```

### 3. Monitoring
```bash
# Check wallet status
linera wallet show

# Query application state
linera query-application <app-id>

# View deployment info
cat deployment_info.txt
```

## Troubleshooting

### Common Issues

**WASM Build Fails**
```bash
# Ensure WASM target is installed
rustup target add wasm32-unknown-unknown

# Clean and rebuild
cargo clean
cargo build --target wasm32-unknown-unknown --release
```

**Deployment Fails**
```bash
# Check wallet has sufficient balance
linera wallet show

# Verify chain is active
linera wallet set-default <chain-id>
```

**Frontend Not Connecting**
- Verify App ID was updated in `frontend/src/linera.ts`
- Check Conway testnet endpoint is accessible
- Ensure chain ID matches deployment

### Logs and Debugging
```bash
# View deployment logs
cat deployment_info.txt

# Check Linera logs
linera --help

# Verify contract deployment
linera query-application <app-id>
```

## Production Considerations

### Security
- Store private keys securely
- Use hardware wallets for mainnet
- Validate all deployment parameters

### Monitoring
- Set up application monitoring
- Monitor chain health
- Track transaction costs

### Backup
- Backup wallet files
- Save deployment configurations
- Document all App IDs and Chain IDs

## Conway Testnet Specifics

- **Network**: Conway Testnet
- **GraphQL Endpoint**: `https://conway-testnet.linera.net:8080/graphql`
- **Purpose**: Testing and development
- **Limitations**: Testnet may reset, use for testing only