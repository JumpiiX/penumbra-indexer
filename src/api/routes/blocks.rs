/*
* Block API module.
*
* Provides endpoints for retrieving blockchain block data from the database,
* including fetching recent blocks and specific blocks by height.
*/

use axum::{extract::{State, Path}, http::StatusCode, Json};
use sqlx::{Pool, Postgres};
use crate::{db, models::block::{BlockList, StoredBlock}};
use super::common::{database_error, not_found_error, ErrorResponse};

/*
* Retrieves the latest blocks.
*
* Fetches a list of the most recent blocks in descending order by height.
*
* @param pool Database connection pool
* @return JSON response containing recent blocks
*/
pub async fn get_latest_blocks(
    State(pool): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<BlockList>), (StatusCode, Json<ErrorResponse>)> {
    match db::blocks::get_latest_blocks(&pool).await {
        Ok(blocks) => {
            let summaries = blocks.into_iter()
                .map(|block| block.to_summary())
                .collect();
            let response = BlockList::new(summaries);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err(database_error(e)),
    }
}

/*
* Retrieves a specific block by its height.
*
* Returns the block details for the given height if it exists.
*
* @param pool Database connection pool
* @param height Block height to query
* @return JSON response containing the requested block data
*/
pub async fn get_block_by_height(
    State(pool): State<Pool<Postgres>>,
    Path(height): Path<i64>,
) -> Result<(StatusCode, Json<StoredBlock>), (StatusCode, Json<ErrorResponse>)> {
    match db::blocks::get_block_by_height(&pool, height).await {
        Ok(Some(block)) => Ok((StatusCode::OK, Json(block))),
        Ok(None) => Err(not_found_error(format!("Block at height {} not found", height))),
        Err(e) => Err(database_error(e)),
    }
}
