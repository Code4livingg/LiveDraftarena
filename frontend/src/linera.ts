import { createClient, type Signer } from "@linera/client";

export const GRAPHQL_ENDPOINT = "https://conway-testnet.linera.net:8080/graphql";
export const LOBBY_APP_ID = "REPLACE_AFTER_DEPLOY";

export function createLineraClient(signer: Signer) {
  return createClient({
    endpoint: GRAPHQL_ENDPOINT,
    signer,
  });
}

// Local storage keys
const STORAGE_KEYS = {
  CHAIN_ID: 'livedraft_chain_id',
  SIGNER_KEY: 'livedraft_signer_key',
};

// Get stored chain ID or return null
export const getStoredChainId = (): string | null => {
  return localStorage.getItem(STORAGE_KEYS.CHAIN_ID);
};

// Store chain ID in localStorage
export const storeChainId = (chainId: string): void => {
  localStorage.setItem(STORAGE_KEYS.CHAIN_ID, chainId);
};

// Clear stored data
export const clearStoredData = (): void => {
  localStorage.removeItem(STORAGE_KEYS.CHAIN_ID);
  localStorage.removeItem(STORAGE_KEYS.SIGNER_KEY);
};

// Operation types matching Rust contract
export const OPERATIONS = {
  // Lobby operations
  CREATE_ROOM: (room_name: string, max_players: number) => ({
    CreateRoom: { room_name, max_players }
  }),
  
  // DraftRoom operations
  JOIN_ROOM: () => ({ JoinRoom: {} }),
  START_DRAFT: () => ({ StartDraft: {} }),
  PICK_ITEM: (item_id: number) => ({ PickItem: { item_id } }),
  FINALIZE_DRAFT: () => ({ FinalizeDraft: {} }),
};

// GraphQL queries
export const QUERIES = {
  // Query lobby for all rooms
  LOBBY_ROOMS: `
    query {
      rooms {
        chain_id
        metadata {
          room_name
          max_players
          status
        }
      }
    }
  `,
  
  // Query specific draft room state
  ROOM_STATE: `
    query {
      players
      max_players
      current_turn
      round
      max_rounds
      pool {
        id
        name
        power
      }
      picks
      status
      creator
    }
  `
};

// Validate chain ID format
export const isValidChainId = (chainId: string): boolean => {
  return /^[a-f0-9]{64}$/.test(chainId);
};