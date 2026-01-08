use rmcp::{
    RoleClient, ServiceError, ServiceExt as _,
    model::{
        CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation,
        InitializeRequestParam,
    },
    service::RunningService,
    transport::StreamableHttpClientTransport,
};

use crate::mcp::{Api, ClientError, model::*};

pub struct Client {
    service: RunningService<RoleClient, InitializeRequestParam>,
}

impl Client {
    pub async fn connect(
        url: &str,
        client_info: Implementation,
    ) -> Result<Self, crate::error::Error> {
        let client_info = InitializeRequestParam {
            client_info: client_info,
            ..Default::default()
        };
        let transport = StreamableHttpClientTransport::from_uri(url);
        let client = client_info.serve(transport).await.inspect_err(|e| {
            tracing::error!("client error: {:?}", e);
        })?;

        // Initialize
        let server_info = client.peer_info();
        tracing::info!("Connected to server: {server_info:#?}");

        Ok(Self { service: client })
    }

    // Close client connection
    pub async fn close(self) -> Result<(), tokio::task::JoinError> {
        self.service.cancel().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Api for Client {
    type Error = ClientError;

    /// Send ping message to another node.
    #[cfg(feature = "devtools")]
    async fn dev_ping(
        &self,
        params: DevPingRequest,
    ) -> Result<DevPingResponse, Self::Error> {
        let tool_result = self
            .service
            .call_tool(CallToolRequestParam {
                name: "device_ping".into(),
                arguments: serde_json::json!(params).as_object().cloned(),
            })
            .await?;
        tracing::info!("Tool result: {tool_result:#?}");
        let result = tool_result
            .into_typed()
            .map_err(|e| ClientError::DeserializeResponse(e))?;
        Ok(result)
    }
}
