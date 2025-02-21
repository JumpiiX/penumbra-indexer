/*
* Blockchain statistics API module.
*
* Handles API endpoints related to retrieving blockchain statistics,
* including block counts, transaction volumes, and validator activity.
*/

use axum::{extract::State, http::StatusCode, Json};
use sqlx::{Pool, Postgres};
use crate::{db, models::stats::ChainStats};
use super::common::{database_error, ErrorResponse};

/*
* Retrieves blockchain statistics.
*
* Fetches aggregated data on blockchain activity, including the
* number of blocks, active validators, and total transactions.
*
* @param pool Database connection pool
* @return ChainStats containing blockchain metrics
*/
pub async fn get_chain_stats(
    State(pool): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<ChainStats>), (StatusCode, Json<ErrorResponse>)> {
    match db::stats::get_chain_stats(&pool).await {
        Ok(stats) => Ok((StatusCode::OK, Json(stats))),
        Err(e) => Err(database_error(e)),
    }
}
