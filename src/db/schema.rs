/*
* Database schema management module.
*
* Contains all table definitions and handles schema migrations
* or updates. Keeps database structure separate from operations.
*/

use sqlx::{Pool, Postgres};

/* SQL definitions for the blocks table */
pub const BLOCKS_TABLE_SQL: &str = r#"
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

/* SQL definitions for the transactions table */
pub const TRANSACTIONS_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS transactions (
        id SERIAL PRIMARY KEY,
        tx_hash TEXT UNIQUE NOT NULL,
        block_height BIGINT NOT NULL REFERENCES blocks(height),
        time TIMESTAMP WITH TIME ZONE NOT NULL,
        data TEXT NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
    )
"#;

/*
* Initializes or upgrades the database schema.
*
* Creates all necessary tables if they don't exist and
* performs any required migrations to keep the schema up to date.
*
* @param pool PostgreSQL connection pool
*/
pub async fn initialize_schema(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Create tables in the proper order (referenced tables first)
    sqlx::query(BLOCKS_TABLE_SQL)
        .execute(pool)
        .await?;

    sqlx::query(TRANSACTIONS_TABLE_SQL)
        .execute(pool)
        .await?;

    // Create any necessary indices for better query performance
    create_indices(pool).await?;

    Ok(())
}

/*
* Creates optimized database indices for better query performance.
*
* This function is automatically called during schema initialization
* but can also be run separately if needed.
*/
async fn create_indices(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Index for faster transaction lookups by block height
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_transactions_block_height ON transactions(block_height)"
    )
        .execute(pool)
        .await?;

    // Index for faster timestamp-based block queries
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_blocks_time ON blocks(time)"
    )
        .execute(pool)
        .await?;

    Ok(())
}
