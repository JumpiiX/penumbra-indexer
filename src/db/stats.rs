/*
* Database operations for chain statistics.
*
* Handles all database interactions related to blockchain statistics,
* including calculating metrics and retrieving historical data.
*/

use sqlx::{Pool, Postgres};
use crate::models::stats::{DbChainStats, DbDailyStats};

/* SQL queries for statistics */

/* SQL for retrieving chain statistics */
const GET_CHAIN_STATS_SQL: &str = r#"
    SELECT
        (SELECT MAX(height) FROM blocks) as total_blocks,
        (SELECT SUM(tx_count) FROM blocks) as total_transactions,
        (SELECT SUM(burn_amount) FROM blocks) as total_burn,
        (
            SELECT AVG(EXTRACT(EPOCH FROM (time - lag(time) OVER (ORDER BY height))))::float8
            FROM blocks
            WHERE height > 1
        ) as avg_block_time
"#;

/* SQL for retrieving daily statistics */
const GET_DAILY_STATS_SQL: &str = r#"
    SELECT
        date_trunc('day', time) as date,
        COUNT(*) as tx_count,
        SUM(burn_amount) as total_burn
    FROM blocks
    GROUP BY date_trunc('day', time)
    ORDER BY date DESC
    LIMIT 30
"#;

/*
* Retrieves chain statistics.
*
* @param pool Database connection pool
* @return Result containing chain statistics
*/
pub async fn get_chain_stats(
    pool: &Pool<Postgres>,
) -> Result<DbChainStats, sqlx::Error> {
    sqlx::query_as::<_, DbChainStats>(GET_CHAIN_STATS_SQL)
        .fetch_one(pool)
        .await
}

/*
* Retrieves daily statistics.
*
* @param pool Database connection pool
* @return Result containing vector of daily statistics
*/
pub async fn get_daily_stats(
    pool: &Pool<Postgres>,
) -> Result<Vec<DbDailyStats>, sqlx::Error> {
    sqlx::query_as::<_, DbDailyStats>(GET_DAILY_STATS_SQL)
        .fetch_all(pool)
        .await
}