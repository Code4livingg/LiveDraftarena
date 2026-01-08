# LiveDraft Arena Service

Backend service for LiveDraft Arena that connects to Conway testnet.

## Development

```bash
# Set required environment variables
export LIVEDRAFT_APP_ID=your_deployed_app_id

# Start service
./start.sh

# Or run directly
cargo run
```

## Configuration

The service requires:
- **LIVEDRAFT_APP_ID**: Deployed application ID from Conway testnet
- **Linera wallet**: Initialized wallet at `~/.config/linera/wallet.json`

Optional configuration:
- **LIVEDRAFT_CHAIN_ID**: Specific chain ID (uses wallet default if not set)
- **PORT**: Server port (default: 8080)
- **RUST_LOG**: Log level (default: info)

## Endpoints

- **GraphQL**: `http://localhost:8080/graphql`
- **Playground**: `http://localhost:8080/playground`
- **Health**: `http://localhost:8080/health`

## GraphQL Schema

### Queries
- `rooms`: Get all draft rooms from lobby
- `roomState(chainId: String)`: Get specific room state
- `myPicks(chainId: String, playerAddress: String)`: Get player's picks

### Mutations
- `createRoom(input: CreateRoomInput)`: Create new room
- `joinRoom(chainId: String)`: Join existing room
- `startDraft(chainId: String)`: Start draft (creator only)
- `pickItem(chainId: String, input: PickItemInput)`: Pick item during draft
- `finalizeDraft(chainId: String)`: Finalize completed draft