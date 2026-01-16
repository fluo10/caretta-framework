use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{entity::workspace_config, types::DocTicket};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceJoinRequest {
    pub name: String,
    pub ticket: DocTicket
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceJoinResponse{
    pub workspace: workspace_config::Model
}
