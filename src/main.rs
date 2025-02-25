/*
 * Penumbra Indexer - Main Application Entry Point
 *
 * Coordinates initialization and startup of core services:
 * - Environment configuration
 * - Database connection
 * - API server
 * - Block indexing client
 *
 * Manages concurrent tasks for API and block synchronization
 * using Tokio async runtime.
 */

mod db;
mod api;
mod models;
mod client;
mod error;

use std::error::Error;
use std::env;
use std::time::Duration;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tokio::time;
use tracing::{info, error, warn};
use crate::client::PenumbraClient;

const DEFAULT_BATCH_SIZE: u64 = 100;

/*
 * Main application entry point.
 *
 * Orchestrates startup sequence:
 * 1. Initialize logging
 * 2. Load configuration
 * 3. Connect to database
 * 4. Start API server
 * 5. Start block indexing process
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    println!("Starting Penumbra Indexer...");

    dotenv().ok();

    let database_url = env::var("DB_URL").expect("DB_URL must be set");
    println!("Database URL: {}", database_url);

    let rpc_url = env::var("RPC_URL")
        .unwrap_or_else(|_| {
            let default = "http://grpc.penumbra.silentvalidator.com:26657".to_string();
            println!("RPC_URL not set, using default: {}", default);
            default
        });

    let api_port = env::var("API_PORT")
        .unwrap_or_else(|_| {
            println!("API_PORT not set, using default: 3000");
            "3000".to_string()
        })
        .parse::<u16>()
        .expect("API_PORT must be a valid port number");

    let batch_size = env::var("BATCH_SIZE")
        .unwrap_or_else(|_| {
            println!("BATCH_SIZE not set, using default: {}", DEFAULT_BATCH_SIZE);
            DEFAULT_BATCH_SIZE.to_string()
        })
        .parse::<u64>()
        .unwrap_or(DEFAULT_BATCH_SIZE);

    println!("Configuration loaded successfully");

    println!("Waiting for database to be ready...");
    let mut retry_count = 0;
    let max_retries = 10;
    let mut pool = None;

    while retry_count < max_retries {
        match db::init_db(&database_url).await {
            Ok(p) => {
                pool = Some(p);
                println!("✅ Database connection established successfully");
                break;
            },
            Err(e) => {
                retry_count += 1;
                println!("Database connection attempt {}/{} failed: {}", retry_count, max_retries, e);
                if retry_count < max_retries {
                    let wait_time = 2 * retry_count;
                    println!("Retrying in {} seconds...", wait_time);
                    time::sleep(Duration::from_secs(wait_time)).await;
                }
            }
        }
    }

    let pool = match pool {
        Some(p) => p,
        None => {
            println!("❌ Failed to connect to database after {} attempts. Exiting...", max_retries);
            return Err("Failed to connect to database".into());
        }
    };

    println!("Creating API router...");
    let app = api::create_router(pool.clone());

    println!("Starting API server on port {}", api_port);
    let api_handle = tokio::spawn(async move {
        match TcpListener::bind(("0.0.0.0", api_port)).await {
            Ok(listener) => {
                println!("API server listening on port {}", api_port);
                if let Err(e) = axum::serve(listener, app).await {
                    println!("API server error: {}", e);
                }
            },
            Err(e) => {
                println!("Failed to bind API server to port {}: {}", api_port, e);
            }
        }
    });

    println!("Starting block indexer...");
    let indexer_handle = tokio::spawn({
        let pool = pool.clone();
        async move {
            println!("Connecting to Penumbra node at {}", rpc_url);
            let client = match PenumbraClient::connect(&rpc_url, pool).await {
                Ok(client) => {
                    println!("✅ Connected to Penumbra node");
                    client
                },
                Err(e) => {
                    println!("❌ Failed to connect to Penumbra node: {}", e);
                    return;
                }
            };

            println!("Starting blockchain synchronization with batch size: {}", batch_size);
            if let Err(e) = client.sync_from_genesis(batch_size).await {
                println!("Error during initial sync: {}", e);
            }

            let mut last_processed_block: Option<u64> = None;

            println!("Entering synchronization loop");
            loop {
                match client.get_status().await {
                    Ok(status) => {
                        let latest_height: u64 = status.result.sync_info.latest_block_height
                            .parse()
                            .unwrap_or(0);

                        if Some(latest_height) != last_processed_block {
                            println!("Processing new block at height {}", latest_height);
                            if let Err(e) = client.fetch_blocks(latest_height, latest_height, 5).await {
                                println!("Error fetching block {}: {}", latest_height, e);
                            }
                            last_processed_block = Some(latest_height);
                        }
                    }
                    Err(e) => {
                        println!("Error getting node status: {}", e);
                    }
                }
            }
        }
    });

    println!("All services started successfully - running indefinitely");

    tokio::select! {
        result = api_handle => {
            if let Err(e) = result {
                println!("API server task failed: {}", e);
            } else {
                println!("API server task completed unexpectedly");
            }
        },
        result = indexer_handle => {
            if let Err(e) = result {
                println!("Indexer task failed: {}", e);
            } else {
                println!("Indexer task completed unexpectedly");
            }
        }
    }

    println!("One of the critical tasks has terminated unexpectedly - application will now exit");
    Err("Critical service terminated".into())
}
