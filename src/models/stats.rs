/*
* Stats model definitions.
*
* Defines entities for storing blockchain statistics and their serialization
* properties. Includes the core ChainStats model which maps to the
* database schema and provides relevant blockchain metrics.
*/

use serde::Serialize;
use sqlx::FromRow;

/*
* Represents aggregated blockchain statistics stored in the database.
*
* Maps directly to the 'chain_stats' table via SQLx's FromRow trait.
* Contains key blockchain metrics such as block count, active validators,
* transaction statistics, and average block time.
*/
#[derive(Debug, Serialize, FromRow)]
pub struct ChainStats {
    /* Total number of blocks produced */
    pub total_blocks: i64,

    /* Number of currently active validators */
    pub active_validators: i64,

    /* Total number of transactions recorded */
    pub total_transactions: i64,

    /* Average time taken to produce a block (optional) */
    pub avg_block_time: Option<f64>,
}
