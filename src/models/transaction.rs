/*
* Transaction model definitions.
*
* Defines entities for storing transaction data and their serialization
* properties. Includes the core Transaction model which maps to the
* database schema and TransactionList for API responses.
*/

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/*
* Represents a blockchain transaction stored in the database.
*
* Maps directly to the 'transactions' table via SQLx's FromRow trait.
* Contains both metadata about the transaction and its blockchain data.
*/
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    /* Database identifier */
    pub id: i32,

    /* Unique transaction identifier hash */
    pub tx_hash: String,

    /* Block height where this transaction was included */
    pub block_height: i64,

    /* Transaction timestamp */
    pub time: DateTime<Utc>,

    /* Full transaction data (usually base64-encoded) */
    pub data: String,

    /* When this transaction record was created in the indexer */
    pub created_at: DateTime<Utc>,
}

/*
* Container for transaction lists in API responses.
*
* Provides additional metadata like total count alongside the
* actual transaction records.
*/
#[derive(Debug, Serialize)]
pub struct TransactionList {
    /* Collection of transaction records */
    pub transactions: Vec<Transaction>,

    /* Total count of transactions in the list */
    pub total_count: i64,
}

impl TransactionList {
    /*
     * Creates a new TransactionList from a collection of transactions.
     *
     * Automatically calculates the total count based on the collection size.
     *
     * @param transactions Vector of Transaction objects to include
     * @return A new TransactionList instance
     */
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let total_count = transactions.len() as i64;
        Self { transactions, total_count }
    }
}