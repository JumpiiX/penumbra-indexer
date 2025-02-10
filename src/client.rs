use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

#[derive(Debug)]
pub struct PenumbraClient {
    client: HttpClient,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct BlockResponse {
    result: BlockResult,
}

#[derive(Debug, Deserialize)]
struct BlockResult {
    block: Block,
}

#[derive(Debug, Deserialize)]
struct Block {
    header: BlockHeader,
    data: BlockData,
}

#[derive(Debug, Deserialize)]
struct BlockHeader {
    height: String,
    time: String,
    last_block_id: Option<BlockId>,
}

#[derive(Debug, Deserialize)]
struct BlockId {
    hash: String,
}

#[derive(Debug, Deserialize)]
struct BlockData {
    txs: Option<Vec<String>>,
}

impl PenumbraClient {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn Error>> {
        println!("Attempting to connect with RPC config...");

        let client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(30))
            .build()?;

        println!("HTTP client created successfully");

        Ok(Self {
            client,
            base_url: addr.to_string(),
        })
    }

    pub async fn fetch_blocks(
        &self,
        start_height: u64,
        end_height: u64,
        batch_size: u64,
    ) -> Result<(), Box<dyn Error>> {
        let mut current_height = start_height;

        while current_height <= end_height {
            let batch_end = std::cmp::min(current_height + batch_size, end_height);

            println!("Fetching blocks {} to {}", current_height, batch_end);

            for height in current_height..=batch_end {
                match self.fetch_block(height).await {
                    Ok(block) => {
                        println!("Block {}", height);
                        println!("  Time: {}", block.result.block.header.time);
                        if let Some(last_block) = &block.result.block.header.last_block_id {
                            println!("  Previous block hash: {}", last_block.hash);
                        }
                        if let Some(txs) = &block.result.block.data.txs {
                            println!("  Transaction count: {}", txs.len());
                        }
                        println!("-------------------");
                    }
                    Err(e) => {
                        eprintln!("Error fetching block {}: {}", height, e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            }

            current_height = batch_end + 1;
        }

        Ok(())
    }

    async fn fetch_block(&self, height: u64) -> Result<BlockResponse, Box<dyn Error>> {
        let url = format!("{}/block?height={}", self.base_url, height);

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}