use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use chrono::{DateTime, Utc};
use crate::grpc_client::proto;

/// Database handler for storing and retrieving blocks
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Creates a new database connection and initializes the schema
    pub async fn connect(database_url: &str) -> Result<Self> {
        // Create connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        // Create blocks table if it doesn't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS blocks (
                height BIGINT PRIMARY KEY,
                hash TEXT NOT NULL,
                timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
                num_transactions INTEGER NOT NULL,
                data JSONB NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );"#,
        )
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    /// Stores a block in the database
    pub async fn store_block(&self, block: &proto::Block) -> Result<()> {
        let timestamp = block.timestamp.as_ref()
            .map(|ts| DateTime::from_timestamp(ts.seconds, ts.nanos as u32))
            .flatten()
            .unwrap_or_else(|| Utc::now());

        sqlx::query(
            r#"
            INSERT INTO blocks (height, hash, timestamp, num_transactions, data)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (height) DO UPDATE
            SET hash = $2, timestamp = $3, num_transactions = $4, data = $5
            "#,
        )
            .bind(block.height as i64)
            .bind(hex::encode(&block.hash))
            .bind(timestamp)
            .bind(block.transactions.len() as i32)
            .bind(serde_json::to_value(block)?)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Gets the latest blocks from the database
    pub async fn get_latest_blocks(&self, limit: i64) -> Result<Vec<serde_json::Value>> {
        let blocks = sqlx::query!(
            r#"
            SELECT data as "data!: serde_json::Value"
            FROM blocks
            ORDER BY height DESC
            LIMIT $1
            "#,
            limit
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(blocks.into_iter().map(|b| b.data).collect())
    }
}