use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sqlx::{Pool, Postgres};
use crate::{db, models::BlockList};

/*
* Handler for retrieving the latest blockchain blocks.
*
* This function fetches the most recent blocks from the database
* and returns them as a JSON response. It handles potential
* database errors and provides a consistent API endpoint.
*
* @function get_latest_blocks
* @param {Pool<Postgres>} pool - Database connection pool
* @returns {IntoResponse} JSON response with latest blocks or error
*/
pub async fn get_latest_blocks(
    State(pool): State<Pool<Postgres>>,
) -> impl IntoResponse {
    match db::get_latest_blocks(&pool).await {
        Ok(blocks) => {
            let total_count = blocks.len() as i64;
            let response = BlockList {
                blocks,
                total_count,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Database error: {}", e)),
            )
                .into_response()
        }
    }
}
