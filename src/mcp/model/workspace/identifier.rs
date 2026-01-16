// use caretta_id::CarettaId;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{WorkspacePublicKey};
#[cfg(feature = "server")]
use crate::{mcp::model::Error, types::Database};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum WorkspaceIdentifier {
    Id(i64),
    Name(String),
    PublicKey(WorkspacePublicKey),
}

#[cfg(feature = "server")]
impl WorkspaceIdentifier {
    pub async fn to_public_key(&self, _db: &Database) -> Result<Option<WorkspacePublicKey>, Error> {
        match self {
            WorkspaceIdentifier::Id(_x) => todo!(),
            WorkspaceIdentifier::Name(_x) => todo!(),
            WorkspaceIdentifier::PublicKey(x) => Ok(Some(x.clone())),
        }
    }
}

impl std::fmt::Display for WorkspaceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceIdentifier::Id(x) => write!(f, "id: {}", x),
            WorkspaceIdentifier::Name(x) => write!(f, "name: {}", x),
            WorkspaceIdentifier::PublicKey(x) => write!(f, "public_key: {}", x),
        }
    }
}
