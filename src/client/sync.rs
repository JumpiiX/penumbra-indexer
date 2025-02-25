/*
* Block synchronization logic for Penumbra blockchain.
*
* Handles fetching and processing blocks from the blockchain,
* storing them in the database with appropriate error handling
* and retry logic.
*/

use std::error::Error;
use std::time::Duration;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use crate::client::rpc::RpcClient;
use crate::client::models::BlockResponse;
use crate::models::StoredBlock;

/* Default retry delay in seconds */
const RETRY_DELAY: u64 = 5;

/* Default batch size for block synchronization */
const DEFAULT_BATCH_SIZE: u64 = 100;

/*
* Main client for interacting with the Penumbra blockchain.
*
* This client handles:
* - RPC communication with the node
* - Block fetching and parsing
* - Database storage of block data
*/
#[derive(Debug, Clone)]
pub struct PenumbraClient {
    rpc_client: RpcClient,
    pub db_pool: Pool<Postgres>,
}

impl PenumbraClient {
    /*
    * Creates a new PenumbraClient instance.
    *
    * @param addr Base URL of the Penumbra RPC endpoint
    * @param pool PostgreSQL connection pool for database operations
    * @return Result containing either the client instance or an error
    */
    pub async fn connect(addr: &str, pool: Pool<Postgres>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        println!("Attempting to connect with RPC config...");

        let rpc_client = RpcClient::new(addr)?;

        println!("HTTP client created successfully");

        Ok(Self {
            rpc_client,
            db_pool: pool,
        })
    }

    /*
    * Retrieves the current status of the Penumbra node.
    */
    pub async fn get_status(&self) -> Result<crate::client::models::StatusResponse, Box<dyn Error + Send + Sync>> {
        self.rpc_client.get_status().await
    }

    /*
    * Synchronizes blocks from genesis to the current blockchain height.
    * Used for initial sync when the indexer first starts.
    *
    * @param batch_size Number of blocks to fetch in each batch
    */
    pub async fn sync_from_genesis(&self, batch_size: u64) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Check if initial sync should be skipped
        let skip_initial_sync = std::env::var("SKIP_INITIAL_SYNC")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        if skip_initial_sync {
            println!("Skipping initial sync as configured by environment variable");
            return Ok(());
        }

        // Get the current blockchain height
        let status = self.get_status().await?;
        let chain_height: u64 = status.result.sync_info.latest_block_height
            .parse()
            .unwrap_or(0);

        if chain_height == 0 {
            return Err("Failed to parse chain height".into());
        }

        println!("Current blockchain height: {}", chain_height);

        // Get the highest block we have in our database
        let latest_blocks = crate::db::blocks::get_latest_blocks(&self.db_pool).await?;
        let db_height = if !latest_blocks.is_empty() {
            latest_blocks[0].height as u64
        } else {
            0 // Database is empty
        };

        println!("Latest indexed height: {}", db_height);

        // If database is up to date
        if db_height >= chain_height {
            println!("Database is already up to date with blockchain");
            return Ok(());
        }

        // Start from the known first valid block if database is empty
        let start_height = if db_height == 0 {
            println!("Starting sync from first known valid block (2611800)...");
            2611800 // Known first valid block
        } else {
            println!("Continuing sync from last indexed block...");
            db_height + 1
        };

        println!("Fetching blocks from {} to {} (total: {} blocks)",
                 start_height, chain_height, chain_height - start_height + 1);

        // Sync blocks using existing fetch_blocks method
        self.fetch_blocks(start_height, chain_height, batch_size).await?;

        println!("Initial blockchain synchronization completed");
        Ok(())
    }
    /*
    * Fetches a range of blocks from the Penumbra blockchain.
    *
    * @param start_height Starting block height
    * @param end_height Ending block height
    * @param batch_size Number of blocks to fetch in each batch
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
    */
    async fn process_single_block(&self, height: u64) -> Result<(), Box<dyn Error + Send + Sync>> {
        let block = self.fetch_block(height).await?;

        println!("Block {}", height);
        println!("  Time: {}", block.result.block.header.time);
        if let Some(last_block) = &block.result.block.header.last_block_id {
            println!("  Previous block hash: {}", last_block.hash);
        }

        let tx_count = block.result.block.data.txs.as_ref().map_or(0, |txs| txs.len()) as i32;
        println!("  Transaction count: {}", tx_count);
        println!("-------------------");

        let result_json = serde_json::to_value(&block.result)?;

        let mut total_burn = 0.0;
        if let Some(txs) = &block.result.block.data.txs {
            for tx_data in txs.iter() {
                if let Some(burn) = self.extract_burn_amount(tx_data) {
                    total_burn += burn;
                }
            }
        }

        let stored_block = StoredBlock {
            height: height as i64,
            time: block.result.block.header.time,
            hash: block.result.block_id.hash.clone(),
            proposer_address: block.result.block.header.proposer_address.clone(),
            tx_count,
            previous_block_hash: block.result.block.header.last_block_id.map(|id| id.hash),
            burn_amount: total_burn,
            data: result_json,
            created_at: Utc::now(),
        };

        crate::db::blocks::store_block(&self.db_pool, stored_block.clone()).await?;

        if let Some(txs) = &block.result.block.data.txs {
            for (i, tx_data) in txs.iter().enumerate() {
                let tx_hash = format!("{}_{}", block.result.block_id.hash, i);

                // Extract transaction type and amount
                let (action_type, amount) = self.analyze_transaction(tx_data);

                crate::db::transactions::store_transaction(
                    &self.db_pool,
                    &tx_hash,
                    height as i64,
                    block.result.block.header.time,
                    &action_type,
                    amount,
                    tx_data
                ).await?;
            }
        }

        Ok(())
    }

    /*
    * Analyzes a transaction to determine its type and amount.
    *
    * @param tx_data Raw transaction data
    * @return Tuple of (action_type, optional_amount)
    */
    fn analyze_transaction(&self, tx_data: &str) -> (String, Option<f64>) {
        // Here you would implement the logic to decode the transaction data
        // and determine the type and amount based on your chain's specifics

        // For now, returning placeholder values
        if tx_data.contains("spend") {
            ("spend".to_string(), Some(3.0))
        } else {
            ("not yet supported act...".to_string(), None)
        }
    }

    /*
    * Extracts the burn amount from a transaction.
    *
    * @param tx_data Raw transaction data
    * @return Optional burn amount
    */
    fn extract_burn_amount(&self, tx_data: &str) -> Option<f64> {
        // Here you would implement the logic to decode the transaction data
        // and extract any burn amount based on your chain's specifics

        // For now, returning None as placeholder
        None
    }

    /*
    * Fetches a single block from the Penumbra blockchain.
    */
    async fn fetch_block(&self, height: u64) -> Result<BlockResponse, Box<dyn Error + Send + Sync>> {
        self.rpc_client.get_block(height).await
    }
}
