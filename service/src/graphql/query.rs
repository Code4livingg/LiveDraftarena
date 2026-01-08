use async_graphql::{Context, Object, Result};
use linera_client::ClientContext;
use linera_core::data_types::{ApplicationId, ChainId};
use linera_sdk::base::Owner;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::{error, info, warn};

use crate::types::{DraftRoomState, RoomData, RoomStatus};
use super::get_context;

// Import contract types for state queries
use livedraft_arena::{
    LiveDraftArena, 
    DraftRoomMetadata, 
    RoomStatus as ContractRoomStatus, 
    DraftRoom,
    Lobby,
    draft_room::{DraftItem as ContractDraftItem, DraftStatus as ContractDraftStatus}
};

/// GraphQL Query root
pub struct QueryRoot {
    client: ClientContext,
    app_id: ApplicationId,
    default_chain_id: ChainId,
}

impl QueryRoot {
    pub fn new(client: ClientContext, app_id: ApplicationId, default_chain_id: ChainId) -> Self {
        Self {
            client,
            app_id,
            default_chain_id,
        }
    }

    /// Helper function to deserialize Lobby state from query response
    /// 
    /// Linera query responses contain the serialized application state.
    /// The format can vary - it might be JSON, bincode, or other formats.
    /// We try multiple deserialization strategies to handle different cases.
    async fn deserialize_lobby_state(&self, response_bytes: &[u8]) -> Result<HashMap<ChainId, DraftRoomMetadata>> {
        info!("Attempting to deserialize Lobby state from {} bytes", response_bytes.len());
        
        // Strategy 1: Try JSON deserialization first (most common for queries)
        if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(response_bytes) {
            info!("Successfully parsed response as JSON");
            
            // Handle different JSON structures that Linera might produce
            
            // Case 1: Direct LiveDraftArena enum serialization
            if let Some(lobby_obj) = json_value.get("Lobby") {
                return self.extract_rooms_from_lobby_json(lobby_obj).await;
            }
            
            // Case 2: Wrapped in additional structure
            if let Some(state_obj) = json_value.get("state") {
                if let Some(lobby_obj) = state_obj.get("Lobby") {
                    return self.extract_rooms_from_lobby_json(lobby_obj).await;
                }
            }
            
            // Case 3: Direct rooms object (if Linera serializes MapView directly)
            if let Some(rooms_obj) = json_value.get("rooms") {
                return self.extract_rooms_from_json_object(rooms_obj).await;
            }
            
            // Case 4: The entire response is the rooms MapView
            if json_value.is_object() {
                return self.extract_rooms_from_json_object(&json_value).await;
            }
        }
        
        // Strategy 2: Try bincode deserialization
        if let Ok(live_draft_arena) = bincode::deserialize::<LiveDraftArena>(response_bytes) {
            info!("Successfully deserialized with bincode");
            match live_draft_arena {
                LiveDraftArena::Lobby(_lobby) => {
                    warn!("Bincode deserialization successful but cannot extract MapView data without storage context");
                    // We can't access the MapView data directly from the deserialized struct
                    // because MapView requires a storage context to load its data
                    return Ok(HashMap::new());
                }
                LiveDraftArena::DraftRoom(_) => {
                    return Err(async_graphql::Error::new("Expected Lobby but got DraftRoom state"));
                }
            }
        }
        
        // Strategy 3: Try as raw string (sometimes Linera returns string-encoded JSON)
        if let Ok(json_str) = std::str::from_utf8(response_bytes) {
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_str) {
                info!("Successfully parsed response as string-encoded JSON");
                if let Some(lobby_obj) = json_value.get("Lobby") {
                    return self.extract_rooms_from_lobby_json(lobby_obj).await;
                }
            }
        }
        
        error!("All deserialization strategies failed for Lobby state");
        Err(async_graphql::Error::new("Failed to deserialize Lobby state: unsupported format"))
    }

    /// Extract rooms from Lobby JSON object
    async fn extract_rooms_from_lobby_json(&self, lobby_obj: &serde_json::Value) -> Result<HashMap<ChainId, DraftRoomMetadata>> {
        if let Some(rooms_obj) = lobby_obj.get("rooms") {
            self.extract_rooms_from_json_object(rooms_obj).await
        } else {
            warn!("No 'rooms' field found in Lobby JSON object");
            Ok(HashMap::new())
        }
    }

    /// Extract rooms from a JSON object representing the MapView
    async fn extract_rooms_from_json_object(&self, rooms_obj: &serde_json::Value) -> Result<HashMap<ChainId, DraftRoomMetadata>> {
        let mut rooms = HashMap::new();
        
        if let Some(rooms_map) = rooms_obj.as_object() {
            for (chain_id_str, metadata_value) in rooms_map {
                // Parse chain ID from string key
                if let Ok(chain_id) = ChainId::from_str(chain_id_str) {
                    // Deserialize metadata
                    if let Ok(metadata) = serde_json::from_value::<DraftRoomMetadata>(metadata_value.clone()) {
                        rooms.insert(chain_id, metadata);
                    } else {
                        warn!("Failed to deserialize room metadata for chain {}", chain_id_str);
                    }
                } else {
                    warn!("Failed to parse chain ID: {}", chain_id_str);
                }
            }
        } else if let Some(rooms_array) = rooms_obj.as_array() {
            // Handle case where MapView is serialized as array of [key, value] pairs
            for entry in rooms_array {
                if let Some(entry_array) = entry.as_array() {
                    if entry_array.len() == 2 {
                        if let (Some(key_str), Some(value_obj)) = (entry_array[0].as_str(), &entry_array[1]) {
                            if let Ok(chain_id) = ChainId::from_str(key_str) {
                                if let Ok(metadata) = serde_json::from_value::<DraftRoomMetadata>(value_obj.clone()) {
                                    rooms.insert(chain_id, metadata);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        info!("Extracted {} rooms from JSON object", rooms.len());
        Ok(rooms)
    }

    /// Helper function to deserialize DraftRoom state from query response
    /// 
    /// For DraftRoom, this is LiveDraftArena::DraftRoom(DraftRoom) where DraftRoom
    /// contains Vec<Owner>, Vec<DraftItem>, MapView<Owner, Vec<DraftItem>>, etc.
    /// We use multiple strategies to handle different serialization formats.
    async fn deserialize_draft_room_state(&self, response_bytes: &[u8], chain_id: ChainId) -> Result<Option<DraftRoomStateData>> {
        info!("Attempting to deserialize DraftRoom state from {} bytes for chain {}", response_bytes.len(), chain_id);
        
        // Strategy 1: Try JSON deserialization first
        if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(response_bytes) {
            info!("Successfully parsed DraftRoom response as JSON");
            
            // Case 1: Direct LiveDraftArena enum serialization
            if let Some(draft_room_obj) = json_value.get("DraftRoom") {
                return self.extract_draft_room_from_json(draft_room_obj, chain_id).await;
            }
            
            // Case 2: Wrapped in additional structure
            if let Some(state_obj) = json_value.get("state") {
                if let Some(draft_room_obj) = state_obj.get("DraftRoom") {
                    return self.extract_draft_room_from_json(draft_room_obj, chain_id).await;
                }
            }
            
            // Case 3: The entire response is the DraftRoom object
            if json_value.is_object() && json_value.get("players").is_some() {
                return self.extract_draft_room_from_json(&json_value, chain_id).await;
            }
        }
        
        // Strategy 2: Try bincode deserialization
        if let Ok(live_draft_arena) = bincode::deserialize::<LiveDraftArena>(response_bytes) {
            info!("Successfully deserialized DraftRoom with bincode");
            match live_draft_arena {
                LiveDraftArena::DraftRoom(_draft_room) => {
                    warn!("Bincode deserialization successful but cannot extract view data without storage context");
                    return Ok(None);
                }
                LiveDraftArena::Lobby(_) => {
                    return Err(async_graphql::Error::new("Expected DraftRoom but got Lobby state"));
                }
            }
        }
        
        // Strategy 3: Try as raw string
        if let Ok(json_str) = std::str::from_utf8(response_bytes) {
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_str) {
                info!("Successfully parsed DraftRoom response as string-encoded JSON");
                if let Some(draft_room_obj) = json_value.get("DraftRoom") {
                    return self.extract_draft_room_from_json(draft_room_obj, chain_id).await;
                }
            }
        }
        
        error!("All deserialization strategies failed for DraftRoom state on chain {}", chain_id);
        Err(async_graphql::Error::new("Failed to deserialize DraftRoom state: unsupported format"))
    }

    /// Extract DraftRoom data from JSON object
    async fn extract_draft_room_from_json(&self, draft_room_obj: &serde_json::Value, chain_id: ChainId) -> Result<Option<DraftRoomStateData>> {
        // Extract all the DraftRoom fields with proper error handling
        let players = self.extract_players_from_json(draft_room_obj)?;
        let max_players = draft_room_obj.get("max_players")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u8;
        let current_turn = draft_room_obj.get("current_turn")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u8;
        let round = draft_room_obj.get("round")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u8;
        let max_rounds = draft_room_obj.get("max_rounds")
            .and_then(|v| v.as_u64())
            .unwrap_or(3) as u8;
        let pool = self.extract_pool_from_json(draft_room_obj)?;
        let status = self.extract_status_from_json(draft_room_obj)?;
        let creator = self.extract_creator_from_json(draft_room_obj)?;
        
        let room_state = DraftRoomStateData {
            chain_id,
            players,
            max_players,
            current_turn,
            round,
            max_rounds,
            pool,
            status,
            creator,
        };
        
        info!("Successfully extracted DraftRoom state for chain {}: {} players, {} pool items", 
              chain_id, room_state.players.len(), room_state.pool.len());
        Ok(Some(room_state))
    }

    /// Extract player picks from DraftRoom state for a specific owner
    /// 
    /// The picks are stored in a MapView<Owner, Vec<DraftItem>> in the contract.
    /// We need to find the entry for the current player's Owner address.
    async fn extract_player_picks(&self, response_bytes: &[u8], player_owner: &Owner) -> Result<Vec<crate::types::DraftItem>> {
        info!("Extracting picks for player owner: {}", player_owner);
        
        // Try JSON deserialization first
        if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(response_bytes) {
            // Look for DraftRoom variant and picks field
            let picks_obj = if let Some(draft_room_obj) = json_value.get("DraftRoom") {
                draft_room_obj.get("picks")
            } else if let Some(state_obj) = json_value.get("state") {
                state_obj.get("DraftRoom").and_then(|dr| dr.get("picks"))
            } else {
                json_value.get("picks") // Direct picks object
            };
            
            if let Some(picks_obj) = picks_obj {
                return self.extract_picks_from_json_object(picks_obj, player_owner).await;
            }
        }
        
        // Try string-encoded JSON
        if let Ok(json_str) = std::str::from_utf8(response_bytes) {
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(draft_room_obj) = json_value.get("DraftRoom") {
                    if let Some(picks_obj) = draft_room_obj.get("picks") {
                        return self.extract_picks_from_json_object(picks_obj, player_owner).await;
                    }
                }
            }
        }
        
        info!("No picks found for player {} (this is normal for new players)", player_owner);
        Ok(vec![])
    }

    /// Extract picks from JSON object representing MapView<Owner, Vec<DraftItem>>
    async fn extract_picks_from_json_object(&self, picks_obj: &serde_json::Value, player_owner: &Owner) -> Result<Vec<crate::types::DraftItem>> {
        let owner_str = player_owner.to_string();
        
        // Case 1: MapView serialized as object with Owner strings as keys
        if let Some(picks_map) = picks_obj.as_object() {
            if let Some(player_picks_value) = picks_map.get(&owner_str) {
                if let Ok(contract_items) = serde_json::from_value::<Vec<ContractDraftItem>>(player_picks_value.clone()) {
                    let service_items = contract_items.into_iter().map(|item| {
                        crate::types::DraftItem {
                            id: item.id as u32,
                            name: item.name,
                            power: item.power,
                        }
                    }).collect();
                    
                    info!("Found {} picks for player {}", service_items.len(), player_owner);
                    return Ok(service_items);
                }
            }
        }
        
        // Case 2: MapView serialized as array of [key, value] pairs
        if let Some(picks_array) = picks_obj.as_array() {
            for entry in picks_array {
                if let Some(entry_array) = entry.as_array() {
                    if entry_array.len() == 2 {
                        if let Some(key_str) = entry_array[0].as_str() {
                            if key_str == owner_str {
                                if let Ok(contract_items) = serde_json::from_value::<Vec<ContractDraftItem>>(entry_array[1].clone()) {
                                    let service_items = contract_items.into_iter().map(|item| {
                                        crate::types::DraftItem {
                                            id: item.id as u32,
                                            name: item.name,
                                            power: item.power,
                                        }
                                    }).collect();
                                    
                                    info!("Found {} picks for player {} (array format)", service_items.len(), player_owner);
                                    return Ok(service_items);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        info!("No picks found for player {} in MapView", player_owner);
        Ok(vec![])
    }

    // Helper methods for JSON extraction
    fn extract_players_from_json(&self, draft_room_obj: &serde_json::Value) -> Result<Vec<String>> {
        if let Some(players_array) = draft_room_obj.get("players").and_then(|v| v.as_array()) {
            let players = players_array.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            Ok(players)
        } else {
            Ok(vec![])
        }
    }

    fn extract_pool_from_json(&self, draft_room_obj: &serde_json::Value) -> Result<Vec<crate::types::DraftItem>> {
        if let Some(pool_array) = draft_room_obj.get("pool").and_then(|v| v.as_array()) {
            let mut pool = Vec::new();
            for item_value in pool_array {
                if let Ok(contract_item) = serde_json::from_value::<ContractDraftItem>(item_value.clone()) {
                    pool.push(crate::types::DraftItem {
                        id: contract_item.id as u32,
                        name: contract_item.name,
                        power: contract_item.power,
                    });
                }
            }
            Ok(pool)
        } else {
            Ok(vec![])
        }
    }

    fn extract_status_from_json(&self, draft_room_obj: &serde_json::Value) -> Result<RoomStatus> {
        if let Some(status_str) = draft_room_obj.get("status").and_then(|v| v.as_str()) {
            match status_str {
                "Waiting" => Ok(RoomStatus::Waiting),
                "Drafting" => Ok(RoomStatus::Drafting),
                "Finished" => Ok(RoomStatus::Finished),
                _ => Ok(RoomStatus::Waiting),
            }
        } else {
            Ok(RoomStatus::Waiting)
        }
    }

    fn extract_creator_from_json(&self, draft_room_obj: &serde_json::Value) -> Result<Option<String>> {
        Ok(draft_room_obj.get("creator")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()))
    }
}

/// Intermediate struct for DraftRoom state data
struct DraftRoomStateData {
    chain_id: ChainId,
    players: Vec<String>,
    max_players: u8,
    current_turn: u8,
    round: u8,
    max_rounds: u8,
    pool: Vec<crate::types::DraftItem>,
    status: RoomStatus,
    creator: Option<String>,
}

#[Object]
impl QueryRoot {
    /// Get all draft rooms from the Lobby chain
    /// 
    /// This queries the Lobby contract state and deserializes the MapView<ChainId, DraftRoomMetadata>
    /// to return all created rooms with their metadata.
    async fn rooms(&self, ctx: &Context<'_>) -> Result<Vec<RoomData>> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        
        info!("Player {} querying rooms from Lobby on chain: {}", player_id, self.default_chain_id);

        // Query the Lobby application state on the default chain
        // This returns the serialized LiveDraftArena::Lobby state
        match self.client.query_application(self.default_chain_id, self.app_id).await {
            Ok(response) => {
                info!("Player {} successfully queried Lobby state, deserializing rooms...", player_id);
                
                // Deserialize the Lobby state to extract the rooms MapView
                match self.deserialize_lobby_state(&response).await {
                    Ok(rooms_map) => {
                        // Convert HashMap<ChainId, DraftRoomMetadata> to Vec<RoomData>
                        let mut rooms = Vec::new();
                        
                        for (chain_id, metadata) in rooms_map {
                            // Convert contract types to service types
                            let status = match metadata.status {
                                ContractRoomStatus::Waiting => RoomStatus::Waiting,
                                ContractRoomStatus::Drafting => RoomStatus::Drafting,
                                ContractRoomStatus::Finished => RoomStatus::Finished,
                            };
                            
                            rooms.push(RoomData {
                                chain_id: chain_id.to_string(),
                                room_name: metadata.room_name,
                                max_players: metadata.max_players,
                                current_players: 0, // TODO: Query actual player count from DraftRoom
                                status,
                            });
                        }
                        
                        info!("Player {} successfully retrieved {} rooms from Lobby", player_id, rooms.len());
                        Ok(rooms)
                    }
                    Err(e) => {
                        error!("Player {} failed to deserialize Lobby state: {}", player_id, e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Player {} failed to query Lobby state on chain {}: {}", player_id, self.default_chain_id, e);
                Err(async_graphql::Error::new(format!("Failed to query Lobby: {}", e)))
            }
        }
    }

    /// Get the state of a specific draft room
    /// 
    /// This queries a DraftRoom contract on its microchain and deserializes the complete
    /// room state including players, turn order, card pool, and draft status.
    async fn room_state(&self, ctx: &Context<'_>, chain_id: String) -> Result<Option<DraftRoomState>> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        
        info!("Player {} querying DraftRoom state for chain: {}", player_id, chain_id);

        // Parse chain ID for the DraftRoom microchain
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Query the DraftRoom application state on the specified microchain
        // This returns the serialized LiveDraftArena::DraftRoom state
        match self.client.query_application(chain_id, self.app_id).await {
            Ok(response) => {
                info!("Player {} successfully queried DraftRoom state, deserializing...", player_id);
                
                // Deserialize the DraftRoom state
                match self.deserialize_draft_room_state(&response, chain_id).await {
                    Ok(Some(room_data)) => {
                        // Convert to GraphQL response type
                        let room_state = DraftRoomState {
                            chain_id: room_data.chain_id.to_string(),
                            players: room_data.players,
                            max_players: room_data.max_players,
                            current_turn: room_data.current_turn,
                            round: room_data.round,
                            max_rounds: room_data.max_rounds,
                            pool: room_data.pool,
                            status: room_data.status,
                        };
                        
                        info!("Player {} successfully retrieved DraftRoom state for chain {}", player_id, chain_id);
                        Ok(Some(room_state))
                    }
                    Ok(None) => {
                        warn!("Player {} found no DraftRoom state for chain {}", player_id, chain_id);
                        Ok(None)
                    }
                    Err(e) => {
                        error!("Player {} failed to deserialize DraftRoom state for chain {}: {}", player_id, chain_id, e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Player {} failed to query DraftRoom state for chain {}: {}", player_id, chain_id, e);
                Err(async_graphql::Error::new(format!("Failed to query DraftRoom: {}", e)))
            }
        }
    }

    /// Get current user's picks in a room
    /// 
    /// This queries the DraftRoom state and extracts the picks MapView<Owner, Vec<DraftItem>>
    /// to return only the cards picked by the current player.
    async fn my_picks(&self, ctx: &Context<'_>, chain_id: String) -> Result<Vec<crate::types::DraftItem>> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        let player_owner = context.get_player_owner();
        
        info!("Player {} querying their picks in DraftRoom {}", player_id, chain_id);

        // Parse chain ID for the DraftRoom microchain
        let chain_id = chain_id.parse::<ChainId>()
            .map_err(|e| async_graphql::Error::new(format!("Invalid chain ID: {}", e)))?;

        // Query the DraftRoom application state to access the picks MapView
        match self.client.query_application(chain_id, self.app_id).await {
            Ok(response) => {
                info!("Player {} successfully queried DraftRoom for picks, extracting player data...", player_id);
                
                // Extract picks for the specific player from the MapView<Owner, Vec<DraftItem>>
                match self.extract_player_picks(&response, player_owner).await {
                    Ok(picks) => {
                        info!("Player {} successfully retrieved {} picks from DraftRoom {}", player_id, picks.len(), chain_id);
                        Ok(picks)
                    }
                    Err(e) => {
                        error!("Player {} failed to extract picks from DraftRoom {}: {}", player_id, chain_id, e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Player {} failed to query DraftRoom {} for picks: {}", player_id, chain_id, e);
                Err(async_graphql::Error::new(format!("Failed to query picks: {}", e)))
            }
        }
    }

    /// Get player information (for debugging/display)
    async fn player_info(&self, ctx: &Context<'_>) -> Result<String> {
        let context = get_context(ctx);
        let player_id = context.get_player_id();
        let player_owner = context.get_player_owner();
        
        Ok(format!(
            "Player ID: {} | Owner: {}",
            player_id,
            player_owner
        ))
    }

    /// Health check endpoint
    async fn health(&self) -> Result<String> {
        Ok("Service is running".to_string())
    }
}