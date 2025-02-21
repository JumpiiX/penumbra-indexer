use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sqlx::{Pool, Postgres};
use crate::{db, models::BlockList, models::TransactionList};
use axum::extract::Path;
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


/*
 * Handler for GET /api/blocks/:height endpoint.
 *
 * Retrieves a specific block by its height. Returns appropriate HTTP status:
 * - 200 OK with block data if found
 * - 404 Not Found if block doesn't exist
 * - 500 Internal Server Error on database errors
 *
 * @param pool PostgreSQL connection pool injected by Axum state
 * @param height Block height from URL parameter
 * @return Response with either block data or error information
 */
pub async fn get_block_by_height(
    State(pool): State<Pool<Postgres>>,
    Path(height): Path<i64>,
) -> impl IntoResponse {
    match db::get_block_by_height(&pool, height).await {
        Ok(Some(block)) => {
            (StatusCode::OK, Json(block)).into_response()
        }
        Ok(None) => {
            let error_response = ErrorResponse {
                error: format!("Block at height {} not found", height),
                code: StatusCode::NOT_FOUND.as_u16(),
            };
            (StatusCode::NOT_FOUND, Json(error_response)).into_response()
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("Database error: {}", e),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

/*
 * Handler for GET /api/stats endpoint.
 *
 * Provides statistical information about the blockchain state
 * including block counts, transaction volumes, and performance metrics.
 *
 * @param pool PostgreSQL connection pool injected by Axum state
 * @return JSON response with chain statistics or error information
 */
pub async fn get_chain_stats(
    State(pool): State<Pool<Postgres>>,
) -> impl IntoResponse {
    match db::get_chain_stats(&pool).await {
        Ok(stats) => {
            (StatusCode::OK, Json(stats)).into_response()
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("Database error: {}", e),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

pub async fn get_latest_transactions(
    State(pool): State<Pool<Postgres>>,
) -> impl IntoResponse {
    match db::get_latest_transactions(&pool, 50).await {
        Ok(transactions) => {
            let response = TransactionList::new(transactions);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("Database error: {}", e),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

pub async fn get_transactions_by_block_height(
    State(pool): State<Pool<Postgres>>,
    Path(height): Path<i64>,
) -> impl IntoResponse {
    match db::get_transactions_by_block_height(&pool, height).await {
        Ok(transactions) => {
            if transactions.is_empty() {
                let error_response = ErrorResponse {
                    error: format!("No transactions found for block at height {}", height),
                    code: StatusCode::NOT_FOUND.as_u16(),
                };
                return (StatusCode::NOT_FOUND, Json(error_response)).into_response();
            }

            let response = TransactionList::new(transactions);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("Database error: {}", e),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}