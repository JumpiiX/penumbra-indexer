/*
 * Database operations module for the Penumbra indexer.
 *
 * This module handles:
 * - Database initialization
 * - Block storage and retrieval
 * - Historical blockchain data persistence
 *
 * @version 1.0
 */

use sqlx::{Pool, Postgres};
use crate::models::{ChainStats, StoredBlock};

/* Maximum number of database connections */
const MAX_DB_CONNECTIONS: u32 = 5;

/*
 * SQL for creating the blocks table.
 * Defines the schema for storing blockchain data.
 */
const CREATE_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS blocks (
        height BIGINT PRIMARY KEY,
        time TIMESTAMP WITH TIME ZONE NOT NULL,
        hash TEXT NOT NULL,
        proposer_address TEXT NOT NULL,
        tx_count INTEGER NOT NULL,
        previous_block_hash TEXT,
        data JSONB NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
    )
"#;

/*
 * SQL for inserting or updating a block.
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

/*
 * SQL for retrieving the latest blocks.
 * Orders by height descending and limits the result.
 */
const GET_LATEST_BLOCKS_SQL: &str = r#"
    SELECT * FROM blocks
    ORDER BY height DESC
    LIMIT $1
"#;

/*
 * SQL for retrieving a specific block by height.
 * Includes full block data and ensures index usage.
 */
const GET_BLOCK_BY_HEIGHT_SQL: &str = r#"
    SELECT
        height,
        time,
        hash,
        proposer_address,
        tx_count,
        previous_block_hash,
        data,
        created_at
    FROM blocks
    WHERE height = $1
"#;

/*
 * SQL for retrieving chain statistics.
 * Calculates various metrics about the blockchain state.
 */
const GET_CHAIN_STATS_SQL: &str = r#"
    WITH block_times AS (
        SELECT
            EXTRACT(EPOCH FROM (time - lag(time) OVER (ORDER BY height)))::float8 as block_time
        FROM blocks
    )
    SELECT
        (SELECT MAX(height) FROM blocks) as total_blocks,
        COUNT(DISTINCT proposer_address) as active_validators,
        SUM(tx_count) as total_transactions,
        AVG(block_time)::float8 as avg_block_time
    FROM blocks, block_times
    WHERE block_time IS NOT NULL
"#;

/*
 * Initializes the database connection and schema.
 *
 * Sets up the PostgreSQL connection pool and creates the necessary tables
 * if they don't exist.
 *
 * @param database_url Connection string for the PostgreSQL database
 * @return Pool<Postgres> Database connection pool
 * @throws sqlx::Error If connection or table creation fails
 */
pub async fn init_db(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(database_url)
        .await?;

    // Create tables if they don't exist
    sqlx::query(CREATE_TABLE_SQL)
        .execute(&pool)
        .await?;

    Ok(pool)
}

/*
 * Stores a block in the database.
 *
 * Performs an upsert operation (insert or update) for the given block
 * and maintains the maximum number of blocks by cleaning up old ones.
 *
 * @param pool Database connection pool
 * @param block Block data to store
 * @throws sqlx::Error If database operations fail
 */
pub async fn store_block(
    pool: &Pool<Postgres>,
    block: StoredBlock,
) -> Result<(), sqlx::Error> {
    // Begin transaction
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
 * @return Vec<StoredBlock> List of latest blocks
 * @throws sqlx::Error If query fails
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
 * This function performs a direct query for a single block matching
 * the provided height. Returns None if no block is found.
 *
 * @param pool Database connection pool
 * @param height The blockchain height to query for
 * @return Result<Option<StoredBlock>> The block if found, None if not exists
 * @throws sqlx::Error If the database query fails
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

/*
 * Retrieves statistical information about the blockchain.
 *
 * Calculates metrics including:
 * - Total number of blocks
 * - Number of active validators
 * - Total transactions processed
 * - Average block time
 *
 * @param pool Database connection pool
 * @return Result<ChainStats> Chain statistics
 * @throws sqlx::Error If the query fails
 */
pub async fn get_chain_stats(
    pool: &Pool<Postgres>,
) -> Result<ChainStats, sqlx::Error> {
    sqlx::query_as::<_, ChainStats>(GET_CHAIN_STATS_SQL)
        .fetch_one(pool)
        .await
}
