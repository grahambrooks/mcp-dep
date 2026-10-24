use anyhow::Result;
use rmcp::{
    handler::server::router::tool::ToolRouter,
    model::{ContentBlock, ErrorData as McpError, *},
    tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};

#[derive(Debug, Clone)]
pub struct PingServer {
    tool_router: ToolRouter<PingServer>,
}

impl Default for PingServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl PingServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// A simple ping tool that responds with pong
    #[tool(description = "A simple ping tool that responds with pong")]
    async fn ping(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![ContentBlock::text("pong")]))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for PingServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2024_11_05)
            .with_server_info(Implementation::from_build_env())
            .with_instructions("A simple MCP server with a ping tool that responds with pong")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let service = PingServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
