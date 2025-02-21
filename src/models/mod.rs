/*
 * Database models and response types for the Penumbra indexer.
 *
 * This module contains the core data structures used for:
 * - Database storage (StoredBlock)
 * - API responses (BlockList)
 * - Data serialization/deserialization
 *
 * @version 1.0
 */
mod blocks;
mod transactions;
mod stats;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/*
 * Represents a block as stored in the database.
 *
 * This structure combines essential block information with metadata
 * needed for indexing and querying. It maps directly to the 'blocks'
 * table in PostgreSQL via sqlx::FromRow.
 *
 * @see BlockList
 */
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoredBlock {
    /* Block height in the blockchain */
    pub height: i64,

    /* Timestamp when the block was created */
    pub time: DateTime<Utc>,

    /* Unique hash identifying this block */
    pub hash: String,

    /* Address of the validator who proposed this block */
    pub proposer_address: String,

    /* Number of transactions in this block */
    pub tx_count: i32,

    /* Hash of the previous block, if any */
    pub previous_block_hash: Option<String>,

    /* Complete block data stored as JSON */
    pub data: serde_json::Value,

    /* Timestamp when this block was indexed */
    pub created_at: DateTime<Utc>,
}

/*
 * Response structure for block list queries.
 *
 * Used by the API to return a paginated list of blocks
 * along with the total count.
 *
 * @property blocks List of blocks to return
 * @property total_count Total number of blocks in the result
 */
#[derive(Debug, Serialize)]
pub struct BlockList {
    pub blocks: Vec<StoredBlock>,
    pub total_count: i64,
}

impl BlockList {
    /*
     * Creates a new BlockList instance.
     *
     * @param blocks Vector of blocks to include
     * @return BlockList instance with calculated total count
     */
    pub fn new(blocks: Vec<StoredBlock>) -> Self {
        let total_count = blocks.len() as i64;
        Self {
            blocks,
            total_count,
        }
    }
}

/*
 * Statistics about the blockchain state.
 *
 * Provides aggregated metrics about the blockchain including:
 * - Total number of blocks stored
 * - Count of unique validators (proposers)
 * - Total transactions processed
 * - Average time between blocks
 *
 * @see get_chain_stats
 */
#[derive(Debug, Serialize, FromRow)]
pub struct ChainStats {
    /* Total number of blocks in the chain */
    pub total_blocks: i64,

    /* Number of unique validators seen */
    pub active_validators: i64,

    /* Total number of transactions processed */
    pub total_transactions: i64,

    /* Average time between blocks in seconds */
    pub avg_block_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i32,
    pub tx_hash: String,
    pub block_height: i64,
    pub time: DateTime<Utc>,
    pub data: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TransactionList {
    pub transactions: Vec<Transaction>,
    pub total_count: i64,
}

impl TransactionList {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let total_count = transactions.len() as i64;
        Self {
            transactions,
            total_count,
        }
    }
}
