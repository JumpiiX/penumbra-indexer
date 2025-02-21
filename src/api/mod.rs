pub mod routes;

use axum::{
    Router,
    routing::get,
};
use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};

/*
 * Router configuration for the Penumbra API.
 *
 * Sets up route handling with CORS support and database connection state.
 * Defines API endpoints for retrieving blockchain data.
 *
 * @function create_router
 * @param {Pool<Postgres>} pool - Database connection pool
 * @returns {Router} Configured Axum router
 */
pub fn create_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/blocks", get(routes::get_latest_blocks))
        .route("/api/blocks/:height", get(routes::get_block_by_height))
        .route("/api/stats", get(routes::get_chain_stats))
        .route("/api/transactions", get(routes::get_latest_transactions))
        .route("/api/blocks/:height/transactions", get(routes::get_transactions_by_block_height))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(pool)
}
