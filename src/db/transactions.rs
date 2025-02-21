/*
* Transaction-specific database operations.
*
* Handles all database interactions related to blockchain transactions,
* including storing and retrieving transaction data.
*/

use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use crate::models::Transaction;

/* SQL queries for transactions */

/* SQL for inserting a new transaction.
 * Uses ON CONFLICT to handle duplicate inserts.
 */
const INSERT_TRANSACTION_SQL: &str = r#"
    INSERT INTO transactions (
        tx_hash, block_height, time, data, created_at
    )
    VALUES ($1, $2, $3, $4, $5)
    ON CONFLICT (tx_hash) DO NOTHING
"#;

/* SQL for retrieving transactions by block height. */
const GET_TRANSACTIONS_BY_BLOCK_HEIGHT_SQL: &str = r#"
    SELECT * FROM transactions
    WHERE block_height = $1
    ORDER BY id ASC
"#;

/* SQL for retrieving the latest transactions. */
const GET_LATEST_TRANSACTIONS_SQL: &str = r#"
    SELECT * FROM transactions
    ORDER BY block_height DESC, id ASC
    LIMIT $1
"#;

/*
* Stores a transaction in the database.
*
* @param pool Database connection pool
* @param tx_hash Transaction hash identifier
* @param block_height Block height containing this transaction
* @param time Transaction timestamp
* @param data Transaction data (usually base64-encoded)
*/
pub async fn store_transaction(
    pool: &Pool<Postgres>,
    tx_hash: &str,
    block_height: i64,
    time: DateTime<Utc>,
    data: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(INSERT_TRANSACTION_SQL)
        .bind(tx_hash)
        .bind(block_height)
        .bind(time)
        .bind(data)
        .bind(Utc::now())
        .execute(pool)
        .await?;

    Ok(())
}

/*
* Retrieves the latest transactions.
*
* Gets the most recent transactions across all blocks.
*
* @param pool Database connection pool
* @param limit Maximum number of transactions to retrieve
* @return Vector of transaction data
*/
pub async fn get_latest_transactions(
    pool: &Pool<Postgres>,
    limit: i64,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(GET_LATEST_TRANSACTIONS_SQL)
        .bind(limit)
        .fetch_all(pool)
        .await
}

/*
* Retrieves transactions for a specific block height.
*
* @param pool Database connection pool
* @param height Block height to query
* @return Vector of transactions for the given block
*/
pub async fn get_transactions_by_block_height(
    pool: &Pool<Postgres>,
    height: i64,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(GET_TRANSACTIONS_BY_BLOCK_HEIGHT_SQL)
        .bind(height)
        .fetch_all(pool)
        .await
}