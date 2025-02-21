/*
* Blockchain statistics database operations.
*
* Handles all database interactions related to blockchain statistics,
* including calculating block times, validator counts, and transaction volumes.
*/

use sqlx::{Pool, Postgres};
use crate::models::ChainStats;

/* SQL for retrieving chain statistics. */
const GET_CHAIN_STATS_SQL: &str = r#"
    WITH block_times AS (
        SELECT
            EXTRACT(EPOCH FROM (time - lag(time) OVER (ORDER BY height)))::float8 as block_time
        FROM blocks
    )
    SELECT
        (SELECT MAX(height) FROM blocks) as total_blocks,
        (SELECT COUNT(DISTINCT proposer_address) FROM blocks) as active_validators,
        (SELECT SUM(tx_count) FROM blocks) as total_transactions,
        AVG(block_time)::float8 as avg_block_time
    FROM block_times
    WHERE block_time IS NOT NULL
"#;

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
*/
pub async fn get_chain_stats(
    pool: &Pool<Postgres>,
) -> Result<ChainStats, sqlx::Error> {
    sqlx::query_as::<_, ChainStats>(GET_CHAIN_STATS_SQL)
        .fetch_one(pool)
        .await
}
