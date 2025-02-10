use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use crate::models::StoredBlock;

#[derive(Debug, Clone)]
pub struct PenumbraClient {
    client: HttpClient,
    base_url: String,
    db_pool: Pool<Postgres>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockResponse {
    pub result: BlockResult,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockResult {
    pub block: Block,
    pub block_id: BlockId,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub data: BlockData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockHeader {
    pub height: String,
    pub time: DateTime<Utc>,
    pub last_block_id: Option<BlockId>,
    pub proposer_address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockId {
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockData {
    pub txs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub result: NodeStatus,
}

#[derive(Debug, Deserialize)]
pub struct NodeStatus {
    pub sync_info: SyncInfo,
}

#[derive(Debug, Deserialize)]
pub struct SyncInfo {
    pub latest_block_height: String,
    pub latest_block_time: DateTime<Utc>,
    pub catching_up: bool,
}

impl PenumbraClient {
    pub async fn connect(addr: &str, pool: Pool<Postgres>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        println!("Attempting to connect with RPC config...");

        let client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(30))
            .build()?;

        println!("HTTP client created successfully");

        Ok(Self {
            client,
            base_url: addr.to_string(),
            db_pool: pool,
        })
    }

    pub async fn get_status(&self) -> Result<StatusResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/status", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    pub async fn fetch_blocks(
        &self,
        start_height: u64,
        end_height: u64,
        batch_size: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
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

                        // Store block in database
                        let result_json = serde_json::to_value(&block.result).unwrap_or_default();
                        let stored_block = StoredBlock {
                            height: height as i64,
                            time: block.result.block.header.time,
                            hash: block.result.block_id.hash.clone(),
                            proposer_address: block.result.block.header.proposer_address.clone(),
                            tx_count: block.result.block.data.txs.map_or(0, |txs| txs.len()) as i32,
                            previous_block_hash: block.result.block.header.last_block_id.map(|id| id.hash),
                            data: result_json,
                            created_at: Utc::now(),
                        };

                        if let Err(e) = crate::db::store_block(&self.db_pool, stored_block).await {
                            eprintln!("Error storing block in database: {}", e);
                        }
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

    async fn fetch_block(&self, height: u64) -> Result<BlockResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/block?height={}", self.base_url, height);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }
}