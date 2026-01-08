use async_graphql::{Context, Object, Result};
use linera_client::Client;
use linera_core::data_types::{ApplicationId, ChainId};
use serde_json::json;
use tracing::{error, info};

use crate::types::{CreateRoomInput, OperationResult, PickItemInput};

/// GraphQL Mutation root
pub struct MutationRoot {
    client: Client,
    app_id: ApplicationId,
    default_chain_id: ChainId,
}

impl MutationRoot {
    pub fn new(client: Client, app_id: ApplicationId, default_chain_id: ChainId) -> Self {
        Self {
            client,
            app_id,
            default_chain_id,
        }
    }
}

#[Object]
impl MutationRoot {
    /// Create a new draft room
    async fn create_room(&self, input: CreateRoomInput) -> Result<OperationResult> {
        info!("Creating room: {} with {} max players", input.room_name, input.max_players);

        // Validate input
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
        let operation = json!({
            "CreateRoom": {
                "room_name": input.room_name,
                "max_players": input.max_players
            }
        });

        // Submit operation to the lobby chain
        match self.client.execute_operation(self.default_chain_id, self.app_id, operation).await {
            Ok(response) => {
                info!("Successfully created room");
                Ok(OperationResult {
                    success: true,
                    message: "Room created successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)), // Would extract actual hash
                })
            }
            Err(e) => {
                error!("Failed to create room: {}", e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to create room: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }

    /// Join a draft room
    async fn join_room(&self, chain_id: String) -> Result<OperationResult> {
        info!("Joining room on chain: {}", chain_id);

        // Parse chain ID
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Create the operation matching the contract's Operation enum
        let operation = json!({
            "JoinRoom": {}
        });

        // Submit operation to the room's microchain
        match self.client.execute_operation(chain_id, self.app_id, operation).await {
            Ok(response) => {
                info!("Successfully joined room");
                Ok(OperationResult {
                    success: true,
                    message: "Joined room successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)),
                })
            }
            Err(e) => {
                error!("Failed to join room: {}", e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to join room: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }

    /// Start a draft (creator only)
    async fn start_draft(&self, chain_id: String) -> Result<OperationResult> {
        info!("Starting draft on chain: {}", chain_id);

        // Parse chain ID
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Create the operation matching the contract's Operation enum
        let operation = json!({
            "StartDraft": {}
        });

        // Submit operation to the room's microchain
        match self.client.execute_operation(chain_id, self.app_id, operation).await {
            Ok(response) => {
                info!("Successfully started draft");
                Ok(OperationResult {
                    success: true,
                    message: "Draft started successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)),
                })
            }
            Err(e) => {
                error!("Failed to start draft: {}", e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to start draft: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }

    /// Pick an item during draft
    async fn pick_item(&self, chain_id: String, input: PickItemInput) -> Result<OperationResult> {
        info!("Picking item {} on chain: {}", input.item_id, chain_id);

        // Parse chain ID
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Create the operation matching the contract's Operation enum
        let operation = json!({
            "PickItem": {
                "item_id": input.item_id
            }
        });

        // Submit operation to the room's microchain
        match self.client.execute_operation(chain_id, self.app_id, operation).await {
            Ok(response) => {
                info!("Successfully picked item");
                Ok(OperationResult {
                    success: true,
                    message: "Item picked successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)),
                })
            }
            Err(e) => {
                error!("Failed to pick item: {}", e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to pick item: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }

    /// Finalize draft when complete
    async fn finalize_draft(&self, chain_id: String) -> Result<OperationResult> {
        info!("Finalizing draft on chain: {}", chain_id);

        // Parse chain ID
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Create the operation matching the contract's Operation enum
        let operation = json!({
            "FinalizeDraft": {}
        });

        // Submit operation to the room's microchain
        match self.client.execute_operation(chain_id, self.app_id, operation).await {
            Ok(response) => {
                info!("Successfully finalized draft");
                Ok(OperationResult {
                    success: true,
                    message: "Draft finalized successfully".to_string(),
                    transaction_hash: Some(format!("{:?}", response)),
                })
            }
            Err(e) => {
                error!("Failed to finalize draft: {}", e);
                Ok(OperationResult {
                    success: false,
                    message: format!("Failed to finalize draft: {}", e),
                    transaction_hash: None,
                })
            }
        }
    }
}