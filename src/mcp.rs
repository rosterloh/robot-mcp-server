use anyhow::Result;
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ErrorData as McpError, *},
    schemars::JsonSchema,
    tool, tool_handler, tool_router,
};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct DnsService {
    tool_router: ToolRouter<DnsService>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DnsLookupRequest {
    #[schemars(description = "The domain name to lookup")]
    pub domain: String,
}

impl DnsService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for DnsService {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl DnsService {
    #[tool(description = "Perform DNS lookup for a domain name")]
    async fn dns_lookup(
        &self,
        Parameters(request): Parameters<DnsLookupRequest>,
    ) -> Result<CallToolResult, McpError> {
        let response = reqwest::get(format!(
            "https://api.hackertarget.com/dnslookup/?q={}",
            request.domain
        ))
        .await
        .map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::from(format!("Request failed: {}", e)),
            data: None,
        })?;

        let text = response.text().await.map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::from(format!("Failed to read response: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(text)]))
    }
}

#[tool_handler]
impl ServerHandler for DnsService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "A DNS lookup service that queries domain information using the HackerTarget API. Use the dns_lookup tool to perform DNS lookups for any domain name.".to_string()
            ),
            ..Default::default()
        }
    }
}
