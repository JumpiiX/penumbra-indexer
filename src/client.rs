/*
 * Penumbra blockchain client implementation.
 *
 * This module provides the main client interface for interacting with the Penumbra blockchain.
 * It handles RPC communication, block fetching, and database storage.
 *
 * @jumpiix
 * @version 1.0
 */

use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use crate::models::StoredBlock;

/* Default timeout for HTTP requests in seconds */
const DEFAULT_TIMEOUT: u64 = 30;
/* Default retry delay in seconds */
const RETRY_DELAY: u64 = 5;

/*
 * Main client for interacting with the Penumbra blockchain.
 *
 * This client handles:
 * - RPC communication with the node
 * - Block fetching and parsing
 * - Database storage of block data
 *
 * @see BlockResponse
 * @see StatusResponse
 */
#[derive(Debug, Clone)]
pub struct PenumbraClient {
    client: HttpClient,
    base_url: String,
    db_pool: Pool<Postgres>,
}

/*
 * Response wrapper for block-related RPC calls.
 *
 * @property result The actual block result data
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockResponse {
    pub result: BlockResult,
}

/*
 * Container for block data and metadata.
 *
 * @property block The actual block content
 * @property block_id The unique identifier for this block
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockResult {
    pub block: Block,
    pub block_id: BlockId,
}

/*
 * Represents a block in the Penumbra blockchain.
 *
 * @property header Block header containing metadata
 * @property data Actual block data including transactions
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub data: BlockData,
}

/*
 * Header information for a block.
 *
 * @property height Block height as a string (converted to i64 for storage)
 * @property time Block timestamp
 * @property last_block_id Reference to the previous block
 * @property proposer_address Address of the validator who proposed this block
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockHeader {
    pub height: String,
    pub time: DateTime<Utc>,
    pub last_block_id: Option<BlockId>,
    pub proposer_address: String,
}

/*
 * Unique identifier for a block.
 *
 * @property hash The unique hash of the block
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockId {
    pub hash: String,
}

/*
 * Contains the actual block data including transactions.
 *
 * @property txs Optional list of transactions
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockData {
    pub txs: Option<Vec<String>>,
}

/*
 * Response structure for node status queries.
 *
 * @property result Contains node status information
 */
#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub result: NodeStatus,
}

/*
 * Contains node-specific status information.
 *
 * @property sync_info Information about node synchronization
 */
#[derive(Debug, Deserialize)]
pub struct NodeStatus {
    pub sync_info: SyncInfo,
}

/*
 * Information about the node's synchronization status.
 *
 * @property latest_block_height Current block height of the node
 * @property latest_block_time Timestamp of the latest block
 * @property catching_up Whether the node is still catching up
 */
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SyncInfo {
    pub latest_block_height: String,
    pub latest_block_time: DateTime<Utc>,
    pub catching_up: bool,
}

impl PenumbraClient {
    /*
     * Creates a new PenumbraClient instance.
     *
     * @param addr Base URL of the Penumbra RPC endpoint
     * @param pool PostgreSQL connection pool for database operations
     * @return Result containing either the client instance or an error
     * @throws Error if connection fails
     *
     * Example:
     * ```no_run
     * use sqlx::PgPool;
     *
     * async fn example() {
     *     let pool = PgPool::connect("postgres://...").await.unwrap();
     *     let client = PenumbraClient::connect("http://node:26657", pool).await.unwrap();
     * }
     * ```
     */
    pub async fn connect(addr: &str, pool: Pool<Postgres>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        println!("Attempting to connect with RPC config...");

        let client = HttpClient::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .connect_timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()?;

        println!("HTTP client created successfully");

        Ok(Self {
            client,
            base_url: addr.to_string(),
            db_pool: pool,
        })
    }

    /*
     * Retrieves the current status of the Penumbra node.
     *
     * @return Result containing either the node status or an error
     * @throws Error if the request fails
     */
    pub async fn get_status(&self) -> Result<StatusResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/status", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    /*
     * Fetches a range of blocks from the Penumbra blockchain.
     *
     * @param start_height Starting block height
     * @param end_height Ending block height
     * @param batch_size Number of blocks to fetch in each batch
     * @return Result indicating success or failure
     * @throws Error if block fetching fails
     *
     * Note: This method automatically stores fetched blocks in the database.
     */
    pub async fn fetch_blocks(
        &self,
        start_height: u64,
        end_height: u64,
        batch_size: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut current_height = start_height;

        while current_height <= end_height {
            let batch_end = std::cmp::min(current_height + batch_size, end_height);

            for height in current_height..=batch_end {
                if let Err(e) = self.process_single_block(height).await {
                    eprintln!("Error processing block {}: {}", height, e);
                    tokio::time::sleep(Duration::from_secs(RETRY_DELAY)).await;
                    continue;
                }
            }

            current_height = batch_end + 1;
        }

        Ok(())
    }

    /*
     * Fetches and processes a single block.
     *
     * @param height The height of the block to fetch
     * @return Result indicating success or failure
     * @throws Error if block processing fails
     */
    async fn process_single_block(&self, height: u64) -> Result<(), Box<dyn Error + Send + Sync>> {
        let block = self.fetch_block(height).await?;

        println!("Block {}", height);
        println!("  Time: {}", block.result.block.header.time);
        if let Some(last_block) = &block.result.block.header.last_block_id {
            println!("  Previous block hash: {}", last_block.hash);
        }

        // Use clone or as_ref to avoid the move
        let tx_count = block.result.block.data.txs.as_ref().map_or(0, |txs| txs.len()) as i32;
        println!("  Transaction count: {}", tx_count);
        println!("-------------------");

        let result_json = serde_json::to_value(&block.result)?;
        let stored_block = StoredBlock {
            height: height as i64,
            time: block.result.block.header.time,
            hash: block.result.block_id.hash.clone(),
            proposer_address: block.result.block.header.proposer_address.clone(),
            tx_count: tx_count,  // Use the variable we calculated above
            previous_block_hash: block.result.block.header.last_block_id.map(|id| id.hash),
            data: result_json,
            created_at: Utc::now(),
        };

        crate::db::store_block(&self.db_pool, stored_block.clone()).await?;

        // Now we can use txs again
        if let Some(txs) = &block.result.block.data.txs {
            for (i, tx_data) in txs.iter().enumerate() {
                // Generate a transaction hash
                let tx_hash = format!("{}_{}", block.result.block_id.hash, i);

                crate::db::store_transaction(
                    &self.db_pool,
                    &tx_hash,
                    height as i64,
                    block.result.block.header.time,
                    tx_data
                ).await?;
            }
        }

        Ok(())
    }
    /*
     * Fetches a single block from the Penumbra blockchain.
     *
     * @param height The height of the block to fetch
     * @return Result containing either the block data or an error
     * @throws Error if the request fails
     */
    async fn fetch_block(&self, height: u64) -> Result<BlockResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/block?height={}", self.base_url, height);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }
}
