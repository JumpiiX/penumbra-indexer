/*
* API module for the Penumbra indexer.
*/

pub mod routes;

use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};

/*
* Creates and configures the API router.
*/
pub fn create_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/blocks", get(routes::blocks::get_latest_blocks))
        .route("/api/blocks/:height", get(routes::blocks::get_block_by_height))
        .route("/api/stats", get(routes::stats::get_chain_stats))
        .route("/api/transactions", get(routes::transactions::get_latest_transactions))
        .route("/api/blocks/:height/transactions", get(routes::transactions::get_transactions_by_block_height))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(pool)
}
