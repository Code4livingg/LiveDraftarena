use linera_sdk::{
    base::ChainId,
    views::{MapView, RootView, View},
    Contract, ContractRuntime,
};
use serde::{Deserialize, Serialize};

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
pub enum Message {}

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
                    return vec![];
                }
                if max_players < 2 || max_players > 8 {
                    return vec![];
                }

                // Store room metadata
                let metadata = DraftRoomMetadata {
                    room_name,
                    max_players,
                    status: RoomStatus::Waiting,
                };

                // Use a dummy chain ID for now
                let chain_id = ChainId::root(0);
                let _ = self.rooms.insert(&chain_id, metadata);

                vec![]
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        // No message handling needed yet
    }
}