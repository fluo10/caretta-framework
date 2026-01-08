#[cfg(feature = "server")]
use iroh::discovery::DiscoveryError;
use rmcp::ErrorData;
use serde::{Deserialize, Serialize};

use crate::mcp::model::DeviceIdentifier;

/// Error returned from McpServer.
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Target device not found: {0}")]
    DeviceNotFound(DeviceIdentifier),
    #[error("Device discovery failed: {0}")]
    DeviceDiscoveryFailed(String),
    #[error("iroh-ping failed: {0}")]
    DevicePingFailed(String),
}

impl From<Error> for ErrorData {
    fn from(value: Error) -> Self {
        let data = match serde_json::to_value(&value) {
            Ok(x) => x,
            Err(_) => serde_json::Value::String(format!("{:?}", &value)),
        };
        let msg = format!("{:?}", &value);
        ErrorData::internal_error(msg, Some(data))
    }
}

#[cfg(feature = "server")]
impl From<DiscoveryError> for Error {
    fn from(value: DiscoveryError) -> Self {
        match value {
            DiscoveryError::NoResults { endpoint_id, meta:_ } => {
                Error::DeviceNotFound(DeviceIdentifier::PublicKey(endpoint_id.into()))
            }
            x => Error::DeviceDiscoveryFailed(format!("{:?}", x)),
        }
    }
}
