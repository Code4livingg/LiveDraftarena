# LiveDraft Arena Service

Production Linera backend service for LiveDraft Arena with real Conway testnet integration.

## Features

- **Real Linera Integration**: Executes actual operations on Conway testnet
- **Multi-user Identity**: Deterministic player IDs mapped to Linera Owner addresses
- **GraphQL API**: Complete schema for lobby and draft room operations
- **Stateless Architecture**: No database required, all state on-chain
- **Concurrent Users**: Multiple players can interact simultaneously

## Architecture

```
Frontend (React) 
    ↓ HTTP + GraphQL
Backend Service (Rust + async-graphql)
    ↓ Linera Client API  
Conway Testnet (Linera Network)
    ↓ Smart Contracts
Lobby Chain + DraftRoom Microchains
```

## Linera Execution Flow

### Operations (Mutations)

1. **CreateRoom**: 
   - Executes on Lobby chain (default_chain_id)
   - Opens new microchain for DraftRoom
   - Stores metadata in Lobby MapView

2. **JoinRoom**: 
   - Executes on DraftRoom microchain
   - Validates room capacity and status
   - Adds player to room state

3. **StartDraft**: 
   - Executes on DraftRoom microchain  
   - Creator-only operation
   - Initializes Wave-5 card pool

4. **PickItem**: 
   - Executes on DraftRoom microchain
   - Validates player turn in snake draft
   - Updates pool and player picks

5. **FinalizeDraft**: 
   - Executes on DraftRoom microchain
   - Marks draft as complete

### State Queries

1. **rooms()**: Queries Lobby chain MapView<ChainId, DraftRoomMetadata>
2. **roomState()**: Queries DraftRoom microchain state
3. **myPicks()**: Extracts player picks from DraftRoom MapView<Owner, Vec<DraftItem>>

## Prerequisites

- Rust 1.70+
- Linera CLI installed and initialized
- Conway testnet wallet with chains
- Deployed LiveDraft Arena application

## Setup

### 1. Initialize Linera Wallet

```bash
# Initialize wallet (if not done)
linera wallet init

# Verify connection to Conway testnet
linera query-validators
```

### 2. Deploy Application

```bash
# Build and deploy contracts
cd scripts
./deploy_conway.sh

# This will output the Application ID - save it for the service
```

### 3. Configure Environment

```bash
# Required: Application ID from deployment
export LIVEDRAFT_APP_ID="your_deployed_app_id_here"

# Optional overrides
export LINERA_WALLET_PATH="/path/to/wallet.json"  # defaults to ~/.config/linera/wallet.json
export LIVEDRAFT_CHAIN_ID="your_lobby_chain_id"   # defaults to wallet default chain
export PORT="8080"                                # defaults to 8080
```

### 4. Run Service

```bash
cd service

# Development mode with logs
RUST_LOG=info cargo run

# Production mode
cargo run --release
```

## API Endpoints

- **GraphQL**: `http://localhost:8080/graphql`
- **Playground**: `http://localhost:8080/playground` 
- **Health**: `http://localhost:8080/health`

## Multi-User Identity System

### Player Identity Mapping

Each browser session gets a deterministic Linera Owner:

```rust
// 1. Generate/retrieve 16-char hex player ID
let player_id = "a1b2c3d4e5f67890";

// 2. Create deterministic Owner address
let owner = sha256("livedraft_player_" + player_id);

// 3. Use Owner for all contract operations
client.execute_operation(chain_id, app_id, operation).await
```

### Session Persistence

- **localStorage**: Player ID persists across browser refreshes
- **HTTP Headers**: `x-player-id` sent with each request
- **Cookies**: Backup identity mechanism
- **Multi-Browser**: Each browser gets unique identity

## Error Handling

All Linera execution errors bubble up to GraphQL:

```json
{
  "data": {
    "createRoom": {
      "success": false,
      "message": "Room name cannot be empty",
      "transactionHash": null
    }
  }
}
```

Contract panics become GraphQL errors with full error context.

## Development Status

**✅ Implemented**: 
- Real Linera client integration
- All mutation operations
- State query framework
- Multi-user identity system
- Error handling and logging

## Real On-Chain State Queries

The service now implements **real Linera view deserialization** for all GraphQL queries:

### Query Implementation

**`rooms()`** - Lobby Chain Query:
- Queries `LiveDraftArena::Lobby` state on default chain
- Deserializes `MapView<ChainId, DraftRoomMetadata>` from response
- Handles multiple serialization formats (JSON, bincode, string-encoded)
- Returns all created rooms with metadata

**`roomState(chainId)`** - DraftRoom Microchain Query:
- Queries `LiveDraftArena::DraftRoom` state on specified microchain
- Deserializes complete room state (players, pool, turn order, status)
- Extracts `Vec<Owner>`, `Vec<DraftItem>`, turn counters, and draft status
- Returns comprehensive room information

**`myPicks(chainId)`** - Player Picks Query:
- Queries DraftRoom state and extracts `MapView<Owner, Vec<DraftItem>>`
- Filters picks for current player's Owner address
- Handles different MapView serialization formats (object/array)
- Returns only the current player's drafted cards

### Deserialization Strategies

The service uses **multiple deserialization strategies** to handle Linera's various response formats:

1. **JSON Deserialization** (Primary):
   - Handles `LiveDraftArena` enum variants
   - Supports nested state structures
   - Parses MapView as JSON objects or arrays

2. **Bincode Deserialization** (Fallback):
   - Direct binary deserialization of contract types
   - Limited by MapView storage context requirements

3. **String-Encoded JSON** (Alternative):
   - Handles cases where Linera returns JSON as strings
   - Additional parsing layer for compatibility

### Error Handling

- **Graceful Degradation**: Missing state returns empty results, not errors
- **Clear Error Messages**: Deserialization failures surface with context
- **Multiple Attempts**: Tries different formats before failing
- **Detailed Logging**: Comprehensive tracing for debugging

### View Access Patterns

**MapView Handling**:
```rust
// Lobby rooms: MapView<ChainId, DraftRoomMetadata>
// Serialized as: {"chain_id_string": {"room_name": "...", ...}}

// DraftRoom picks: MapView<Owner, Vec<DraftItem>>  
// Serialized as: {"owner_address_string": [{"id": 1, "name": "..."}, ...]}
```

**State Extraction**:
- Direct field access from deserialized JSON
- Type conversion between contract and service types
- Owner address string parsing and matching

The implementation provides **real-time on-chain state access** without caching or mocking.

## Testing

### Single User
```bash
# Start service
cargo run

# Open GraphQL playground
open http://localhost:8080/playground

# Test operations
mutation { createRoom(input: {roomName: "Test", maxPlayers: 4}) { success message } }
```

### Multi User
```bash
# Open multiple browsers
# Each gets unique player ID
# Test concurrent operations
```

## Troubleshooting

### Wallet Issues
```bash
# Check wallet status
linera wallet show

# Verify chains
linera query-balance

# Test connectivity
linera query-validators
```

### Application Issues
```bash
# Verify deployment
linera query-application $LIVEDRAFT_APP_ID

# Check logs
RUST_LOG=debug cargo run
```

## Production Deployment

The service is designed for production Conway testnet deployment with configurable networking and security.

### Production Configuration

**Environment Variables:**

```bash
# Required
export LIVEDRAFT_APP_ID="your_deployed_app_id_here"

# Production networking (bind to all interfaces for public access)
export BIND_ADDRESS="0.0.0.0"                    # defaults to 0.0.0.0
export PORT="8080"                               # defaults to 8080

# Production CORS (restrict to your frontend domain)
export CORS_ORIGINS="https://your-frontend-domain.com,https://www.your-frontend-domain.com"

# Production logging (info level for performance)
export RUST_LOG="info"                          # defaults to info in production

# Optional overrides
export LINERA_WALLET_PATH="/path/to/wallet.json"  # defaults to ~/.config/linera/wallet.json
export LIVEDRAFT_CHAIN_ID="your_lobby_chain_id"   # defaults to wallet default chain
```

**Deployment Modes:**

```bash
# Development mode (open CORS, debug logging)
DEPLOYMENT_MODE=development ./start.sh

# Production mode (restricted CORS, optimized binary)
DEPLOYMENT_MODE=production ./start.sh
```

### Production Security Features

- **Network Binding**: Binds to `0.0.0.0` for public VPS/cloud deployment
- **CORS Protection**: Configurable origins to restrict frontend access
- **Structured Logging**: Production-optimized log format and levels
- **Error Handling**: Graceful error responses without sensitive information
- **Stateless Architecture**: No local database required, horizontally scalable

### VPS/Cloud Deployment

**1. Server Setup:**
```bash
# Install Rust and Linera CLI on your server
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install Linera CLI following official docs

# Initialize wallet on server
linera wallet init
```

**2. Deploy Application:**
```bash
# Clone repository
git clone your-repo
cd livedraft-arena

# Deploy contracts to Conway testnet
./scripts/deploy_conway.sh
```

**3. Configure Production Environment:**
```bash
# Create production .env file
cat > service/.env << EOF
LIVEDRAFT_APP_ID=your_deployed_app_id
BIND_ADDRESS=0.0.0.0
PORT=8080
CORS_ORIGINS=https://your-frontend-domain.com
RUST_LOG=info
DEPLOYMENT_MODE=production
EOF
```

**4. Start Production Service:**
```bash
cd service
DEPLOYMENT_MODE=production ./start.sh
```

**5. Verify Deployment:**
```bash
# Health check
curl http://your-server-ip:8080/health

# GraphQL endpoint
curl -X POST http://your-server-ip:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "query { rooms { chainId roomName } }"}'
```

### Production Monitoring

**Health Checks:**
- `GET /health` - Service health status
- Structured JSON logging for monitoring tools
- Graceful error handling with proper HTTP status codes

**Scaling:**
- Multiple service instances can run concurrently
- All state stored on Linera network (stateless backend)
- Load balancer compatible (no session affinity required)

### Production Troubleshooting

**Network Issues:**
```bash
# Verify server is binding correctly
netstat -tlnp | grep :8080

# Test CORS configuration
curl -H "Origin: https://your-frontend-domain.com" \
     -H "Access-Control-Request-Method: POST" \
     -X OPTIONS http://your-server-ip:8080/graphql
```

**Linera Connectivity:**
```bash
# Test Conway testnet connection
linera query-validators

# Verify wallet access
linera wallet show
```