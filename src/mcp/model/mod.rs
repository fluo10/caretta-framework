#[cfg(feature = "devtools")]
mod dev;
#[cfg(feature = "devtools")]
pub use dev::*;

mod device_identifier;
mod device_info;
mod error;

pub use device_identifier::*;
pub use device_info::*;
pub use error::Error;

mod workspace_identifier;
pub use workspace_identifier::*;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::DocTicket;


#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceGetRequest {
    pub target: DeviceIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceGetResponse {
    pub info: DeviceInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceListRequest;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceListResponse {
    pub info: Vec<DeviceInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceRemoveRequest {
    pub target: DeviceIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceRemoveResponse;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceInviteRequest;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceInviteResponze {
    pub ticket: DocTicket,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceInitRequest;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceInitResponse;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceJoinRequest {
    pub ticket: DocTicket,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceJoinResponse;
