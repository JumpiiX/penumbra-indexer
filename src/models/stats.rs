/*
* Stats model definitions.
*
* Defines entities for storing blockchain statistics and their
* serialization properties.
*/

use serde::Serialize;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/* Database model for chain statistics */
#[derive(Debug, Serialize, FromRow)]
pub struct DbChainStats {
    pub total_blocks: i64,
    pub total_transactions: i64,
    pub total_burn: f64,
    pub avg_block_time: Option<f64>
}

/* Database model for daily statistics */
#[derive(Debug, Serialize, FromRow)]
pub struct DbDailyStats {
    pub date: DateTime<Utc>,
    pub tx_count: i64,
    pub total_burn: f64
}

/* API response model for chain statistics */
#[derive(Debug, Serialize)]
pub struct ChainStats {
    pub total_blocks: i64,
    pub total_transactions: i64,
    pub total_burn: f64,
    pub avg_block_time: Option<f64>,
    pub transaction_history: Vec<DailyStats>,
    pub burn_history: Vec<DailyStats>
}

/* API response model for daily statistics */
#[derive(Debug, Serialize)]
pub struct DailyStats {
    pub date: DateTime<Utc>,
    pub value: f64
}