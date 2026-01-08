use anyhow::{Context, Result};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::GraphQLBadRequest;
use linera_client::{Client, ClientOptions};
use linera_core::data_types::{ApplicationId, ChainId};
use std::convert::Infallible;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{info, warn};
use warp::{http::Response as HttpResponse, Filter, Rejection, Reply};

mod graphql;
mod types;

use graphql::{MutationRoot, QueryRoot};

/// Conway testnet configuration
const CONWAY_TESTNET_ENDPOINT: &str = "https://conway-testnet.linera.net:8080";

/// Default wallet path (can be overridden by environment variable)
fn default_wallet_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config")
        .join("linera")
        .join("wallet.json")
}

/// Load Linera client with wallet from disk
async fn load_linera_client() -> Result<Client> {
    // Get wallet path from environment or use default
    let wallet_path = std::env::var("LINERA_WALLET_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| default_wallet_path());

    info!("Loading Linera wallet from: {}", wallet_path.display());

    // Verify wallet file exists
    if !wallet_path.exists() {
        anyhow::bail!(
            "Wallet file not found at {}. Please ensure linera CLI is initialized.",
            wallet_path.display()
        );
    }

    // Create client options for Conway testnet
    let options = ClientOptions::new()
        .with_wallet_path(wallet_path)
        .with_endpoint(CONWAY_TESTNET_ENDPOINT.to_string());

    // Load client
    let client = Client::new(options)
        .await
        .context("Failed to create Linera client")?;

    info!("Successfully connected to Conway testnet: {}", CONWAY_TESTNET_ENDPOINT);
    
    Ok(client)
}

/// Get application ID from environment variable
fn get_application_id() -> Result<ApplicationId> {
    let app_id_str = std::env::var("LIVEDRAFT_APP_ID")
        .context("LIVEDRAFT_APP_ID environment variable not set")?;
    
    ApplicationId::from_str(&app_id_str)
        .context("Invalid application ID format")
}

/// Get default chain ID from environment variable or client
async fn get_default_chain_id(client: &Client) -> Result<ChainId> {
    // Try to get from environment first
    if let Ok(chain_id_str) = std::env::var("LIVEDRAFT_CHAIN_ID") {
        return ChainId::from_str(&chain_id_str)
            .context("Invalid chain ID format in LIVEDRAFT_CHAIN_ID");
    }

    // Otherwise, get default chain from client
    let chains = client.get_chains().await
        .context("Failed to get chains from client")?;
    
    chains.into_iter()
        .next()
        .context("No chains available in wallet")
}

/// Handle GraphQL requests
async fn graphql_handler(
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
    request: async_graphql::Request,
) -> Result<impl Reply, Rejection> {
    let response = schema.execute(request).await;
    Ok(async_graphql_warp::Response::from(response))
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
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting LiveDraft Arena service...");

    // Load Linera client and configuration
    let client = load_linera_client().await?;
    let app_id = get_application_id()?;
    let default_chain_id = get_default_chain_id(&client).await?;

    info!("Application ID: {}", app_id);
    info!("Default Chain ID: {}", default_chain_id);

    // Create GraphQL schema
    let schema = Schema::build(
        QueryRoot::new(client.clone(), app_id, default_chain_id),
        MutationRoot::new(client, app_id, default_chain_id),
        EmptySubscription,
    )
    .finish();

    // Create GraphQL endpoint
    let graphql_route = warp::path("graphql")
        .and(warp::post())
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(|request| graphql_handler(schema.clone(), request));

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

    // Combine all routes
    let routes = graphql_route
        .or(playground_route)
        .or(health_route)
        .with(warp::cors().allow_any_origin().allow_headers(vec!["content-type"]))
        .recover(handle_rejection);

    // Start server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .context("Invalid PORT environment variable")?;

    info!("GraphQL endpoint: http://localhost:{}/graphql", port);
    info!("GraphQL playground: http://localhost:{}/playground", port);
    info!("Health check: http://localhost:{}/health", port);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;

    Ok(())
}