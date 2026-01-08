use anyhow::Result;
use linera_core::data_types::Owner;
use sha2::{Digest, Sha256};
use std::str::FromStr;
use warp::http::HeaderMap;

/// Player identity management for multi-user sessions
/// 
/// Each browser session gets a deterministic player ID that maps to a Linera Owner.
/// This allows multiple users to play simultaneously without authentication.

const PLAYER_ID_HEADER: &str = "x-player-id";
const PLAYER_ID_COOKIE: &str = "livedraft_player_id";

/// Generate a deterministic Linera Owner from a player ID
/// 
/// This creates a consistent Owner address for each player session.
/// The same player ID will always generate the same Owner address.
/// 
/// Process:
/// 1. Hash "livedraft_player_" + player_id with SHA256
/// 2. Convert hash to hex string (64 characters)
/// 3. Parse as Linera Owner address
/// 
/// This ensures each browser session has a unique, persistent Linera identity
/// that can sign transactions and own assets on the Linera network.
pub fn player_id_to_owner(player_id: &str) -> Result<Owner> {
    // Create a deterministic hash from the player ID
    let mut hasher = Sha256::new();
    hasher.update(b"livedraft_player_");
    hasher.update(player_id.as_bytes());
    let hash = hasher.finalize();
    
    // Convert hash to hex string (64 characters for Owner)
    let owner_str = format!("{:x}", hash);
    
    // Parse as Owner (this creates a valid Linera Owner address)
    Owner::from_str(&owner_str)
        .map_err(|e| anyhow::anyhow!("Failed to create Owner from player ID: {}", e))
}

/// Extract player ID from HTTP request headers or cookies
/// 
/// Priority:
/// 1. x-player-id header (for explicit player identification)
/// 2. livedraft_player_id cookie (for browser persistence)
/// 3. Generate new player ID if none found
pub fn extract_player_id(headers: &HeaderMap) -> String {
    // Try to get player ID from header first
    if let Some(header_value) = headers.get(PLAYER_ID_HEADER) {
        if let Ok(player_id) = header_value.to_str() {
            if !player_id.is_empty() && is_valid_player_id(player_id) {
                return player_id.to_string();
            }
        }
    }
    
    // Try to get player ID from cookie
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some(value) = cookie.strip_prefix(&format!("{}=", PLAYER_ID_COOKIE)) {
                    if !value.is_empty() && is_valid_player_id(value) {
                        return value.to_string();
                    }
                }
            }
        }
    }
    
    // Generate new player ID if none found
    generate_player_id()
}

/// Generate a new random player ID
/// 
/// Creates a unique identifier for a new player session.
/// This is deterministic based on current timestamp and random data.
fn generate_player_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    
    // Create a hash from timestamp and some entropy
    let mut hasher = Sha256::new();
    hasher.update(b"player_session_");
    hasher.update(timestamp.to_be_bytes());
    hasher.update(std::process::id().to_be_bytes()); // Add process ID for uniqueness
    
    let hash = hasher.finalize();
    
    // Take first 16 characters of hex for a shorter player ID
    format!("{:x}", hash)[..16].to_string()
}

/// Validate player ID format
/// 
/// Ensures player IDs are safe and consistent.
fn is_valid_player_id(player_id: &str) -> bool {
    // Must be 16 hex characters
    player_id.len() == 16 && player_id.chars().all(|c| c.is_ascii_hexdigit())
}

/// Create a Set-Cookie header value for player ID persistence
/// 
/// This allows browsers to maintain the same player ID across refreshes.
pub fn create_player_id_cookie(player_id: &str) -> String {
    format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        PLAYER_ID_COOKIE,
        player_id,
        60 * 60 * 24 * 30 // 30 days
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_player_id_to_owner_deterministic() {
        let player_id = "1234567890abcdef";
        let owner1 = player_id_to_owner(player_id).unwrap();
        let owner2 = player_id_to_owner(player_id).unwrap();
        assert_eq!(owner1, owner2);
    }
    
    #[test]
    fn test_different_player_ids_different_owners() {
        let owner1 = player_id_to_owner("1234567890abcdef").unwrap();
        let owner2 = player_id_to_owner("fedcba0987654321").unwrap();
        assert_ne!(owner1, owner2);
    }
    
    #[test]
    fn test_valid_player_id() {
        assert!(is_valid_player_id("1234567890abcdef"));
        assert!(!is_valid_player_id("invalid"));
        assert!(!is_valid_player_id("123")); // too short
    }
}