/*
* Database and API model definitions.
*
* Defines and organizes modules for blockchain-related data models,
* including blocks, transactions, and statistics. These models are
* utilized for database interactions and API responses.
*/

pub mod block;
pub mod transaction;
pub mod stats;

pub use block::StoredBlock;
pub use transaction::Transaction;
pub use stats::{StatsResponse, CurrentBlockStats, TransactionStats, BurnStats, ChartPoint};
