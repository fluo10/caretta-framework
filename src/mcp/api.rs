#[cfg(feature = "devtools")]
use crate::mcp::model::{DevPingRequest, DevPingResponse};

#[async_trait::async_trait]
pub trait Api {
    type Error;

    /// Ping another device.
    /// 
    /// This function is for testing so works between unauthorized devices.
    #[cfg(feature = "devtools")]
    async fn dev_ping(
        &self,
        params: DevPingRequest,
    ) -> Result<DevPingResponse, Self::Error>;

}
