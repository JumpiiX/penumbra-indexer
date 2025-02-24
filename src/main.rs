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
    let database_url = env::var("DB_URL").expect("DATABASE_URL must be set");
    let rpc_url = env::var("RPC_URL")
        .unwrap_or_else(|_| "http://grpc.penumbra.silentvalidator.com:26657".to_string());
    let api_port = env::var("API_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("API_PORT must be a valid port number");
    let batch_size = env::var("BATCH_SIZE")
        .unwrap_or_else(|_| DEFAULT_BATCH_SIZE.to_string())
        .parse::<u64>()
        .unwrap_or(DEFAULT_BATCH_SIZE);

    println!("Connecting to database at {}", database_url);
    time::sleep(Duration::from_secs(5)).await;

    let pool = db::init_db(&database_url).await?;
    println!("Database initialized successfully");

    let app = api::create_router(pool.clone());
    let api_handle = tokio::spawn(async move {
        let listener = TcpListener::bind(("0.0.0.0", api_port)).await.unwrap();
        println!("API server listening on port {}", api_port);
        axum::serve(listener, app).await.unwrap();
    });

    println!("Starting block indexer...");
    let indexer_handle = tokio::spawn({
        let pool = pool.clone();
        async move {
            let client = match PenumbraClient::connect(&rpc_url, pool).await {
                Ok(client) => {
                    println!("Connected to Penumbra node at {}", rpc_url);
                    client
                },
                Err(e) => {
                    eprintln!("Failed to connect to Penumbra node: {}", e);
                    return;
                }
            };

            println!("Performing initial blockchain synchronization from genesis...");
            if let Err(e) = client.sync_from_genesis(batch_size).await {
                eprintln!("Error during initial sync: {}", e);
            }

            let mut last_processed_block: Option<u64> = None;

            loop {
                match client.get_status().await {
                    Ok(status) => {
                        let latest_height: u64 = status.result.sync_info.latest_block_height
                            .parse()
                            .unwrap_or(0);

                        if Some(latest_height) != last_processed_block {
                            if let Err(e) = client.fetch_blocks(latest_height, latest_height, 5).await {
                                eprintln!("Error fetching latest block: {}", e);
                            }
                            last_processed_block = Some(latest_height);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting node status: {}", e);
                    }
                }
            }
        }
    });

    println!("All services started successfully");
    tokio::try_join!(api_handle, indexer_handle)?;

    Ok(())
}