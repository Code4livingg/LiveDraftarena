use linera_sdk::{
    base::Owner,
    views::{MapView, RootView, View},
    Contract, ContractRuntime,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Status of a draft room
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DraftStatus {
    Waiting,
    Drafting,
    Finished,
}

/// An item that can be drafted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftItem {
    pub id: u8,
    pub name: String,
    pub power: u32,
}

/// The DraftRoom application state
#[derive(RootView)]
pub struct DraftRoom {
    pub players: Vec<Owner>,
    pub max_players: u8,
    pub current_turn: u8,
    pub round: u8,
    pub max_rounds: u8,
    pub pool: Vec<DraftItem>,
    pub picks: MapView<Owner, Vec<DraftItem>>,
    pub status: DraftStatus,
    pub creator: Option<Owner>,
}

/// Operations for the DraftRoom
#[derive(Debug, Deserialize, Serialize)]
pub enum DraftRoomOperation {
    JoinRoom,
    StartDraft,
    PickItem { item_id: u8 },
    FinalizeDraft,
}

/// Messages for DraftRoom
#[derive(Debug, Deserialize, Serialize)]
pub enum DraftRoomMessage {
    // No cross-chain messages needed yet
}

/// Errors for DraftRoom operations
#[derive(Debug, Error)]
pub enum DraftRoomError {
    #[error("Room is not in waiting status")]
    NotWaiting,
    #[error("Room is full")]
    RoomFull,
    #[error("Player already joined")]
    AlreadyJoined,
    #[error("Only creator can start draft")]
    NotCreator,
    #[error("Room is not in drafting status")]
    NotDrafting,
    #[error("Not your turn")]
    NotYourTurn,
    #[error("Item not found in pool")]
    ItemNotFound,
    #[error("Authentication required")]
    AuthenticationRequired,
    #[error("Draft not finished")]
    DraftNotFinished,
}

impl DraftRoom {
    /// Initialize hardcoded Wave-5 pool
    fn initialize_pool() -> Vec<DraftItem> {
        vec![
            DraftItem { id: 1, name: "Lightning Bolt".to_string(), power: 100 },
            DraftItem { id: 2, name: "Counterspell".to_string(), power: 90 },
            DraftItem { id: 3, name: "Giant Growth".to_string(), power: 80 },
            DraftItem { id: 4, name: "Dark Ritual".to_string(), power: 85 },
            DraftItem { id: 5, name: "Healing Salve".to_string(), power: 70 },
            DraftItem { id: 6, name: "Ancestral Recall".to_string(), power: 95 },
            DraftItem { id: 7, name: "Black Lotus".to_string(), power: 100 },
            DraftItem { id: 8, name: "Mox Pearl".to_string(), power: 90 },
            DraftItem { id: 9, name: "Time Walk".to_string(), power: 95 },
            DraftItem { id: 10, name: "Swords to Plowshares".to_string(), power: 85 },
            DraftItem { id: 11, name: "Force of Will".to_string(), power: 90 },
            DraftItem { id: 12, name: "Brainstorm".to_string(), power: 75 },
            DraftItem { id: 13, name: "Sol Ring".to_string(), power: 85 },
            DraftItem { id: 14, name: "Path to Exile".to_string(), power: 80 },
            DraftItem { id: 15, name: "Demonic Tutor".to_string(), power: 90 },
        ]
    }

    /// Get current player based on turn and round
    fn get_current_player(&self) -> Option<&Owner> {
        if self.players.is_empty() {
            return None;
        }

        let player_count = self.players.len() as u8;
        
        // Snake draft: odd rounds go forward, even rounds go backward
        let player_index = if self.round % 2 == 1 {
            // Forward direction
            self.current_turn % player_count
        } else {
            // Backward direction
            (player_count - 1) - (self.current_turn % player_count)
        };

        self.players.get(player_index as usize)
    }

    /// Advance to next turn/round
    fn advance_turn(&mut self) {
        let player_count = self.players.len() as u8;
        
        self.current_turn += 1;
        
        // Check if we completed a round
        if self.current_turn >= player_count {
            self.current_turn = 0;
            self.round += 1;
            
            // Check if draft is complete
            if self.round > self.max_rounds {
                self.status = DraftStatus::Finished;
            }
        }
    }
}

impl Contract for DraftRoom {
    type Message = DraftRoomMessage;
    type Parameters = u8; // max_players
    type InstantiationArgument = Owner; // creator

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        DraftRoom {
            players: Vec::new(),
            max_players: 0,
            current_turn: 0,
            round: 1,
            max_rounds: 3,
            pool: Vec::new(),
            picks: MapView::load(runtime.root_view_storage_context())
                .await
                .expect("Failed to load picks"),
            status: DraftStatus::Waiting,
            creator: None,
        }
    }

    async fn instantiate(&mut self, creator: Self::InstantiationArgument) {
        self.creator = Some(creator);
        self.max_players = *self.runtime.parameters();
    }

    async fn execute_operation(&mut self, operation: DraftRoomOperation) -> Vec<Self::Message> {
        match operation {
            DraftRoomOperation::JoinRoom => {
                let signer = self
                    .runtime
                    .authenticated_signer()
                    .ok_or(DraftRoomError::AuthenticationRequired)
                    .expect("Authentication required");

                if self.status != DraftStatus::Waiting {
                    panic!("{}", DraftRoomError::NotWaiting);
                }

                if self.players.len() >= self.max_players as usize {
                    panic!("{}", DraftRoomError::RoomFull);
                }

                if self.players.contains(&signer) {
                    panic!("{}", DraftRoomError::AlreadyJoined);
                }

                self.players.push(signer);
                
                // Initialize empty picks for the player
                self.picks
                    .insert(&signer, Vec::new())
                    .expect("Failed to initialize player picks");

                vec![]
            }

            DraftRoomOperation::StartDraft => {
                let signer = self
                    .runtime
                    .authenticated_signer()
                    .ok_or(DraftRoomError::AuthenticationRequired)
                    .expect("Authentication required");

                if self.creator != Some(signer) {
                    panic!("{}", DraftRoomError::NotCreator);
                }

                if self.status != DraftStatus::Waiting {
                    panic!("{}", DraftRoomError::NotWaiting);
                }

                self.pool = Self::initialize_pool();
                self.status = DraftStatus::Drafting;
                self.current_turn = 0;
                self.round = 1;

                vec![]
            }

            DraftRoomOperation::PickItem { item_id } => {
                let signer = self
                    .runtime
                    .authenticated_signer()
                    .ok_or(DraftRoomError::AuthenticationRequired)
                    .expect("Authentication required");

                if self.status != DraftStatus::Drafting {
                    panic!("{}", DraftRoomError::NotDrafting);
                }

                let current_player = self
                    .get_current_player()
                    .ok_or(DraftRoomError::NotYourTurn)
                    .expect("No current player");

                if *current_player != signer {
                    panic!("{}", DraftRoomError::NotYourTurn);
                }

                // Find and remove item from pool
                let item_index = self
                    .pool
                    .iter()
                    .position(|item| item.id == item_id)
                    .ok_or(DraftRoomError::ItemNotFound)
                    .expect("Item not found");

                let picked_item = self.pool.remove(item_index);

                // Add to player's picks
                let mut player_picks = self
                    .picks
                    .get(&signer)
                    .await
                    .expect("Failed to get player picks")
                    .unwrap_or_default();
                
                player_picks.push(picked_item);
                
                self.picks
                    .insert(&signer, player_picks)
                    .expect("Failed to update player picks");

                // Advance turn
                self.advance_turn();

                vec![]
            }

            DraftRoomOperation::FinalizeDraft => {
                if self.status != DraftStatus::Finished {
                    panic!("{}", DraftRoomError::DraftNotFinished);
                }

                // Draft is already finished, nothing to do
                vec![]
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        // No message handling needed
    }
}