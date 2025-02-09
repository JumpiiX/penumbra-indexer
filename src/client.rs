use anyhow::{Result, Context};
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request, Status,
};
use std::time::Duration;
use crate::proto::penumbra::core::component::compact_block::v1::{
    query_service_client::QueryServiceClient,
    CompactBlockRangeRequest,
    CompactBlock,
};

pub struct Client {
    client: QueryServiceClient<Channel>,
    endpoint: String,
}

impl Client {
    /// Creates a new client with the specified endpoint
    pub async fn connect(endpoint: &str) -> Result<Self> {
        println!("Connecting to {}...", endpoint);

        let channel = Self::create_channel(endpoint).await?;

        Ok(Self {
            client: QueryServiceClient::new(channel),
            endpoint: endpoint.to_string(),
        })
    }

    /// Creates a channel with proper configuration
    async fn create_channel(endpoint_url: &str) -> Result<Channel> {
        let mut config = Channel::from_shared(endpoint_url.to_string())
            .context("Failed to create channel builder")?
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .concurrency_limit(8)
            .tcp_keepalive(Some(Duration::from_secs(60)));

        if endpoint_url.starts_with("https://") {
            config = config.tls_config(ClientTlsConfig::new())?;
        }

        // Connect
        let channel = config
            .connect()
            .await
            .context("Failed to connect to endpoint")?;

        Ok(channel)
    }

    /// Gets a range of blocks with optional streaming
    pub async fn get_block_range(
        &mut self,
        start_height: u64,
        end_height: u64,
        keep_alive: bool
    ) -> Result<()> {
        println!("Fetching blocks from {} to {} (keep_alive: {})",
                 start_height, end_height, keep_alive);

        let request = Request::new(CompactBlockRangeRequest {
            start_height,
            end_height,
            keep_alive,
        });

        let mut stream = self.client
            .compact_block_range(request)
            .await
            .context("Failed to get block range response")?
            .into_inner();

        let mut blocks_received = 0;
        while let Some(response) = stream.message().await? {
            if let Some(block) = response.compact_block {
                self.print_block_info(&block);
                blocks_received += 1;
            }
        }

        println!("Received {} blocks in total", blocks_received);
        Ok(())
    }

    /// Gets the latest blocks (default: last 10)
    pub async fn get_latest_blocks(&mut self) -> Result<()> {
        self.get_block_range(0, 10, false).await
    }

    /// Stream new blocks as they are created
    pub async fn stream_new_blocks(&mut self) -> Result<()> {
        println!("Starting to stream new blocks...");
        self.get_block_range(0, 0, true).await
    }

    /// Prints formatted block information
    fn print_block_info(&self, block: &CompactBlock) {
        println!("\n=== Block {} ===", block.height);
        println!("└─ Block Root: 0x{}", hex::encode(&block.block_root));
        println!("└─ Epoch Root: 0x{}", hex::encode(&block.epoch_root));
        println!("└─ Proposal Started: {}", block.proposal_started);
        println!("└─ App Parameters Updated: {}", block.app_parameters_updated);
        println!("└─ Epoch Index: {}", block.epoch_index);
    }

    /// Handles common gRPC errors
    fn handle_error(&self, status: &Status) -> Result<()> {
        match status.code() {
            tonic::Code::Unavailable => {
                println!("Service unavailable. The node might be down or unreachable.");
                println!("Endpoint: {}", self.endpoint);
            }
            tonic::Code::InvalidArgument => {
                println!("Invalid argument provided to the request.");
            }
            tonic::Code::NotFound => {
                println!("Requested data not found.");
            }
            tonic::Code::Internal => {
                println!("Internal server error occurred.");
            }
            _ => {
                println!("Unexpected error: {:?}", status);
            }
        }
        Err(anyhow::anyhow!("gRPC error: {}", status))
    }
}