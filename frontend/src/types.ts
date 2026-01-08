// Types matching the backend GraphQL schema

export interface DraftItem {
  id: number;
  name: string;
  power: number;
}

export interface RoomData {
  chainId: string;
  roomName: string;
  maxPlayers: number;
  currentPlayers: number;
  status: 'Waiting' | 'Drafting' | 'Finished';
}

export interface DraftRoomState {
  chainId: string;
  players: string[];
  maxPlayers: number;
  currentTurn: number;
  round: number;
  maxRounds: number;
  pool: DraftItem[];
  status: 'Waiting' | 'Drafting' | 'Finished';
}

export interface OperationResult {
  success: boolean;
  message: string;
  transactionHash?: string;
}

// GraphQL response types
export interface RoomsResponse {
  rooms: RoomData[];
}

export interface RoomStateResponse {
  roomState: DraftRoomState | null;
}

export interface MyPicksResponse {
  myPicks: DraftItem[];
}

// Input types for mutations
export interface CreateRoomInput {
  roomName: string;
  maxPlayers: number; // Will be converted to u8 in backend
}

export interface PickItemInput {
  itemId: number; // Frontend u32, converted to u8 in backend
}