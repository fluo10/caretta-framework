use std::time::Duration;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::mcp::model::DeviceIdentifier;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DevPingRequest {
    pub target: DeviceIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DevPingResponse {
    pub rtt: Duration,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DevToolsRequest;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DevToolsResponse;