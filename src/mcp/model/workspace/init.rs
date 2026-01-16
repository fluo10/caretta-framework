use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::entity::workspace_config;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceInitRequest{
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceInitResponse{
    pub workspace: workspace_config::Model

}