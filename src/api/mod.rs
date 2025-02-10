pub mod routes;

use axum::{
    Router,
    routing::get,
};
use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};

pub fn create_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/blocks", get(routes::get_latest_blocks))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(pool)
}