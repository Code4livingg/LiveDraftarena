# Changelog

All notable changes to LiveDraft Arena will be documented in this file.

## Wave 5

### Added
- Implemented real-time on-chain drafting game
- Lobby creates DraftRoom microchains
- Turn-based deterministic draft logic
- Real linera-web frontend integration
- Conway testnet deployment support
- Multi-user ready architecture

### Smart Contracts
- Unified contract with `Lobby` and `DraftRoom` variants
- Snake draft algorithm with forward/backward turn progression
- Hardcoded Wave-5 card pool (15 classic Magic cards)
- Room creation with configurable max players (2-8)
- Turn validation and state enforcement

### Frontend
- React + TypeScript with `@linera/client`
- Real-time GraphQL polling (1-2 second intervals)
- Wallet connection with chain selection
- Room discovery and creation interface
- Turn-based card picking with visual feedback

### Infrastructure
- Automated Conway testnet deployment scripts
- Production-ready build pipeline
- GraphQL service integration
- Cross-platform deployment support

### Technical Features
- Microchain isolation per draft room
- Deterministic game state management
- Real-time state synchronization
- Persistent wallet and chain configuration