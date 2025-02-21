/*
* Database connection and access module for the Penumbra indexer.
*/

pub mod schema;
pub mod blocks;
pub mod transactions;
pub mod stats;

use sqlx::{Pool, Postgres};

/* Maximum number of database connections */
const MAX_DB_CONNECTIONS: u32 = 5;

/*
* Initializes the database connection and creates all required tables.
*/
pub async fn init_db(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    // Create and configure the connection pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(database_url)
        .await?;

    // Initialize database schema
    schema::initialize_schema(&pool).await?;

    Ok(pool)
}
