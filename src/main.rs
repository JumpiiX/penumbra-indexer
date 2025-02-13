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
*
* @module main
* @requires tokio
* @requires tracing
* @requires dotenv
*/
mod client;
mod db;
mod api;
mod models;

use std::error::Error;
use std::env;
use dotenv::dotenv;
use tokio::time::Duration;
use client::PenumbraClient;

/*
* Main application entry point.
*
* Orchestrates startup sequence:
* 1. Initialize logging
* 2. Load configuration
* 3. Connect to database
* 4. Start API server
* 5. Start block indexing process
*
* @async
* @returns {Result} Initialization result
* @throws {Error} Configuration or service startup failures
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Starting Penumbra Indexer...");

    // Load environment variables
    dotenv().ok();

    // Use PORT environment variable for Railway compatibility
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid port number");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let rpc_url = env::var("RPC_URL")
        .unwrap_or_else(|_| "http://grpc.penumbra.silentvalidator.com:26657".to_string());

    println!("Connecting to database at {}", database_url);

    // Retry database connection with backoff
    let pool = loop {
        match db::init_db(&database_url).await {
            Ok(pool) => break pool,
            Err(e) => {
                eprintln!("Database connection failed: {}. Retrying in 5 seconds...", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    };

    println!("Database initialized successfully");

    // Start API server
    let app = api::create_router(pool.clone());
    let api_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await.unwrap();
        println!("API server listening on port {}", port);
        axum::serve(listener, app).await.unwrap();
    });

    println!("Starting block indexer...");
    // Start block indexer
    let indexer_handle = tokio::spawn({
        let pool = pool.clone();
        async move {
            let client = loop {
                match PenumbraClient::connect(&rpc_url, pool.clone()).await {
                    Ok(client) => break client,
                    Err(e) => {
                        eprintln!("Failed to connect to Penumbra node: {}. Retrying in 10 seconds...", e);
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    }
                }
            };

            println!("Connected to Penumbra node at {}", rpc_url);

            loop {
                match client.get_status().await {
                    Ok(status) => {
                        let current_height: u64 = status.result.sync_info.latest_block_height
                            .parse()
                            .unwrap_or(0);
                        let start_height = if current_height > 10 {
                            current_height - 10
                        } else {
                            0
                        };

                        println!("Fetching blocks {} to {}", start_height, current_height);
                        if let Err(e) = client.fetch_blocks(start_height, current_height, 5).await {
                            eprintln!("Error fetching blocks: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting node status: {}", e);
                    }
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });

    println!("All services started successfully");

    // Wait for both tasks
    tokio::try_join!(api_handle, indexer_handle)?;

    Ok(())
}