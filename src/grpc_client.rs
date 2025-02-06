use anyhow::Result;
use tonic::transport::Channel;

// Import the generated proto code
pub mod proto {
    tonic::include_proto!("penumbra.core.component.block.v1");
}

use proto::block_service_client::BlockServiceClient;
use proto::{GetLatestBlockRequest, GetBlockByHeightRequest};

pub struct GrpcClient {
    client: BlockServiceClient<Channel>,
}

impl GrpcClient {
    pub async fn connect(addr: &str) -> Result<Self> {
        let channel = Channel::from_shared(addr.to_string())?
            .connect()
            .await?;

        let client = BlockServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn get_latest_block(&mut self) -> Result<proto::Block> {
        let request = tonic::Request::new(GetLatestBlockRequest {});
        let response = self.client.get_latest_block(request).await?;
        Ok(response.into_inner().block.unwrap())
    }

    pub async fn get_block_by_height(&mut self, height: u64) -> Result<proto::Block> {
        let request = tonic::Request::new(GetBlockByHeightRequest { height });
        let response = self.client.get_block_by_height(request).await?;
        Ok(response.into_inner().block.unwrap())
    }
}