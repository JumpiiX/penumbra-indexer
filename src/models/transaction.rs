use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Transaction {
    /// Internal transaction ID
    pub id: i32,

    /// Unique transaction hash
    pub tx_hash: String,

    /// Block height where this transaction was included
    pub block_height: i64,

    /// Timestamp when the transaction was processed
    #[schema(value_type = String, format = "date-time", example = "2025-02-25T12:34:56Z")]
    pub time: DateTime<Utc>,

    /// Type of action performed in this transaction
    pub action_type: String,

    /// Amount involved in the transaction (if applicable)
    pub amount: Option<f64>,

    /// Raw transaction data
    pub data: String,

    /// Timestamp when the transaction was indexed
    #[schema(value_type = String, format = "date-time", example = "2025-02-25T12:34:56Z")]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionSummary {
    /// Unique transaction hash
    pub tx_hash: String,

    /// Block height where this transaction was included
    pub block_height: i64,

    /// Type of action performed in this transaction
    pub action_type: String,

    /// Amount involved in the transaction (if applicable)
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionList {
    /// List of transaction summaries
    pub transactions: Vec<TransactionSummary>,

    /// Total count of transactions in the response
    pub total_count: i64,
}

impl TransactionList {
    pub fn new(transactions: Vec<TransactionSummary>) -> Self {
        let total_count = transactions.len() as i64;
        Self { transactions, total_count }
    }
}