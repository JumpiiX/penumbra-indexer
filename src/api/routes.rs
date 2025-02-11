use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sqlx::{Pool, Postgres};
use crate::{db, models::BlockList};

/*
 * Error response structure for API errors
 */
#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

/*
 * Handler for GET /api/blocks endpoint.
 *
 * Fetches the latest blocks from the database and returns them in a paginated format.
 * Implements error handling and proper HTTP status codes.
 *
 * @param pool PostgreSQL connection pool injected by Axum state
 * @return JSON response containing:
 *         - 200 OK with BlockList on success
 *         - 500 Internal Server Error with details on failure
 */
pub async fn get_latest_blocks(
    State(pool): State<Pool<Postgres>>,
) -> impl IntoResponse {
    match db::get_latest_blocks(&pool).await {
        Ok(blocks) => {
            let response = BlockList::new(blocks);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("Database error: {}", e),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_response)
            ).into_response()
        }
    }
}
