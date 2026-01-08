// Types matching the Rust contract definitions

export interface DraftItem {
  id: number;
  name: string;
  power: number;
}

export interface DraftRoomMetadata {
  room_name: string;
  max_players: number;
  status: 'Waiting' | 'Drafting' | 'Finished';
}

export interface RoomData {
  chain_id: string;
  metadata: DraftRoomMetadata;
}

// GraphQL response types
export interface RoomsResponse {
  rooms: RoomData[];
}

// DraftRoom state (for when we query a specific room)
export interface DraftRoomState {
  players: string[];
  max_players: number;
  current_turn: number;
  round: number;
  max_rounds: number;
  pool: DraftItem[];
  picks: Record<string, DraftItem[]>;
  status: 'Waiting' | 'Drafting' | 'Finished';
  creator?: string;
}