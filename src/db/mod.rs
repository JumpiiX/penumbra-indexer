use sqlx::{Pool, Postgres};
use crate::models::StoredBlock;

pub async fn init_db(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::query(
        r#"
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
        "#,
    )
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn store_block(
    pool: &Pool<Postgres>,
    block: StoredBlock,
) -> Result<(), sqlx::Error> {
    // Insert new block
    sqlx::query(
        r#"
        INSERT INTO blocks (height, time, hash, proposer_address, tx_count, previous_block_hash, data, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (height) DO UPDATE
        SET time = EXCLUDED.time,
            hash = EXCLUDED.hash,
            proposer_address = EXCLUDED.proposer_address,
            tx_count = EXCLUDED.tx_count,
            previous_block_hash = EXCLUDED.previous_block_hash,
            data = EXCLUDED.data,
            created_at = EXCLUDED.created_at
        "#,
    )
        .bind(block.height)
        .bind(block.time)
        .bind(&block.hash)
        .bind(&block.proposer_address)
        .bind(block.tx_count)
        .bind(&block.previous_block_hash)
        .bind(&block.data)
        .bind(block.created_at)
        .execute(pool)
        .await?;

    // Keep only latest 10 blocks
    sqlx::query(
        r#"
        DELETE FROM blocks
        WHERE height NOT IN (
            SELECT height FROM blocks
            ORDER BY height DESC
            LIMIT 10
        )
        "#,
    )
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_latest_blocks(
    pool: &Pool<Postgres>,
) -> Result<Vec<StoredBlock>, sqlx::Error> {
    sqlx::query_as::<_, StoredBlock>(
        r#"
        SELECT * FROM blocks
        ORDER BY height DESC
        LIMIT 10
        "#,
    )
        .fetch_all(pool)
        .await
}