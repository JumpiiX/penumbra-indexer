/*
* Database operations for blocks.
*
* Handles all database interactions related to blockchain blocks,
* including storing, retrieving, and analyzing block data.
*/

use sqlx::{Pool, Postgres};
use crate::models::StoredBlock;

/* SQL queries for blocks */

/* SQL for inserting or updating a block */
const UPSERT_BLOCK_SQL: &str = r#"
    INSERT INTO blocks (
        height, time, hash, proposer_address,
        tx_count, previous_block_hash, burn_amount, data, created_at
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    ON CONFLICT (height) DO UPDATE
    SET time = EXCLUDED.time,
        hash = EXCLUDED.hash,
        proposer_address = EXCLUDED.proposer_address,
        tx_count = EXCLUDED.tx_count,
        previous_block_hash = EXCLUDED.previous_block_hash,
        burn_amount = EXCLUDED.burn_amount,
        data = EXCLUDED.data,
        created_at = EXCLUDED.created_at
"#;

/* SQL for retrieving the latest blocks */
const GET_LATEST_BLOCKS_SQL: &str = r#"
    SELECT * FROM blocks
    ORDER BY height DESC
    LIMIT $1
"#;

/* SQL for retrieving a specific block by height */
const GET_BLOCK_BY_HEIGHT_SQL: &str = r#"
    SELECT *
    FROM blocks
    WHERE height = $1
"#;

/*
* Stores a block in the database.
*
* @param pool Database connection pool
* @param block Block data to store
*/
pub async fn store_block(
    pool: &Pool<Postgres>,
    block: StoredBlock,
) -> Result<(), sqlx::Error> {
    sqlx::query(UPSERT_BLOCK_SQL)
        .bind(block.height)
        .bind(block.time)
        .bind(&block.hash)
        .bind(&block.proposer_address)
        .bind(block.tx_count)
        .bind(&block.previous_block_hash)
        .bind(block.burn_amount)
        .bind(&block.data)
        .bind(block.created_at)
        .execute(pool)
        .await?;

    Ok(())
}

/*
* Retrieves the latest blocks from the database.
*
* @param pool Database connection pool
* @return Vector of recent block data
*/
pub async fn get_latest_blocks(
    pool: &Pool<Postgres>,
) -> Result<Vec<StoredBlock>, sqlx::Error> {
    sqlx::query_as::<_, StoredBlock>(GET_LATEST_BLOCKS_SQL)
        .bind(10) // Fetch last 10 blocks
        .fetch_all(pool)
        .await
}

/*
* Retrieves a specific block by its height.
*
* @param pool Database connection pool
* @param height The blockchain height to query for
* @return The block if found, None if not exists
*/
pub async fn get_block_by_height(
    pool: &Pool<Postgres>,
    height: i64,
) -> Result<Option<StoredBlock>, sqlx::Error> {
    sqlx::query_as::<_, StoredBlock>(GET_BLOCK_BY_HEIGHT_SQL)
        .bind(height)
        .fetch_optional(pool)
        .await
}
