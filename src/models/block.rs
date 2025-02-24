/*
* Block model definitions.
*
* Defines entities for storing blockchain block data and their serialization
* properties. Includes the core StoredBlock model which maps to the
* database schema and BlockSummary for API responses.
*/

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/*
* Represents a stored blockchain block.
*
* Maps directly to the 'blocks' table via SQLx's FromRow trait.
* Contains metadata about the block, including its hash, proposer,
* transaction count, burn amount, and references to previous blocks.
*/
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoredBlock {
    /* Block height in the blockchain */
    pub height: i64,

    /* Timestamp of when the block was produced */
    pub time: DateTime<Utc>,

    /* Unique block hash identifier */
    pub hash: String,

    /* Address of the validator who proposed the block */
    pub proposer_address: String,

    /* Number of transactions included in the block */
    pub tx_count: i32,

    /* Hash of the previous block (if available) */
    pub previous_block_hash: Option<String>,

    /* Total amount of tokens burned in this block */
    pub burn_amount: f64,

    /* Full block data in JSON format */
    pub data: serde_json::Value,

    /* Timestamp when the block record was created in the indexer */
    pub created_at: DateTime<Utc>,
}

/*
* Implementation of utility methods for StoredBlock
*/
impl StoredBlock {
    /*
    * Converts a StoredBlock to a BlockSummary
    *
    * @return BlockSummary containing essential block information
    */
    pub fn to_summary(&self) -> BlockSummary {
        BlockSummary {
            height: self.height,
            time: self.time,
            tx_count: self.tx_count
        }
    }
}

/*
* Simplified block summary for API responses.
*
* Contains only the essential fields needed for block listings
* and frontend display.
*/
#[derive(Debug, Serialize)]
pub struct BlockSummary {
    /* Block height */
    pub height: i64,

    /* Block timestamp */
    pub time: DateTime<Utc>,

    /* Number of transactions */
    pub tx_count: i32,
}

/*
* Container for block lists in API responses.
*
* Provides additional metadata like total count alongside the
* actual block summaries.
*/
#[derive(Debug, Serialize)]
pub struct BlockList {
    /* Collection of block summaries */
    pub blocks: Vec<BlockSummary>,

    /* Total count of blocks in the list */
    pub total_count: i64,
}

impl BlockList {
    /*
    * Creates a new BlockList from a collection of block summaries.
    *
    * @param blocks Vector of BlockSummary objects to include
    * @return A new BlockList instance
    */
    pub fn new(blocks: Vec<BlockSummary>) -> Self {
        let total_count = blocks.len() as i64;
        Self { blocks, total_count }
    }
}
