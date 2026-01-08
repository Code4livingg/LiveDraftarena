use async_graphql::{Context, Object, Result};
use linera_client::ClientContext;
use linera_core::data_types::{ApplicationId, ChainId};
use tracing::{error, info};

use crate::types::{CreateRoomInput, OperationResult, PickItemInput};
use super::get_context;

// Import the Operation enum from the contract
use livedraft_arena::Operation;

/// GraphQL Mutation root
pub struct MutationRoot {
    client: ClientContext,
    app_id: ApplicationId,
    default_chain_id: ChainId,
}

impl MutationRoot {
    pub fn new(client: ClientContext, app_id: ApplicationId, default_chain_id: ChainId) -> Self {
        Self {
            client,
            app_id,
            default_chain_id,
        }
    }
}

#[Object]
impl MutationRoot {
    /// Create a new draft room on the Lobby chain
    /// 
    /// This executes a CreateRoom operation on the Lobby contract, which:
    /// 1. Validates the room parameters
    /// 2. Opens a new microchain for the DraftRoom
    /// 3. Stores the room metadata in the Lobby state
    /// 
    /// The operation is signed with the player's deterministic Owner identity.
    async fn create_room(&self, ctx: &Context<'_>, input: CreateRoomInput) -> Result<OperationResult> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        let player_owner = context.get_player_owner();
        
        info!("Player {} creating room: {} with {} max players", 
              player_id, input.room_name, input.max_players);

        // Validate input on the service side for better UX
        if input.room_name.trim().is_empty() {
            return Ok(OperationResult {
                success: false,
                message: "Room name cannot be empty".to_string(),
                transaction_hash: None,
            });
        }

        if input.max_players < 2 || input.max_players > 8 {
            return Ok(OperationResult {
                success: false,
                message: "Max players must be between 2 and 8".to_string(),
                transaction_hash: None,
            });
        }

        // Create the operation matching the contract's Operation enum
        // This will be executed on the Lobby chain (default_chain_id)
        let operation = Operation::CreateRoom {
            room_name: input.room_name.clone(),
            max_players: input.max_players,
        };

        // Execute operation on the Lobby chain using the player's Owner identity
        // The Linera client will:
        // 1. Serialize the operation
        // 2. Create a transaction signed by the player's Owner
        // 3. Submit to the Lobby chain on Conway testnet
        // 4. Wait for confirmation
        match self.client.execute_operation(
            self.default_chain_id, 
            self.app_id, 
            &operation,
        ).await {
            Ok(response) => {
                info!("Player {} successfully created room '{}'", player_id, input.room_name);
                Ok(OperationResult {
                    success: true,
                    message: format!("Room '{}' created successfully", input.room_name),
                    transaction_hash: Some(format!("{:?}", response)), // Extract actual transaction hash
                })
            }
            Err(e) => {
                error!("Player {} failed to create room '{}': {}", player_id, input.room_name, e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to create room: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }

    /// Join a draft room on a specific microchain
    /// 
    /// This executes a JoinRoom operation on the DraftRoom contract, which:
    /// 1. Validates the room is in Waiting status
    /// 2. Checks room capacity
    /// 3. Adds the player to the room
    /// 4. Initializes empty picks for the player
    /// 
    /// The operation is signed with the player's deterministic Owner identity.
    async fn join_room(&self, ctx: &Context<'_>, chain_id: String) -> Result<OperationResult> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        let player_owner = context.get_player_owner();
        
        info!("Player {} joining room on chain: {}", player_id, chain_id);

        // Parse chain ID for the DraftRoom microchain
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Create the JoinRoom operation for the DraftRoom contract
        let operation = Operation::JoinRoom;

        // Execute operation on the DraftRoom microchain
        // The player's Owner identity will be used for authentication in the contract
        match self.client.execute_operation(
            chain_id, 
            self.app_id, 
            &operation,
        ).await {
            Ok(response) => {
                info!("Player {} successfully joined room on chain {}", player_id, chain_id);
                Ok(OperationResult {
                    success: true,
                    message: "Joined room successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)),
                })
            }
            Err(e) => {
                error!("Player {} failed to join room on chain {}: {}", player_id, chain_id, e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to join room: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }

    /// Start a draft (creator only)
    /// 
    /// This executes a StartDraft operation on the DraftRoom contract, which:
    /// 1. Validates the caller is the room creator
    /// 2. Initializes the hardcoded Wave-5 card pool
    /// 3. Sets the room status to Drafting
    /// 4. Resets turn/round counters
    /// 
    /// Only the room creator can start the draft.
    async fn start_draft(&self, ctx: &Context<'_>, chain_id: String) -> Result<OperationResult> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        let player_owner = context.get_player_owner();
        
        info!("Player {} starting draft on chain: {}", player_id, chain_id);

        // Parse chain ID for the DraftRoom microchain
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Create the StartDraft operation for the DraftRoom contract
        let operation = Operation::StartDraft;

        // Execute operation on the DraftRoom microchain
        // The contract will verify the caller is the creator
        match self.client.execute_operation(
            chain_id, 
            self.app_id, 
            &operation,
        ).await {
            Ok(response) => {
                info!("Player {} successfully started draft on chain {}", player_id, chain_id);
                Ok(OperationResult {
                    success: true,
                    message: "Draft started successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)),
                })
            }
            Err(e) => {
                error!("Player {} failed to start draft on chain {}: {}", player_id, chain_id, e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to start draft: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }

    /// Pick an item during draft
    /// 
    /// This executes a PickItem operation on the DraftRoom contract, which:
    /// 1. Validates it's the player's turn
    /// 2. Removes the item from the pool
    /// 3. Adds the item to the player's picks
    /// 4. Advances to the next turn/round
    /// 
    /// Only works when it's the player's turn in the snake draft.
    async fn pick_item(&self, ctx: &Context<'_>, chain_id: String, input: PickItemInput) -> Result<OperationResult> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        let player_owner = context.get_player_owner();
        
        info!("Player {} picking item {} on chain: {}", player_id, input.item_id, chain_id);

        // Parse chain ID for the DraftRoom microchain
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Create the PickItem operation for the DraftRoom contract
        let operation = Operation::PickItem {
            item_id: input.item_id as u8, // Convert from frontend u32 to contract u8
        };

        // Execute operation on the DraftRoom microchain
        // The contract will verify it's the player's turn and handle the pick logic
        match self.client.execute_operation(
            chain_id, 
            self.app_id, 
            &operation,
        ).await {
            Ok(response) => {
                info!("Player {} successfully picked item {} on chain {}", player_id, input.item_id, chain_id);
                Ok(OperationResult {
                    success: true,
                    message: "Item picked successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)),
                })
            }
            Err(e) => {
                error!("Player {} failed to pick item {} on chain {}: {}", player_id, input.item_id, chain_id, e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to pick item: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }

    /// Finalize draft when complete
    /// 
    /// This executes a FinalizeDraft operation on the DraftRoom contract.
    /// The contract validates that all rounds are complete before finalizing.
    async fn finalize_draft(&self, ctx: &Context<'_>, chain_id: String) -> Result<OperationResult> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        let player_owner = context.get_player_owner();
        
        info!("Player {} finalizing draft on chain: {}", player_id, chain_id);

        // Parse chain ID for the DraftRoom microchain
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Create the FinalizeDraft operation for the DraftRoom contract
        let operation = Operation::FinalizeDraft;

        // Execute operation on the DraftRoom microchain
        match self.client.execute_operation(
            chain_id, 
            self.app_id, 
            &operation,
        ).await {
            Ok(response) => {
                info!("Player {} successfully finalized draft on chain {}", player_id, chain_id);
                Ok(OperationResult {
                    success: true,
                    message: "Draft finalized successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)),
                })
            }
            Err(e) => {
                error!("Player {} failed to finalize draft on chain {}: {}", player_id, chain_id, e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to finalize draft: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }
}