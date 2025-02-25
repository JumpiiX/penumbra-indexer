/*
* OpenAPI documentation module for the Penumbra indexer.
*
* Defines the OpenAPI specification for the API endpoints,
* including paths, request parameters, and response schemas.
*/

use utoipa::{OpenApi, ToSchema};
use chrono::{DateTime, Utc};

#[derive(ToSchema)]
#[schema(value_type = String, format = "date-time", example = "2025-02-25T12:34:56Z")]
struct DateTimeSchema(DateTime<Utc>);

#[derive(OpenApi)]
#[openapi(
    paths(
        // Block routes
        crate::api::routes::blocks::get_latest_blocks,
        crate::api::routes::blocks::get_block_by_height,

        // Transaction routes
        crate::api::routes::transactions::get_latest_transactions,
        crate::api::routes::transactions::get_transactions_by_block_height,

        // Statistics routes
        crate::api::routes::stats::get_chain_stats,
    ),
    components(
        schemas(
            // Block schemas
            crate::models::block::StoredBlock,
            crate::models::block::BlockSummary,
            crate::models::block::BlockList,

            // Transaction schemas
            crate::models::transaction::Transaction,
            crate::models::transaction::TransactionSummary,
            crate::models::transaction::TransactionList,

            // Stats schemas
            crate::models::stats::StatsResponse,
            crate::models::stats::CurrentBlockStats,
            crate::models::stats::TransactionStats,
            crate::models::stats::BurnStats,
            crate::models::stats::ChartPoint,

            // Error response schema
            crate::api::routes::common::ErrorResponse,

            // Custom types
            DateTimeSchema
        )
    ),
    tags(
        (name = "Blocks", description = "Block data endpoints"),
        (name = "Transactions", description = "Transaction data endpoints"),
        (name = "Statistics", description = "Blockchain statistics endpoints")
    ),
    info(
        title = "Penumbra Blockchain API",
        version = "1.0.0",
        description = "API for querying Penumbra blockchain data",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "PK LABS",
            url = "https://www.pklabs.me/"
        )
    )
)]
pub struct ApiDoc;
