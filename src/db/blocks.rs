/*
* Block-specific database operations.
*
* Handles all database interactions related to blockchain blocks,
* including storing, retrieving, and analyzing block data.
*/

use sqlx::{Pool, Postgres};
use crate::models::StoredBlock;

/* SQL queries for blocks */

/* SQL for inserting or updating a block.
 * Uses upsert (ON CONFLICT) to handle duplicates.
 */
const UPSERT_BLOCK_SQL: &str = r#"
    INSERT INTO blocks (
        height, time, hash, proposer_address,
        tx_count, previous_block_hash, data, created_at
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ON CONFLICT (height) DO UPDATE
    SET time = EXCLUDED.time,
        hash = EXCLUDED.hash,
        proposer_address = EXCLUDED.proposer_address,
        tx_count = EXCLUDED.tx_count,
        previous_block_hash = EXCLUDED.previous_block_hash,
        data = EXCLUDED.data,
        created_at = EXCLUDED.created_at
"#;

/* SQL for retrieving the latest blocks. */
const GET_LATEST_BLOCKS_SQL: &str = r#"
    SELECT * FROM blocks
    ORDER BY height DESC
    LIMIT $1
"#;

/* SQL for retrieving a specific block by height. */
const GET_BLOCK_BY_HEIGHT_SQL: &str = r#"
    SELECT *
    FROM blocks
    WHERE height = $1
"#;

/*
* Stores a block in the database.
*
* Performs an upsert operation (insert or update) for the given block.
*
* @param pool Database connection pool
* @param block Block data to store
*/
pub async fn store_block(
    pool: &Pool<Postgres>,
    block: StoredBlock,
) -> Result<(), sqlx::Error> {
    // Begin transaction for consistency
    let mut tx = pool.begin().await?;

    // Insert or update block
    sqlx::query(UPSERT_BLOCK_SQL)
        .bind(block.height)
        .bind(block.time)
        .bind(&block.hash)
        .bind(&block.proposer_address)
        .bind(block.tx_count)
        .bind(&block.previous_block_hash)
        .bind(&block.data)
        .bind(block.created_at)
        .execute(&mut *tx)
        .await?;

    // Commit transaction
    tx.commit().await?;

    Ok(())
}

/*
* Retrieves the latest blocks from the database.
*
* Returns the most recent blocks ordered by height descending.
*
* @param pool Database connection pool
* @return Vector of recent block data
*/
pub async fn get_latest_blocks(
    pool: &Pool<Postgres>,
) -> Result<Vec<StoredBlock>, sqlx::Error> {
    sqlx::query_as::<_, StoredBlock>(GET_LATEST_BLOCKS_SQL)
        .bind(10)
        .fetch_all(pool)
        .await
}

/*
* Retrieves a specific block by its height from the database.
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
