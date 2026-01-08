// Real Linera Web Integration
import { LineraClient, Signer } from '@linera/client';

// Hardcoded configuration for Conway testnet
export const LINERA_CONFIG = {
  // Conway testnet GraphQL endpoint
  GRAPHQL_ENDPOINT: 'https://conway-testnet.linera.net:8080/graphql',
  // Hardcoded Lobby application ID (replace with actual deployed app ID)
  LOBBY_APP_ID: 'e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65',
};

// Local storage keys
const STORAGE_KEYS = {
  CHAIN_ID: 'livedraft_chain_id',
  SIGNER_KEY: 'livedraft_signer_key',
};

// Initialize Linera client
export const createLineraClient = (): LineraClient => {
  return new LineraClient({
    endpoint: LINERA_CONFIG.GRAPHQL_ENDPOINT,
  });
};

// Create or load signer from localStorage
export const createOrLoadSigner = async (): Promise<Signer> => {
  const storedKey = localStorage.getItem(STORAGE_KEYS.SIGNER_KEY);
  
  if (storedKey) {
    // Load existing signer
    return Signer.fromPrivateKey(storedKey);
  } else {
    // Create new signer and store it
    const signer = Signer.generate();
    localStorage.setItem(STORAGE_KEYS.SIGNER_KEY, signer.privateKey);
    return signer;
  }
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

// Execute GraphQL query
export const executeQuery = async (
  client: LineraClient,
  chainId: string,
  applicationId: string,
  query: string
): Promise<any> => {
  try {
    const result = await client.query({
      chainId,
      applicationId,
      query,
    });
    return result;
  } catch (error) {
    console.error('GraphQL query failed:', error);
    throw error;
  }
};

// Execute operation on chain
export const executeOperation = async (
  client: LineraClient,
  signer: Signer,
  chainId: string,
  applicationId: string,
  operation: any
): Promise<any> => {
  try {
    const result = await client.executeOperation({
      chainId,
      applicationId,
      operation,
      signer,
    });
    return result;
  } catch (error) {
    console.error('Operation execution failed:', error);
    throw error;
  }
};

// Get user address from signer
export const getUserAddress = (signer: Signer): string => {
  return signer.address;
};

// Validate chain ID format
export const isValidChainId = (chainId: string): boolean => {
  return /^[a-f0-9]{64}$/.test(chainId);
};