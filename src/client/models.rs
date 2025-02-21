/*
* Penumbra blockchain data models.
*
* Contains structures that represent blockchain data as returned
* by the Tendermint RPC API for the Penumbra blockchain.
*/

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/*
* Response wrapper for block-related RPC calls.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockResponse {
    pub result: BlockResult,
}

/*
* Container for block data and metadata.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockResult {
    pub block: Block,
    pub block_id: BlockId,
}

/*
* Represents a block in the Penumbra blockchain.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub data: BlockData,
}

/*
* Header information for a block.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockHeader {
    pub height: String,
    pub time: DateTime<Utc>,
    pub last_block_id: Option<BlockId>,
    pub proposer_address: String,
}

/*
* Unique identifier for a block.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockId {
    pub hash: String,
}

/*
* Contains the actual block data including transactions.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockData {
    pub txs: Option<Vec<String>>,
}

/*
* Response structure for node status queries.
*/
#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub result: NodeStatus,
}

/*
* Contains node-specific status information.
*/
#[derive(Debug, Deserialize)]
pub struct NodeStatus {
    pub sync_info: SyncInfo,
}

/*
* Information about the node's synchronization status.
*/
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SyncInfo {
    pub latest_block_height: String,
    pub latest_block_time: DateTime<Utc>,
    pub catching_up: bool,
}
