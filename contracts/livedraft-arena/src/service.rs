use async_graphql::{Request, Response, Schema, SimpleObject};
use linera_sdk::{Service, ServiceRuntime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::LiveDraftArena;

/// GraphQL service
pub struct LiveDraftArenaService {
    state: Arc<LiveDraftArena>,
}

/// Room data for GraphQL responses
#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct RoomData {
    pub chain_id: String,
    pub room_name: String,
    pub max_players: u8,
    pub status: String,
}

/// GraphQL query root
pub struct QueryRoot {
    state: Arc<LiveDraftArena>,
}

#[async_graphql::Object]
impl QueryRoot {
    /// Get all draft rooms
    async fn rooms(&self) -> Vec<RoomData> {
        let mut rooms = Vec::new();
        
        if let Ok(iter) = self.state.rooms.iter().await {
            for (chain_id, metadata) in iter {
                rooms.push(RoomData {
                    chain_id: chain_id.to_string(),
                    room_name: metadata.room_name,
                    max_players: metadata.max_players,
                    status: format!("{:?}", metadata.status),
                });
            }
        }
        
        rooms
    }
}

impl Service for LiveDraftArenaService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = LiveDraftArena::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        
        LiveDraftArenaService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
            },
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .finish();

        schema.execute(request).await
    }
}