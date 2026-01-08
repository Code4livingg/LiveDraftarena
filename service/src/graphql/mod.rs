mod query;
mod mutation;

pub use query::QueryRoot;
pub use mutation::MutationRoot;

use async_graphql::Context;
use linera_core::data_types::Owner;
use crate::identity::player_id_to_owner;

/// GraphQL context containing player identity information
/// 
/// This context is created for each request and contains the player's
/// identity information derived from HTTP headers/cookies.
#[derive(Clone)]
pub struct GraphQLContext {
    /// Unique player ID for this session
    pub player_id: String,
    /// Linera Owner address derived from player ID
    pub player_owner: Owner,
}

impl GraphQLContext {
    /// Create new GraphQL context with player identity
    pub fn new(player_id: String) -> Self {
        let player_owner = player_id_to_owner(&player_id)
            .expect("Failed to create Owner from player ID");
        
        Self {
            player_id,
            player_owner,
        }
    }
    
    /// Get player ID from GraphQL context
    pub fn get_player_id(&self) -> &str {
        &self.player_id
    }
    
    /// Get Linera Owner for this player
    pub fn get_player_owner(&self) -> &Owner {
        &self.player_owner
    }
}

/// Helper function to extract GraphQL context from async-graphql Context
pub fn get_context(ctx: &Context<'_>) -> &GraphQLContext {
    ctx.data_unchecked::<GraphQLContext>()
}