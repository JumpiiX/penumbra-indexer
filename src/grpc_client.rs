use anyhow::Result;
use tonic::transport::Channel;
use proto::block_service_client::BlockServiceClient;
use proto::{GetLatestBlockRequest, GetBlockByHeightRequest};

/// Proto module containing generated code and serialization implementations
pub mod proto {
    use serde::{Serialize};
    use chrono::{DateTime, Utc};

    tonic::include_proto!("penumbra.core.component.block.v1");

    /// Wrapper for Timestamp to implement Serialize
    #[derive(Debug)]
    pub struct TimestampWrapper(pub prost_types::Timestamp);

    impl From<&prost_types::Timestamp> for TimestampWrapper {
        fn from(ts: &prost_types::Timestamp) -> Self {
            TimestampWrapper(ts.clone())
        }
    }

    impl Serialize for TimestampWrapper {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let dt = DateTime::from_timestamp(self.0.seconds, self.0.nanos as u32)
                .ok_or_else(|| serde::ser::Error::custom("invalid timestamp"))?;
            dt.serialize(serializer)
        }
    }

    /// Implements serialization for Block
    impl Serialize for Block {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut state = serializer.serialize_struct("Block", 4)?;
            state.serialize_field("height", &self.height)?;
            state.serialize_field("hash", &hex::encode(&self.hash))?;
            state.serialize_field("timestamp", &TimestampWrapper::from(self.timestamp.as_ref().unwrap_or(&prost_types::Timestamp::default())))?;
            state.serialize_field("transactions", &self.transactions)?;
            state.end()
        }
    }

    /// Implements serialization for Transaction
    impl Serialize for Transaction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut state = serializer.serialize_struct("Transaction", 2)?;
            state.serialize_field("hash", &hex::encode(&self.hash))?;
            state.serialize_field("data", &hex::encode(&self.data))?;
            state.end()
        }
    }
}

/// GrpcClient handles communication with the Penumbra blockchain node
#[derive(Debug)]
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