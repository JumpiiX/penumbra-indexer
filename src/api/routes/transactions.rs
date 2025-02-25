/*
* Transaction API module.
*
* Provides endpoints for retrieving transaction data from the database,
* including fetching recent transactions and transactions by block height.
*/

use axum::{extract::{State, Path}, http::StatusCode, Json};
use sqlx::{Pool, Postgres};
use crate::{db, models::transaction::TransactionList};
use super::common::{database_error, not_found_error, ErrorResponse};

/*
* Retrieves the latest transactions.
*
* Fetches a list of the most recent transactions.
*
* @param pool Database connection pool
* @return JSON response containing recent transactions
*/
#[utoipa::path(
    get,
    path = "/api/transactions",
    tag = "Transactions",
    responses(
        (status = 200, description = "Latest transactions retrieved successfully", body = TransactionList),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_latest_transactions(
    State(pool): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<TransactionList>), (StatusCode, Json<ErrorResponse>)> {
    match db::transactions::get_latest_transactions(&pool, 50).await {
        Ok(transactions) => {
            let summaries = transactions.into_iter()
                .map(|tx| tx.to_summary())
                .collect();
            let response = TransactionList::new(summaries);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err(database_error(e)),
    }
}

/*
* Retrieves transactions for a specific block height.
*
* Returns all transactions associated with a given block height.
*
* @param pool Database connection pool
* @param height Block height to query
* @return JSON response containing transactions for the specified block
*/
#[utoipa::path(
    get,
    path = "/api/blocks/{height}/transactions",
    tag = "Transactions",
    params(
        ("height" = i64, Path, description = "Block height to retrieve transactions for")
    ),
    responses(
        (status = 200, description = "Transactions retrieved successfully", body = TransactionList),
        (status = 404, description = "No transactions found for the specified block height", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_transactions_by_block_height(
    State(pool): State<Pool<Postgres>>,
    Path(height): Path<i64>,
) -> Result<(StatusCode, Json<TransactionList>), (StatusCode, Json<ErrorResponse>)> {
    match db::transactions::get_transactions_by_block_height(&pool, height).await {
        Ok(transactions) => {
            if transactions.is_empty() {
                return Err(not_found_error(format!("No transactions found for block at height {}", height)));
            }
            let summaries = transactions.into_iter()
                .map(|tx| tx.to_summary())
                .collect();
            let response = TransactionList::new(summaries);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err(database_error(e)),
    }
}