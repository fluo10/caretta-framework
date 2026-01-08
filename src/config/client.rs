use rmcp::model::Implementation;

use crate::{
    config::{LogConfig, McpConfig},
    mcp::client::Client,
};

/// A Config for desktop client
///
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub mcp: McpConfig,
    pub log: LogConfig,
}

impl ClientConfig {
    pub async fn spawn_client(self, client_info: Implementation) -> Client {
        Client::connect(&self.mcp.endpoint_url, client_info)
            .await
            .unwrap()
    }
}
