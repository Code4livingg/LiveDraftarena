use async_graphql::{Enum, SimpleObject};
use linera_core::data_types::ChainId;
use serde::{Deserialize, Serialize};

/// Draft room status matching the contract enum
#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, PartialEq, Eq)]
pub enum RoomStatus {
    Waiting,
    Drafting,
    Finished,
}

/// Draft item matching the contract struct
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct DraftItem {
    pub id: u8,
    pub name: String,
    pub power: u32,
}

/// Draft room metadata matching the contract struct
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct DraftRoomMetadata {
    pub room_name: String,
    pub max_players: u8,
    pub status: RoomStatus,
}

/// Room data for GraphQL responses
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct RoomData {
    pub chain_id: String, // ChainId as string for GraphQL
    pub room_name: String,
    pub max_players: u8,
    pub current_players: u8,
    pub status: RoomStatus,
}

/// Draft room state for individual room queries
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct DraftRoomState {
    pub chain_id: String, // ChainId as string for GraphQL
    pub players: Vec<String>, // Owner addresses as strings
    pub max_players: u8,
    pub current_turn: u8,
    pub round: u8,
    pub max_rounds: u8,
    pub pool: Vec<DraftItem>,
    pub status: RoomStatus,
}

/// Player picks for GraphQL response
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PlayerPicks {
    pub player: String, // Owner address as string
    pub items: Vec<DraftItem>,
}

/// Operation inputs for mutations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomInput {
    pub room_name: String,
    pub max_players: u8, // Match contract u8 type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickItemInput {
    pub item_id: u32, // Frontend uses u32, convert to u8 for contract
}

/// Operation result for mutations
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
    pub transaction_hash: Option<String>,
}