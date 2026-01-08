# LiveDraft Arena

LiveDraft Arena is a real-time collaborative drafting platform where players create rooms and draft cards in turns using a snake draft pattern. Each draft room operates on its own dedicated microchain, enabling isolated state management and concurrent drafting sessions without interference. The lobby coordinates room creation and discovery on the main chain, while individual draft sessions benefit from Linera's microchain architecture for optimal performance and scalability.

## Why Linera

This application leverages Linera's unique microchain architecture in ways impossible on traditional blockchains:

- **Isolated State**: Each draft room runs on its own microchain, preventing state conflicts between concurrent sessions
- **Deterministic Turn Order**: Snake draft logic executes reliably without gas concerns or block timing issues  
- **Real-time Updates**: GraphQL services provide instant state synchronization across all participants
- **Scalable Concurrency**: Unlimited draft rooms can operate simultaneously without network congestion
- **Cross-chain Coordination**: Lobby seamlessly creates and manages microchains for room instances

## Architecture

```
Main Chain (Lobby)
├── Room Discovery & Creation
├── Metadata Storage (room_name, max_players, status)
└── Microchain Management

Microchains (DraftRooms)
├── Player Management (join/leave)
├── Turn-based Drafting Logic
├── Snake Draft Implementation
└── Card Pool & Pick Tracking
```

**Contract Design**: Unified application with parameter-based routing (`ContractParameters::Lobby` vs `ContractParameters::DraftRoom`) enables single deployment with multi-instance functionality.

**State Management**: Lobby maintains room metadata via `MapView<ChainId, DraftRoomMetadata>` while DraftRooms manage isolated game state including player lists, turn tracking, and card selections.

## How to Run (Conway Testnet)

1. **Access**: Navigate to deployed frontend URL
2. **Connect**: Link Linera wallet and select Conway testnet chain
3. **Create Room**: Specify room name and max players (2-8)
4. **Join & Draft**: Enter room, wait for players, start draft, pick cards in snake order

**Technical Requirements**: Linera wallet with Conway testnet access, modern browser with JavaScript enabled.

## Wave-5 Scope

**Implemented Features**:
- ✅ Lobby contract with room creation and discovery
- ✅ DraftRoom contract with turn-based snake drafting
- ✅ React frontend with real-time GraphQL polling
- ✅ Conway testnet deployment with production scripts
- ✅ Hardcoded Wave-5 card pool (15 classic Magic cards)

**Technical Deliverables**:
- Unified Rust contract (`Lobby` + `DraftRoom` variants)
- TypeScript frontend with `@linera/client` integration
- Automated deployment pipeline for Conway testnet
- Real-time state synchronization (1-2 second polling)

## Wave-6+ Vision

**Enhanced Gameplay**:
- Custom card sets and rarity distributions
- Tournament brackets with elimination rounds
- Timed picks with automatic fallback selection
- Draft analytics and player statistics

**Advanced Features**:
- Cross-chain tournaments spanning multiple Linera networks
- NFT integration for tradeable drafted cards
- Reputation system with on-chain player rankings
- Spectator mode with real-time draft viewing

**Ecosystem Integration**:
- Plugin architecture for custom draft formats
- Integration with existing card game ecosystems
- Mobile application with native Linera wallet support
- Decentralized tournament hosting and prize distribution

LiveDraft Arena demonstrates Linera's microchain capabilities while establishing foundation infrastructure for complex multi-player gaming applications.