/*
* Penumbra RPC client implementation.
*
* Handles low-level communication with the Penumbra blockchain RPC endpoints,
* including request formatting and response parsing.
*/

use reqwest::Client as HttpClient;
use std::error::Error;
use std::time::Duration;
use crate::client::models::{BlockResponse, StatusResponse};

/* Default timeout for HTTP requests in seconds */
const DEFAULT_TIMEOUT: u64 = 30;

/*
* Client for making RPC requests to the Penumbra blockchain.
*/
#[derive(Debug, Clone)]
pub struct RpcClient {
    client: HttpClient,
    base_url: String,
}

impl RpcClient {
    /*
    * Creates a new RPC client instance.
    */
    pub fn new(base_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .connect_timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.to_string(),
        })
    }

    /*
    * Fetches the current node status.
    */
    pub async fn get_status(&self) -> Result<StatusResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/status", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    /*
    * Fetches a block at the specified height.
    */
    pub async fn get_block(&self, height: u64) -> Result<BlockResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/block?height={}", self.base_url, height);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }
}
