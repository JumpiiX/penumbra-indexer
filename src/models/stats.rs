use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub current_block: CurrentBlockStats,
    pub total_transactions: TransactionStats,
    pub total_burn: BurnStats,
}

#[derive(Debug, Serialize)]
pub struct CurrentBlockStats {
    pub height: i64,
    pub block_time: String,
    pub received_new: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionStats {
    pub count: i64,
    pub new_today: i64,
    pub history: Vec<ChartPoint>,
}

#[derive(Debug, Serialize)]
pub struct BurnStats {
    pub amount: String,
    pub history: Vec<ChartPoint>,
}

#[derive(Debug, Serialize)]
pub struct ChartPoint {
    pub date: String,
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