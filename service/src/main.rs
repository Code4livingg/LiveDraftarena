use anyhow::{Context, Result};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::GraphQLBadRequest;
use linera_client::{ClientContext, Options as ClientOptions};
use linera_core::data_types::{ApplicationId, ChainId};
use std::convert::Infallible;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{info, warn};
use warp::{http::Response as HttpResponse, Filter, Rejection, Reply};

mod graphql;
mod types;
mod identity;

use graphql::{MutationRoot, QueryRoot, GraphQLContext};
use identity::{extract_player_id, create_player_id_cookie};

/// Conway testnet configuration
const CONWAY_TESTNET_ENDPOINT: &str = "https://conway-testnet.linera.net:8080";

// ============================================================================
// PRODUCTION CONFIGURATION SECTION
// ============================================================================
// The following functions handle production-ready configuration:
// - Network binding to 0.0.0.0 for public deployment
// - Configurable CORS origins for security
// - Structured logging for monitoring
// - Environment-based configuration management
// ============================================================================

/// Default wallet path (can be overridden by environment variable)
fn default_wallet_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config")
        .join("linera")
        .join("wallet.json")
}

/// Load Linera client with wallet from disk
/// 
/// This loads the actual Linera wallet and connects to Conway testnet.
/// The wallet must be initialized with `linera wallet init` first.
async fn load_linera_client() -> Result<ClientContext> {
    // Get wallet path from environment or use default
    let wallet_path = std::env::var("LINERA_WALLET_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| default_wallet_path());

    info!("Loading Linera wallet from: {}", wallet_path.display());

    // Verify wallet file exists
    if !wallet_path.exists() {
        anyhow::bail!(
            "Wallet file not found at {}. Please run 'linera wallet init' first.",
            wallet_path.display()
        );
    }

    // Create client options for Conway testnet
    let options = ClientOptions {
        wallet_path: Some(wallet_path),
        endpoint: Some(CONWAY_TESTNET_ENDPOINT.to_string()),
        ..Default::default()
    };

    info!("Connecting to Conway testnet: {}", CONWAY_TESTNET_ENDPOINT);

    // Load client context - this connects to the network and loads the wallet
    let client_context = ClientContext::new(options)
        .await
        .context("Failed to create Linera client context. Ensure wallet is initialized and Conway testnet is accessible.")?;

    info!("Successfully connected to Conway testnet and loaded wallet");
    
    Ok(client_context)
}

/// Get application ID from environment variable or .env file
/// 
/// This reads the deployed Application ID from:
/// 1. LIVEDRAFT_APP_ID environment variable (highest priority)
/// 2. service/.env file (created by deployment script)
/// 
/// The same Application ID is used for both Lobby and DraftRoom operations.
fn get_application_id() -> Result<ApplicationId> {
    // Try environment variable first (for manual override)
    if let Ok(app_id_str) = std::env::var("LIVEDRAFT_APP_ID") {
        info!("Using Application ID from environment: {}", app_id_str);
        return ApplicationId::from_str(&app_id_str)
            .context("Invalid Application ID format in LIVEDRAFT_APP_ID");
    }
    
    // Try to load from .env file (created by deployment script)
    let env_file_path = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".env");
    
    if env_file_path.exists() {
        info!("Loading Application ID from .env file: {}", env_file_path.display());
        
        let env_content = std::fs::read_to_string(&env_file_path)
            .context("Failed to read .env file")?;
        
        for line in env_content.lines() {
            let line = line.trim();
            if line.starts_with("LIVEDRAFT_APP_ID=") {
                let app_id_str = line.strip_prefix("LIVEDRAFT_APP_ID=")
                    .unwrap_or("")
                    .trim_matches('"')
                    .trim();
                
                if !app_id_str.is_empty() {
                    info!("Found Application ID in .env file: {}", app_id_str);
                    return ApplicationId::from_str(app_id_str)
                        .context("Invalid Application ID format in .env file");
                }
            }
        }
    }
    
    // If neither source is available, provide helpful error
    anyhow::bail!(
        "LIVEDRAFT_APP_ID not found. Please either:\n\
         1. Set environment variable: export LIVEDRAFT_APP_ID=your-app-id\n\
         2. Run deployment script: ./scripts/deploy_conway.sh\n\
         3. Create .env file with: LIVEDRAFT_APP_ID=your-app-id"
    )
}

/// Get default chain ID from environment variable or client wallet
/// 
/// This gets the active chain from the loaded wallet, which is where
/// the Lobby contract should be deployed.
async fn get_default_chain_id(client: &ClientContext) -> Result<ChainId> {
    // Try to get from environment first (for explicit override)
    if let Ok(chain_id_str) = std::env::var("LIVEDRAFT_CHAIN_ID") {
        info!("Using chain ID from environment: {}", chain_id_str);
        return ChainId::from_str(&chain_id_str)
            .context("Invalid chain ID format in LIVEDRAFT_CHAIN_ID");
    }

    // Get the default chain from the wallet
    // This is typically the first chain in the wallet or the active chain
    let default_chain = client.default_chain()
        .await
        .context("Failed to get default chain from wallet. Ensure wallet has at least one chain.")?;

    info!("Using default chain from wallet: {}", default_chain);
    Ok(default_chain)
}

/// Handle GraphQL requests with player identity context
/// 
/// This is the core request handler that:
/// 1. Extracts player identity from HTTP headers/cookies
/// 2. Creates GraphQL context with player's Linera Owner
/// 3. Executes GraphQL operations with proper authentication
/// 4. Returns response with Set-Cookie for session persistence
async fn graphql_handler(
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
    headers: warp::http::HeaderMap,
    request: async_graphql::Request,
) -> Result<impl Reply, Rejection> {
    // Extract or generate player ID from request headers/cookies
    // This creates a deterministic Linera Owner address for the player
    let player_id = extract_player_id(&headers);
    
    info!("Processing GraphQL request for player: {} (Owner will be derived)", player_id);
    
    // Create GraphQL context with player identity
    // The context contains both the player ID and the derived Linera Owner
    let context = GraphQLContext::new(player_id.clone());
    
    // Execute GraphQL request with player context
    // All mutations will use the player's Owner for signing operations
    // All queries will have access to the player's identity for filtering
    let response = schema.execute(request.data(context)).await;
    
    // Create response with Set-Cookie header for player ID persistence
    // This ensures the same browser maintains the same Linera identity
    let cookie_header = create_player_id_cookie(&player_id);
    
    Ok(warp::reply::with_header(
        async_graphql_warp::Response::from(response),
        "Set-Cookie",
        cookie_header,
    ))
}

/// Handle GraphQL errors
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(GraphQLBadRequest(err)) = err.find() {
        return Ok(HttpResponse::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(format!(r#"{{"error": "{}"}}"#, err)));
    }

    warn!("Unhandled rejection: {:?}", err);
    Ok(HttpResponse::builder()
        .status(500)
        .header("content-type", "application/json")
        .body(r#"{"error": "Internal server error"}"#))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Production logging configuration
    // Set log level from environment (defaults to info)
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    
    // Configure structured logging for production
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(log_level.clone())
        .with_target(false)  // Remove module paths in production
        .with_thread_ids(false)  // Remove thread IDs for cleaner logs
        .compact();  // Use compact format for production
    
    // Initialize logging
    subscriber.init();
    
    info!("Starting LiveDraft Arena service with real Linera integration...");
    info!("🔗 Conway Testnet: {}", CONWAY_TESTNET_ENDPOINT);
    info!("👥 Multi-user: Each browser gets unique Linera Owner identity");
    info!("⚡ Real-time: All operations execute on-chain with immediate confirmation");
    info!("📊 Log Level: {}", log_level);

    // Load Linera client and configuration
    let client = load_linera_client().await?;
    let app_id = get_application_id()?;
    let default_chain_id = get_default_chain_id(&client).await?;

    info!("Application ID: {}", app_id);
    info!("Default Chain ID (Lobby): {}", default_chain_id);
    info!("🏛️  Lobby operations will execute on chain: {}", default_chain_id);
    info!("🏠 DraftRoom operations will execute on individual microchains");

    // Create GraphQL schema
    let schema = Schema::build(
        QueryRoot::new(client.clone(), app_id, default_chain_id),
        MutationRoot::new(client, app_id, default_chain_id),
        EmptySubscription,
    )
    .finish();

    // Create GraphQL endpoint with player identity handling
    let graphql_route = warp::path("graphql")
        .and(warp::post())
        .and(warp::headers_cloned()) // Extract headers for player ID
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(move |headers, request| {
            graphql_handler(schema.clone(), headers, request)
        });

    // Create GraphQL playground (for development)
    let playground_route = warp::path("playground")
        .and(warp::get())
        .map(|| {
            HttpResponse::builder()
                .header("content-type", "text/html")
                .body(async_graphql::http::playground_source(
                    async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
                ))
        });

    // Health check endpoint
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    // Production CORS configuration
    // Allow specific origins in production, any origin in development
    let cors_origins = std::env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "*".to_string());
    
    let cors = if cors_origins == "*" {
        info!("🌐 CORS: Allowing all origins (development mode)");
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "x-player-id", "cookie"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"])
    } else {
        info!("🌐 CORS: Allowing specific origins: {}", cors_origins);
        let origins: Vec<&str> = cors_origins.split(',').map(|s| s.trim()).collect();
        warp::cors()
            .allow_origins(origins)
            .allow_headers(vec!["content-type", "x-player-id", "cookie"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"])
    };

    // Combine all routes
    let routes = graphql_route
        .or(playground_route)
        .or(health_route)
        .with(cors)
        .recover(handle_rejection);

    // Production server configuration
    // Bind to 0.0.0.0 for public deployment (not localhost)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .context("Invalid PORT environment variable")?;

    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    info!("🚀 LiveDraft Arena service ready!");
    info!("🌐 Binding to: {}:{}", bind_address, port);
    info!("GraphQL endpoint: http://{}:{}/graphql", bind_address, port);
    info!("GraphQL playground: http://{}:{}/playground", bind_address, port);
    info!("Health check: http://{}:{}/health", bind_address, port);
    info!("🔐 Multi-user identity: Cookie + header based");
    info!("⛓️  Linera integration: Real operations on Conway testnet");
    info!("📊 Each mutation creates actual blockchain transactions");
    info!("🔍 Each query reads live on-chain state");

    // Parse bind address for warp
    let bind_addr: std::net::IpAddr = bind_address.parse()
        .context("Invalid BIND_ADDRESS format")?;

    warp::serve(routes)
        .run((bind_addr, port))
        .await;

    Ok(())
}