mod grpc_client;
mod db;
mod api;

use anyhow::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Starting Penumbra indexer...");

    // Initialize database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/penumbra".to_string());

    let db = db::Database::connect(&database_url).await?;

    // Initialize gRPC client
    let mut client = grpc_client::GrpcClient::connect(
        "https://penumbra.stakewith.binary.builders"
    ).await?;

    // Initialize API
    let api = api::Api::new(db);
    let routes = api.routes();

    // Start the background indexing task
    tokio::spawn(async move {
        loop {
            if let Ok(block) = client.get_latest_block().await {
                println!("Fetched block: {}", block.height);
                // Process and store block
                // ... (implement this part)
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // Start the API server
    println!("Starting API server on 0.0.0.0:3000");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3000))
        .await;

    Ok(())
}