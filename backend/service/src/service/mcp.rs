//! Minimal MCP (Model Context Protocol) endpoint.
//!
//! Handles just enough of the JSON-RPC 2.0 surface for an MCP client to
//! complete the `initialize` handshake and discover that no tools are exposed
//! yet. OSV tools and resources are added on top of this in follow-up work.
//!
//! Spec: <https://modelcontextprotocol.io>

use poem::{IntoResponse, Response, handler, http::StatusCode, web::Json};
use serde_json::{Value, json};

const PROTOCOL_VERSION: &str = "2024-11-05";
const SERVER_NAME: &str = "oppsy";

#[handler]
#[allow(clippy::unused_async)]
pub async fn handle(Json(request): Json<Value>) -> Response {
    let id = request.get("id").cloned();
    let method = request
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or_default();

    if id.is_none() {
        return StatusCode::ACCEPTED.into_response();
    }

    let body = match method {
        "initialize" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": { "tools": {} },
                "serverInfo": {
                    "name": SERVER_NAME,
                    "version": env!("CARGO_PKG_VERSION"),
                },
            },
        }),
        "tools/list" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": { "tools": [] },
        }),
        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32601,
                "message": format!("Method not found: {method}"),
            },
        }),
    };

    Json(body).into_response()
}
