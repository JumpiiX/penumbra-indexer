/*
* Blockchain statistics API module.
*
* Handles API endpoints related to retrieving blockchain statistics,
* including block counts, transaction volumes, and burn amounts.
*/

use axum::{extract::State, http::StatusCode, Json};
use sqlx::{Pool, Postgres};
use crate::{db, models::stats::{ChainStats, DailyStats}};
use super::common::{database_error, ErrorResponse};

/*
* Retrieves blockchain statistics.
*
* @param pool Database connection pool
* @return ChainStats containing blockchain metrics
*/
pub async fn get_chain_stats(
    State(pool): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<ChainStats>), (StatusCode, Json<ErrorResponse>)> {
    let stats = match db::stats::get_chain_stats(&pool).await {
        Ok(base_stats) => base_stats,
        Err(e) => return Err(database_error(e)),
    };

    let daily_stats = match db::stats::get_daily_stats(&pool).await {
        Ok(daily) => daily,
        Err(e) => return Err(database_error(e)),
    };

    // Convert daily stats into the format needed for the response
    let transaction_history: Vec<DailyStats> = daily_stats.iter()
        .map(|stat| DailyStats {
            date: stat.date,
            value: stat.tx_count as f64,
        })
        .collect();

    let burn_history: Vec<DailyStats> = daily_stats.iter()
        .map(|stat| DailyStats {
            date: stat.date,
            value: stat.total_burn,
        })
        .collect();

    let response = ChainStats {
        total_blocks: stats.total_blocks,
        total_transactions: stats.total_transactions,
        total_burn: stats.total_burn,
        avg_block_time: stats.avg_block_time,
        transaction_history,
        burn_history,
    };

    Ok((StatusCode::OK, Json(response)))
}