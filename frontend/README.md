# LiveDraft Arena Frontend

React + TypeScript frontend for LiveDraft Arena with real Linera blockchain integration.

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev
```

## Real Linera Integration

This frontend now uses **real @linera/client integration** with:

### ✅ Features Implemented
- **Real wallet connection** via @linera/client signer
- **Real GraphQL queries** to Conway testnet
- **Real operation submission** to smart contracts
- **Persistent chain selection** via localStorage
- **Real-time polling** (1-2 second intervals)

### 🔧 Configuration
- **Conway Testnet Endpoint**: `https://conway-testnet.linera.net:8080/graphql`
- **Lobby App ID**: `e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65`
- **Signer Persistence**: Private keys stored in localStorage

### 📁 Architecture
```
src/
├── hooks/
│   ├── useLinera.ts        # Client & signer management
│   ├── useGraphQLQuery.ts  # Real GraphQL polling
│   └── useOperation.ts     # Operation execution
├── components/
│   ├── WalletConnect.tsx   # Real wallet connection
│   ├── LobbyPage.tsx       # Real lobby queries & operations
│   └── DraftRoomPage.tsx   # Real draft room state & actions
└── linera.ts              # Core Linera integration
```

### 🚀 Usage Flow
1. **Connect**: Creates/loads signer, selects chain ID
2. **Lobby**: Real GraphQL queries for rooms, real CreateRoom operations
3. **Draft Room**: Real-time room state, real JoinRoom/StartDraft/PickItem operations

### 🔗 Integration Points
- **GraphQL Queries**: Direct to Conway testnet GraphQL endpoint
- **Operations**: Signed and submitted via @linera/client
- **State Management**: Real blockchain state with polling
- **Error Handling**: Real network and contract errors

No more mock data - this is a fully functional Linera dApp frontend!