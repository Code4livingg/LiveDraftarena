use linera_sdk::{
    base::{ChainId, Owner, WithContractAbi, ContractAbi},
    views::{MapView, RootView, View},
    Contract, ContractRuntime,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod service;

/// Draft room status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomStatus {
    Waiting,
    Drafting,
    Finished,
}

/// Metadata for a draft room
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftRoomMetadata {
    pub room_name: String,
    pub max_players: u8,
    pub status: RoomStatus,
}

/// Operations
#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    CreateRoom {
        room_name: String,
        max_players: u8,
    },
}

/// Messages
#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    // No cross-chain messages needed yet
}

/// Errors
#[derive(Debug, Error)]
pub enum LiveDraftArenaError {
    #[error("Room name cannot be empty")]
    EmptyRoomName,
    #[error("Max players must be between 2 and 8")]
    InvalidMaxPlayers,
}

/// Contract ABI
pub struct LiveDraftArenaAbi;

impl ContractAbi for LiveDraftArenaAbi {
    type Operation = Operation;
    type Response = ();
}

impl WithContractAbi for LiveDraftArena {
    type Abi = LiveDraftArenaAbi;
}

/// Application state
#[derive(RootView)]
pub struct LiveDraftArena {
    pub rooms: MapView<ChainId, DraftRoomMetadata>,
}

impl Contract for LiveDraftArena {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        LiveDraftArena {
            rooms: MapView::load(runtime.root_view_storage_context())
                .await
                .expect("Failed to load rooms"),
        }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // Initialize with empty rooms
    }

    async fn execute_operation(&mut self, operation: Operation) -> Vec<Self::Message> {
        match operation {
            Operation::CreateRoom { room_name, max_players } => {
                // Validate input
                if room_name.trim().is_empty() {
                    panic!("{}", LiveDraftArenaError::EmptyRoomName);
                }
                if max_players < 2 || max_players > 8 {
                    panic!("{}", LiveDraftArenaError::InvalidMaxPlayers);
                }

                // Store room metadata
                let metadata = DraftRoomMetadata {
                    room_name,
                    max_players,
                    status: RoomStatus::Waiting,
                };

                // Use a dummy chain ID for now
                let chain_id = ChainId::root(0);
                self.rooms
                    .insert(&chain_id, metadata)
                    .expect("Failed to store room metadata");

                vec![]
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        // No message handling needed yet
    }
}