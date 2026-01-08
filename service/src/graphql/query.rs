use async_graphql::{Context, Object, Result};
use linera_client::Client;
use linera_core::data_types::{ApplicationId, ChainId};
use linera_views::views::View;
use std::collections::HashMap;
use tracing::{error, info};

use crate::types::{DraftRoomState, PlayerPicks, RoomData, RoomStatus};

/// GraphQL Query root
pub struct QueryRoot {
    client: Client,
    app_id: ApplicationId,
    default_chain_id: ChainId,
}

impl QueryRoot {
    pub fn new(client: Client, app_id: ApplicationId, default_chain_id: ChainId) -> Self {
        Self {
            client,
            app_id,
            default_chain_id,
        }
    }
}

#[Object]
impl QueryRoot {
    /// Get all draft rooms from the lobby
    async fn rooms(&self) -> Result<Vec<RoomData>> {
        info!("Querying rooms from lobby on chain: {}", self.default_chain_id);

        // Query the lobby application state
        match self.client.query_application(self.default_chain_id, self.app_id).await {
            Ok(response) => {
                // Parse the response to extract room data
                // This would need to match the actual Lobby state structure
                info!("Successfully queried lobby state");
                
                // For now, return empty list - this would be populated with actual room data
                // from the MapView<ChainId, DraftRoomMetadata> in the Lobby contract
                Ok(vec![])
            }
            Err(e) => {
                error!("Failed to query lobby state: {}", e);
                Err(async_graphql::Error::new(format!("Failed to query rooms: {}", e)))
            }
        }
    }

    /// Get the state of a specific draft room
    async fn room_state(&self, chain_id: String) -> Result<Option<DraftRoomState>> {
        info!("Querying room state for chain: {}", chain_id);

        // Parse chain ID
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Query the draft room application state
        match self.client.query_application(chain_id, self.app_id).await {
            Ok(response) => {
                info!("Successfully queried room state for chain: {}", chain_id);
                
                // Parse the response to extract draft room state
                // This would need to match the actual DraftRoom state structure
                
                // For now, return None - this would be populated with actual room state
                // from the DraftRoom contract fields (players, pool, picks, etc.)
                Ok(None)
            }
            Err(e) => {
                error!("Failed to query room state: {}", e);
                Err(async_graphql::Error::new(format!("Failed to query room state: {}", e)))
            }
        }
    }

    /// Get current user's picks in a room
    async fn my_picks(&self, chain_id: String, player_address: String) -> Result<Vec<crate::types::DraftItem>> {
        info!("Querying picks for player {} in room {}", player_address, chain_id);

        // Parse chain ID
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Query room state and extract player's picks
        match self.client.query_application(chain_id, self.app_id).await {
            Ok(_response) => {
                info!("Successfully queried player picks");
                
                // Extract picks for the specific player from the room state
                // This would parse the MapView<Owner, Vec<DraftItem>> from the contract
                Ok(vec![])
            }
            Err(e) => {
                error!("Failed to query player picks: {}", e);
                Err(async_graphql::Error::new(format!("Failed to query picks: {}", e)))
            }
        }
    }

    /// Health check endpoint
    async fn health(&self) -> Result<String> {
        Ok("Service is running".to_string())
    }
}