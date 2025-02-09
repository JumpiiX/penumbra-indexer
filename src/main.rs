mod proto;
mod client;

use anyhow::Result;
use crate::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // Try different endpoints
    let endpoints = [
        "https://penumbra.stakewith.binary.builders:443",
        "http://penumbra.stakewith.binary.builders:8080",
        "https://penumbra.stakewith.binary.builders",
        "http://penumbra.stakewith.binary.builders",
    ];

    let mut connected = false;
    let mut last_error = None;

    for endpoint in endpoints {
        println!("Trying to connect to {}", endpoint);
        match Client::connect(endpoint).await {
            Ok(mut client) => {
                println!("Connected successfully to {}", endpoint);
                connected = true;
                client.get_latest_blocks().await?;
                break;
            }
            Err(e) => {
                println!("Failed to connect to {}: {}", endpoint, e);
                last_error = Some(e);
            }
        }
    }

    if !connected {
        if let Some(e) = last_error {
            return Err(e);
        }
    }

    Ok(())
}