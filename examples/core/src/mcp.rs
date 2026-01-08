use caretta_framework::mcp::{Api as _, ServiceContext, model::*};
use rmcp::{
    ErrorData, Json,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{Meta, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

#[derive(Clone, Debug, caretta_framework::mcp::Service)]
pub struct Service {
    #[service_context]
    pub context: &'static ServiceContext,
    pub tool_router: ToolRouter<Service>,
}

impl Service {
    pub fn new(context: &'static ServiceContext) -> Self {
        Self {
            context,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler(meta = Meta(rmcp::object!({"tool_meta_key": "tool_meta_value"})))]
impl rmcp::ServerHandler for Service {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "A device and user manager for data syncronization via iroh p2p".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

impl From<&'static ServiceContext> for Service {
    fn from(value: &'static ServiceContext) -> Self {
        Self::new(value)
    }
}
