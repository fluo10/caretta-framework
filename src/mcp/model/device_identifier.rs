// use caretta_id::CarettaId;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::DevicePublicKey;
#[cfg(feature = "server")]
use crate::{mcp::model::{Error}, types::Database};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum DeviceIdentifier {
    // Id(CarettaId),
    // Name(String),
    PublicKey(DevicePublicKey),
}

#[cfg(feature = "server")]
impl DeviceIdentifier {
    pub async fn to_public_key(&self, _db: &Database) -> Result<Option<DevicePublicKey>, Error> {
        match self {
            // DeviceIdentifier::Id(x) => todo!(),
            // DeviceIdentifier::Name(x) => todo!(),
            DeviceIdentifier::PublicKey(x) => Ok(Some(x.clone())),
        }
    }
}

impl std::fmt::Display for DeviceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // DeviceIdentifier::Id(x) => write!(f, "id: {}", x),
            // DeviceIdentifier::Name(x) => write!(f, "name: {}", x),
            DeviceIdentifier::PublicKey(x) => write!(f, "public_key: {}", x),
        }
    }
}
