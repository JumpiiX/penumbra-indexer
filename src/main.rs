mod grpc_client;
mod db;
mod api;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // TODO: Initialize components and start the service

    Ok(())
}