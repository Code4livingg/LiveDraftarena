// GraphQL client for Linera backend service
import { GRAPHQL_ENDPOINT } from './config.ts';

// Player ID management for multi-user sessions
let currentPlayerId: string | null = null;

// Get or generate player ID for this browser session
function getOrCreatePlayerId(): string {
  let playerId = localStorage.getItem('livedraft_player_id');

  if (!playerId) {
    // Generate new player ID (16 hex characters)
    playerId = Array.from(crypto.getRandomValues(new Uint8Array(8)))
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');

    localStorage.setItem('livedraft_player_id', playerId);
  }

  currentPlayerId = playerId;
  return playerId;
}

// GraphQL query and mutation definitions
export const QUERIES = {
  ROOMS: `
    query GetRooms {
      rooms {
        chainId
        roomName
        maxPlayers
        currentPlayers
        status
      }
    }
  `,

  ROOM_STATE: `
    query GetRoomState($chainId: String!) {
      roomState(chainId: $chainId) {
        chainId
        players
        maxPlayers
        currentTurn
        round
        maxRounds
        pool {
          id
          name
          power
        }
        status
      }
    }
  `,

  MY_PICKS: `
    query MyPicks($chainId: String!) {
      myPicks(chainId: $chainId) {
        id
        name
        power
      }
    }
  `,

  PLAYER_INFO: `
    query PlayerInfo {
      playerInfo
    }
  `,

  HEALTH: `
    query Health {
      health
    }
  `
};

export const MUTATIONS = {
  CREATE_ROOM: `
    mutation CreateRoom($input: CreateRoomInput!) {
      createRoom(input: $input) {
        success
        message
        transactionHash
      }
    }
  `,

  JOIN_ROOM: `
    mutation JoinRoom($chainId: String!) {
      joinRoom(chainId: $chainId) {
        success
        message
        transactionHash
      }
    }
  `,

  START_DRAFT: `
    mutation StartDraft($chainId: String!) {
      startDraft(chainId: $chainId) {
        success
        message
        transactionHash
      }
    }
  `,

  PICK_ITEM: `
    mutation PickItem($chainId: String!, $input: PickItemInput!) {
      pickItem(chainId: $chainId, input: $input) {
        success
        message
        transactionHash
      }
    }
  `
};

// GraphQL request function using fetch with player identity
export async function graphqlRequest<T>(
  query: string,
  variables?: Record<string, any>
): Promise<T> {
  const playerId = getOrCreatePlayerId();

  const response = await fetch(GRAPHQL_ENDPOINT, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'x-player-id': playerId,
    },
    credentials: 'include',
    body: JSON.stringify({
      query,
      variables,
    }),
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const result = await response.json();

  if (result.errors) {
    throw new Error(result.errors[0]?.message || 'GraphQL error');
  }

  return result.data;
}

// Get current player ID
export function getCurrentPlayerId(): string {
  return currentPlayerId || getOrCreatePlayerId();
}

// Get player info from backend
export async function getPlayerInfo(): Promise<string> {
  try {
    const data = await graphqlRequest<{ playerInfo: string }>(
      QUERIES.PLAYER_INFO
    );
    return data.playerInfo;
  } catch {
    return 'Unknown player';
  }
}

// Health check function
export async function checkBackendHealth(): Promise<boolean> {
  try {
    const data = await graphqlRequest<{ health: string }>(QUERIES.HEALTH);
    return data.health === 'Service is running';
  } catch {
    return false;
  }
}
