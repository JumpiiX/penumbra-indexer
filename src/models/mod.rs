use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoredBlock {
    pub height: i64,
    pub time: DateTime<Utc>,
    pub hash: String,
    pub proposer_address: String,
    pub tx_count: i32,
    pub previous_block_hash: Option<String>,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BlockList {
    pub blocks: Vec<StoredBlock>,
    pub total_count: i64,
}