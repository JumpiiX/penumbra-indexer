use axum::{extract::State, Json};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use tracing::{error, instrument};

use crate::{
    db::stats::StatsQueries,
    models::stats::{BurnStats, CurrentBlockStats, StatsResponse, TransactionStats},
    error::ApiError,
};

#[instrument(skip(pool))]
pub async fn get_chain_stats(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<StatsResponse>, ApiError> {
    // Capture current time before fetching latest block
    let now = Utc::now();

    // Get current block information
    let latest_block = StatsQueries::get_latest_block_timing(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch latest block: {}", e);
            ApiError::DatabaseError(e)
        })?;

    // Get previous block for time difference calculation
    let prev_block = StatsQueries::get_previous_block_timing(&pool, latest_block.height)
        .await
        .map_err(|e| {
            error!("Failed to fetch previous block: {}", e);
            ApiError::DatabaseError(e)
        })?;

    // Calculate time differences as integers
    let block_time = (latest_block.timestamp - prev_block.timestamp).num_seconds();
    let received_new = (now - latest_block.timestamp).num_seconds().max(0);

    // Get transaction statistics
    let total_tx_count = StatsQueries::get_total_transactions(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch total transactions: {}", e);
            ApiError::DatabaseError(e)
        })?;

    let new_today_tx = StatsQueries::get_today_transactions(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch today's transactions: {}", e);
            ApiError::DatabaseError(e)
        })?;

    let tx_history = StatsQueries::get_transaction_history(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch transaction history: {}", e);
            ApiError::DatabaseError(e)
        })?;

    // Get burn statistics
    let total_burn = StatsQueries::get_total_burn(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch total burn: {}", e);
            ApiError::DatabaseError(e)
        })?;

    let burn_history = StatsQueries::get_burn_history(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch burn history: {}", e);
            ApiError::DatabaseError(e)
        })?;

    // Construct response
    let response = StatsResponse::new(
        CurrentBlockStats::new(latest_block.height, block_time.to_string(), received_new.to_string()),
        TransactionStats::new(total_tx_count, new_today_tx, tx_history),
        BurnStats::new(total_burn, burn_history),
    );

    Ok(Json(response))
}
