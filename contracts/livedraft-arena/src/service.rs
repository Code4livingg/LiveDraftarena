use async_graphql::{Request, Response, Schema};
use linera_sdk::{
    base::ChainId,
    views::View,
    Service, ServiceRuntime,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{DraftRoomMetadata, Lobby};

/// GraphQL service for the Lobby.
pub struct LobbyService {
    state: Arc<Lobby>,
}

/// Room data for GraphQL responses.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoomData {
    pub chain_id: ChainId,
    pub metadata: DraftRoomMetadata,
}

/// GraphQL query root.
pub struct QueryRoot {
    state: Arc<Lobby>,
}

#[async_graphql::Object]
impl QueryRoot {
    /// Get all draft rooms.
    async fn rooms(&self) -> Vec<RoomData> {
        let mut rooms = Vec::new();
        
        // Iterate through all rooms in the MapView
        for (chain_id, metadata) in self.state.rooms.iter().await.unwrap() {
            rooms.push(RoomData {
                chain_id,
                metadata,
            });
        }
        
        rooms
    }
}

impl Service for LobbyService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Lobby::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load lobby state");
        
        LobbyService {
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