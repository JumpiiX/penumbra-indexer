use serde::Serialize;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct StatsResponse {
    /// Current block information
    pub current_block: CurrentBlockStats,

    /// Transaction statistics
    pub total_transactions: TransactionStats,

    /// Token burn statistics
    pub total_burn: BurnStats,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentBlockStats {
    /// Current blockchain height
    pub height: i64,

    /// Time between blocks
    pub block_time: String,

    /// Time since the latest block was received
    pub received_new: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionStats {
    /// Total number of transactions
    pub count: i64,

    /// Number of transactions added today
    pub new_today: i64,

    /// Historical transaction data for charting
    pub history: Vec<ChartPoint>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BurnStats {
    /// Total amount of tokens burned
    pub amount: String,

    /// Historical burn data for charting
    pub history: Vec<ChartPoint>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChartPoint {
    /// Date label for the data point
    pub date: String,

    /// Value for the data point
    pub value: i64,
}

#[derive(Debug)]
pub struct BlockTimingInfo {
    pub height: i64,
    pub timestamp: DateTime<Utc>,
}

impl StatsResponse {
    pub fn new(
        current_block: CurrentBlockStats,
        total_transactions: TransactionStats,
        total_burn: BurnStats,
    ) -> Self {
        Self {
            current_block,
            total_transactions,
            total_burn,
        }
    }
}

impl CurrentBlockStats {
    pub fn new(height: i64, block_time: String, received_new: String) -> Self {
        Self {
            height,
            block_time,
            received_new,
        }
    }
}

impl TransactionStats {
    pub fn new(count: i64, new_today: i64, history: Vec<ChartPoint>) -> Self {
        Self {
            count,
            new_today,
            history,
        }
    }
}

impl BurnStats {
    pub fn new(amount: f64, history: Vec<ChartPoint>) -> Self {
        Self {
            amount: format!("{} UM", amount.round() as i64),
            history,
        }
    }
}