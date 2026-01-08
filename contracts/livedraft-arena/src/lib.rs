use linera_sdk::{
    base::{ChainId, Owner},
    views::{MapView, RootView, View},
    Contract, ContractRuntime,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod draft_room;
pub mod service;

pub use draft_room::{DraftRoom, DraftRoomOperation, DraftRoomMessage, DraftStatus as DraftRoomStatus};

/// Draft room status (for lobby metadata)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomStatus {
    Waiting,
    Drafting,
    Finished,
}

impl From<DraftRoomStatus> for RoomStatus {
    fn from(status: DraftRoomStatus) -> Self {
        match status {
            DraftRoomStatus::Waiting => RoomStatus::Waiting,
            DraftRoomStatus::Drafting => RoomStatus::Drafting,
            DraftRoomStatus::Finished => RoomStatus::Finished,
        }
    }
}

/// Metadata for a draft room
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftRoomMetadata {
    pub room_name: String,
    pub max_players: u8,
    pub status: RoomStatus,
}

/// Parameters to determine contract type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractParameters {
    Lobby,
    DraftRoom { max_players: u8 },
}

/// Operations for Lobby
#[derive(Debug, Deserialize, Serialize)]
pub enum LobbyOperation {
    CreateRoom {
        room_name: String,
        max_players: u8,
    },
}

/// The Lobby application state.
#[derive(RootView)]
pub struct Lobby {
    pub rooms: MapView<ChainId, DraftRoomMetadata>,
    runtime: ContractRuntime<LiveDraftArena>,
}

impl Lobby {
    async fn load(runtime: ContractRuntime<LiveDraftArena>) -> Self {
        Lobby {
            rooms: MapView::load(runtime.root_view_storage_context())
                .await
                .expect("Failed to load rooms"),
            runtime,
        }
    }

    async fn instantiate(&mut self, _argument: ()) {
        // Lobby starts with no rooms
    }

    async fn execute_operation(&mut self, operation: LobbyOperation) -> Vec<()> {
        match operation {
            LobbyOperation::CreateRoom {
                room_name,
                max_players,
            } => {
                // Validate input
                if room_name.trim().is_empty() {
                    panic!("{}", LobbyError::EmptyRoomName);
                }
                if max_players < 2 || max_players > 8 {
                    panic!("{}", LobbyError::InvalidMaxPlayers);
                }

                // Require authenticated signer
                let signer = self
                    .runtime
                    .authenticated_signer()
                    .ok_or(LobbyError::AuthenticationRequired)
                    .expect("Authentication required");

                // Open new microchain for the draft room
                let chain_id = self
                    .runtime
                    .open_chain(
                        self.runtime.application_id(),
                        ContractParameters::DraftRoom { max_players },
                    )
                    .await
                    .expect("Failed to open new chain");

                // Store room metadata
                let metadata = DraftRoomMetadata {
                    room_name,
                    max_players,
                    status: RoomStatus::Waiting,
                };

                self.rooms
                    .insert(&chain_id, metadata)
                    .expect("Failed to store room metadata");

                vec![]
            }
        }
    }
}

/// Unified operations
#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    // Lobby operations
    CreateRoom {
        room_name: String,
        max_players: u8,
    },
    // DraftRoom operations
    JoinRoom,
    StartDraft,
    PickItem { item_id: u8 },
    FinalizeDraft,
}

/// Unified messages
#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    // No cross-chain messages needed yet
}

/// Errors that can occur during contract execution.
#[derive(Debug, Error)]
pub enum LobbyError {
    #[error("Room name cannot be empty")]
    EmptyRoomName,
    #[error("Max players must be between 2 and 8")]
    InvalidMaxPlayers,
    #[error("Authentication required")]
    AuthenticationRequired,
}

/// Unified errors
#[derive(Debug, Error)]
pub enum LiveDraftArenaError {
    #[error("Lobby error: {0}")]
    Lobby(#[from] LobbyError),
    #[error("DraftRoom error: {0}")]
    DraftRoom(#[from] draft_room::DraftRoomError),
}

/// Unified application state
#[derive(RootView)]
pub enum LiveDraftArena {
    Lobby(Lobby),
    DraftRoom(DraftRoom),
}

impl Contract for LiveDraftArena {
    type Message = Message;
    type Parameters = ContractParameters;
    type InstantiationArgument = Option<Owner>; // creator for DraftRoom, None for Lobby

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        match runtime.parameters() {
            ContractParameters::Lobby => {
                let lobby = Lobby::load(runtime).await;
                LiveDraftArena::Lobby(lobby)
            }
            ContractParameters::DraftRoom { .. } => {
                let draft_room = DraftRoom::load(runtime).await;
                LiveDraftArena::DraftRoom(draft_room)
            }
        }
    }

    async fn instantiate(&mut self, creator: Self::InstantiationArgument) {
        match self {
            LiveDraftArena::Lobby(lobby) => {
                lobby.instantiate(()).await;
            }
            LiveDraftArena::DraftRoom(draft_room) => {
                if let Some(creator) = creator {
                    draft_room.instantiate(creator).await;
                }
            }
        }
    }

    async fn execute_operation(&mut self, operation: Operation) -> Vec<Self::Message> {
        match (self, operation) {
            (LiveDraftArena::Lobby(lobby), Operation::CreateRoom { room_name, max_players }) => {
                lobby.execute_operation(LobbyOperation::CreateRoom { room_name, max_players }).await;
                vec![]
            }
            (LiveDraftArena::DraftRoom(draft_room), Operation::JoinRoom) => {
                draft_room.execute_operation(DraftRoomOperation::JoinRoom).await;
                vec![]
            }
            (LiveDraftArena::DraftRoom(draft_room), Operation::StartDraft) => {
                draft_room.execute_operation(DraftRoomOperation::StartDraft).await;
                vec![]
            }
            (LiveDraftArena::DraftRoom(draft_room), Operation::PickItem { item_id }) => {
                draft_room.execute_operation(DraftRoomOperation::PickItem { item_id }).await;
                vec![]
            }
            (LiveDraftArena::DraftRoom(draft_room), Operation::FinalizeDraft) => {
                draft_room.execute_operation(DraftRoomOperation::FinalizeDraft).await;
                vec![]
            }
            _ => {
                // Invalid operation for contract type
                vec![]
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        // No message handling needed yet
    }
}