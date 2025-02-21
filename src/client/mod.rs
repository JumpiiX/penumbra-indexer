/*
 * Penumbra blockchain client module.
 *
 * Provides interfaces for interacting with the Penumbra blockchain
 * through RPC endpoints and manages block synchronization.
 */

pub mod models;
pub mod rpc;
pub mod sync;

pub use sync::PenumbraClient;
