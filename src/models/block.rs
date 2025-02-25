use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct StoredBlock {
    /// Block height in the blockchain
    pub height: i64,

    /// Timestamp of when the block was produced
    #[schema(value_type = String, format = "date-time", example = "2025-02-25T12:34:56Z")]
    pub time: DateTime<Utc>,

    /// Unique block hash identifier
    pub hash: String,

    /// Address of the validator who proposed the block
    pub proposer_address: String,

    /// Number of transactions included in the block
    pub tx_count: i32,

    /// Hash of the previous block (if available)
    pub previous_block_hash: Option<String>,

    /// Total amount of tokens burned in this block
    pub burn_amount: f64,

    /// Full block data in JSON format
    pub data: serde_json::Value,

    /// Timestamp when the block record was created in the indexer
    #[schema(value_type = String, format = "date-time", example = "2025-02-25T12:34:56Z")]
    pub created_at: DateTime<Utc>,
}

impl StoredBlock {
    pub fn to_summary(&self) -> BlockSummary {
        BlockSummary {
            height: self.height,
            time: self.time,
            tx_count: self.tx_count
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BlockSummary {
    /* Block height */
    pub height: i64,

    /* Block timestamp */
    #[schema(value_type = String, format = "date-time", example = "2025-02-25T12:34:56Z")]
    pub time: DateTime<Utc>,

    /* Number of transactions */
    pub tx_count: i32,
}

#[derive(Debug, Serialize, ToSchema)]
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