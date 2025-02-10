mod client;

use std::error::Error;
use client::PenumbraClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "http://grpc.penumbra.silentvalidator.com:26657";

    println!("Connecting to {}", addr);
    let client = PenumbraClient::connect(addr).await?;

    let current_height = 3456307;
    let start_height = current_height - 10;
    let batch_size = 5;

    println!("Starting block fetching from {} to {}", start_height, current_height);
    client.fetch_blocks(start_height, current_height, batch_size).await?;

    Ok(())
}