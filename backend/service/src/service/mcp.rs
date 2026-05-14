//! MCP (Model Context Protocol) endpoint.
//!
//! Exposes an empty MCP server over Streamable HTTP at `/mcp` via the
//! `poem-mcpserver` crate. No tools are registered yet — clients can complete
//! the `initialize` handshake and observe an empty `tools/list`. OSV tools are
//! added on top in follow-up work.
//!
//! Spec: <https://modelcontextprotocol.io>

use poem::IntoEndpoint;
use poem_mcpserver::McpServer;

const SERVER_NAME: &str = "oppsy";

pub fn endpoint() -> impl IntoEndpoint {
    poem_mcpserver::streamable_http::endpoint(|_| {
        McpServer::new().with_server_info(SERVER_NAME, env!("CARGO_PKG_VERSION"))
    })
}
