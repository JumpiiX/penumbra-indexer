/*
* API module for the Penumbra indexer.
*/

pub mod routes;
pub mod openapi;

use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/*
* Creates and configures the API router.
*/
pub fn create_router(pool: Pool<Postgres>) -> Router {
    let api_doc = openapi::ApiDoc::openapi();

    let api_routes = Router::new()
        .route("/blocks", get(routes::blocks::get_latest_blocks))
        .route("/blocks/:height", get(routes::blocks::get_block_by_height))
        .route("/stats", get(routes::stats::get_chain_stats))
        .route("/transactions", get(routes::transactions::get_latest_transactions))
        .route("/blocks/:height/transactions", get(routes::transactions::get_transactions_by_block_height))
        .with_state(pool);

    Router::new()
        .nest("/api", api_routes)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", api_doc)
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
}
