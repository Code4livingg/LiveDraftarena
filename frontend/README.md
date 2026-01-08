# LiveDraft Arena Frontend

React + TypeScript frontend for LiveDraft Arena with real backend GraphQL integration.

## Development

```bash
# Install dependencies
npm install

# Start development server (requires backend service running)
npm run dev
```

## Backend Integration

This frontend connects to the real Linera backend service at `http://localhost:8080/graphql`.

### ✅ Features
- **Real GraphQL queries** to backend service
- **Real mutations** for all operations
- **Real-time polling** (1-2 second intervals)
- **No @linera/client** in browser
- **No mocked data** - all from backend

### 🔧 Architecture
```
Frontend (React) → GraphQL HTTP → Backend Service → Linera Conway Testnet
```

### 📡 GraphQL Operations

**Queries:**
- `rooms` - Get all draft rooms from lobby
- `roomState(chainId)` - Get specific room state
- `health` - Backend health check

**Mutations:**
- `createRoom(input)` - Create new room
- `joinRoom(chainId)` - Join existing room
- `startDraft(chainId)` - Start draft (creator only)
- `pickItem(chainId, input)` - Pick item during draft

### 🚀 Usage Flow
1. **Connect**: Backend health check + wallet simulation
2. **Lobby**: Real GraphQL queries for rooms, real createRoom mutations
3. **Draft Room**: Real-time room state, real operations via GraphQL

### 🔗 Backend Dependency
The frontend requires the backend service to be running:
```bash
# In service/ directory
export LIVEDRAFT_APP_ID=your_app_id
./start.sh
```

No more direct blockchain calls - everything goes through the secure backend service!