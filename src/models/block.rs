/*
* Block model definitions.
*
* Defines entities for storing blockchain block data and their serialization
* properties. Includes the core StoredBlock model which maps to the
* database schema and BlockList for API responses.
*/

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/*
* Represents a stored blockchain block.
*
* Maps directly to the 'blocks' table via SQLx's FromRow trait.
* Contains metadata about the block, including its hash, proposer,
* transaction count, and references to previous blocks.
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

    /* Full block data in JSON format */
    pub data: serde_json::Value,

    /* Timestamp when the block record was created in the indexer */
    pub created_at: DateTime<Utc>,
}

/*
* Container for a list of blockchain blocks.
*
* Provides additional metadata such as total count alongside the
* actual block records.
*/
#[derive(Debug, Serialize)]
pub struct BlockList {
    /* Collection of block records */
    pub blocks: Vec<StoredBlock>,

    /* Total count of blocks in the list */
    pub total_count: i64,
}

impl BlockList {
    /*
    * Creates a new BlockList from a collection of blocks.
    *
    * Automatically calculates the total count based on the collection size.
    *
    * @param blocks Vector of StoredBlock objects to include
    * @return A new BlockList instance
    */
    pub fn new(blocks: Vec<StoredBlock>) -> Self {
        let total_count = blocks.len() as i64;
        Self { blocks, total_count }
    }
}
