use axum::{extract::State, Json, http::StatusCode};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use tracing::{error, instrument};

use crate::{
    db::stats::StatsQueries,
    models::stats::{BurnStats, CurrentBlockStats, StatsResponse, TransactionStats},
};
use super::common::{database_error, ErrorResponse};

#[utoipa::path(
    get,
    path = "/api/stats",
    tag = "Statistics",
    responses(
        (status = 200, description = "Blockchain statistics retrieved successfully", body = StatsResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[instrument(skip(pool))]
pub async fn get_chain_stats(
    State(pool): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<StatsResponse>), (StatusCode, Json<ErrorResponse>)> {
    let now = Utc::now();

    let latest_block = match StatsQueries::get_latest_block_timing(&pool).await {
        Ok(block) => block,
        Err(e) => {
            error!("Failed to fetch latest block: {}", e);
            return Err(database_error(e));
        }
    };

    let prev_block = match StatsQueries::get_previous_block_timing(&pool, latest_block.height).await {
        Ok(block) => block,
        Err(e) => {
            error!("Failed to fetch previous block: {}", e);
            return Err(database_error(e));
        }
    };

    let block_time = (latest_block.timestamp - prev_block.timestamp).num_seconds();
    let received_new = (now - latest_block.timestamp).num_seconds().max(0);

    let total_tx_count = match StatsQueries::get_total_transactions(&pool).await {
        Ok(count) => count,
        Err(e) => {
            error!("Failed to fetch total transactions: {}", e);
            return Err(database_error(e));
        }
    };

    let new_today_tx = match StatsQueries::get_today_transactions(&pool).await {
        Ok(count) => count,
        Err(e) => {
            error!("Failed to fetch today's transactions: {}", e);
            return Err(database_error(e));
        }
    };

    let tx_history = match StatsQueries::get_transaction_history(&pool).await {
        Ok(history) => history,
        Err(e) => {
            error!("Failed to fetch transaction history: {}", e);
            return Err(database_error(e));
        }
    };

    let total_burn = match StatsQueries::get_total_burn(&pool).await {
        Ok(burn) => burn,
        Err(e) => {
            error!("Failed to fetch total burn: {}", e);
            return Err(database_error(e));
        }
    };

    let burn_history = match StatsQueries::get_burn_history(&pool).await {
        Ok(history) => history,
        Err(e) => {
            error!("Failed to fetch burn history: {}", e);
            return Err(database_error(e));
        }
    };

    let response = StatsResponse::new(
        CurrentBlockStats::new(latest_block.height, block_time.to_string(), received_new.to_string()),
        TransactionStats::new(total_tx_count, new_today_tx, tx_history),
        BurnStats::new(total_burn, burn_history),
    );

    Ok((StatusCode::OK, Json(response)))
}
