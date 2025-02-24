/*
* Transaction model definitions.
*/

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i32,
    pub tx_hash: String,
    pub block_height: i64,
    pub time: DateTime<Utc>,
    pub action_type: String,
    pub amount: Option<f64>,
    pub data: String,
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn to_summary(&self) -> TransactionSummary {
        TransactionSummary {
            tx_hash: self.tx_hash.clone(),
            block_height: self.block_height,
            action_type: self.action_type.clone(),
            amount: self.amount
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TransactionSummary {
    pub tx_hash: String,
    pub block_height: i64,
    pub action_type: String,
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct TransactionList {
    pub transactions: Vec<TransactionSummary>,
    pub total_count: i64,
}

impl TransactionList {
    pub fn new(transactions: Vec<TransactionSummary>) -> Self {
        let total_count = transactions.len() as i64;
        Self { transactions, total_count }
    }
}