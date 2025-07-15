use anyhow::{Context, Result};
use rmcp::{
    tool, tool_handler, tool_router,
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    Error as McpError,
};
use serde::Deserialize;
use std::future::Future;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GeminiPromptArgs {
    #[schemars(description = "The prompt to send to Gemini")]
    prompt: String,
    #[schemars(description = "The model to use (optional)")]
    #[serde(default)]
    model: Option<String>,
    #[schemars(description = "Maximum number of tokens (optional)")]
    #[serde(default)]
    max_tokens: Option<u32>,
    #[schemars(description = "Temperature for sampling (optional)")]
    #[serde(default)]
    temperature: Option<f32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GeminiConfigArgs {
    #[schemars(description = "API key for Gemini (optional)")]
    #[serde(default)]
    api_key: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct Empty {}

async fn run_gemini_command(args: Vec<String>) -> Result<String> {
    use tokio::process::Command;
    
    let output = Command::new("gemini")
        .args(&args)
        .output()
        .await
        .context("Failed to execute gemini command")?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        anyhow::bail!(
            "Gemini command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
}

#[derive(Clone)]
struct GeminiServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl GeminiServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
    
    #[tool(description = "Send a prompt to the Gemini CLI")]
    async fn gemini_prompt(
        &self,
        Parameters(GeminiPromptArgs { prompt, model, max_tokens, temperature }): Parameters<GeminiPromptArgs>,
    ) -> Result<String, McpError> {
        let mut cmd_args = vec!["--prompt".to_string(), prompt];
        
        if let Some(model_str) = model {
            cmd_args.push("--model".to_string());
            cmd_args.push(model_str);
        }
        
        if let Some(tokens) = max_tokens {
            cmd_args.push("--max-tokens".to_string());
            cmd_args.push(tokens.to_string());
        }
        
        if let Some(temp) = temperature {
            cmd_args.push("--temperature".to_string());
            cmd_args.push(temp.to_string());
        }
        
        run_gemini_command(cmd_args).await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }
    
    #[tool(description = "List available Gemini models")]
    async fn gemini_list_models(
        &self,
        Parameters(_args): Parameters<Empty>,
    ) -> Result<String, McpError> {
        run_gemini_command(vec!["models".to_string()]).await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }
    
    #[tool(description = "Configure Gemini CLI settings")]
    async fn gemini_config(
        &self,
        Parameters(GeminiConfigArgs { api_key }): Parameters<GeminiConfigArgs>,
    ) -> Result<String, McpError> {
        let mut cmd_args = vec!["config".to_string()];
        
        if let Some(key) = api_key {
            cmd_args.push("--api-key".to_string());
            cmd_args.push(key);
        }
        
        run_gemini_command(cmd_args).await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }
}

#[tool_handler]
impl ServerHandler for GeminiServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("MCP server that exposes Gemini CLI functionality through tools".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), McpError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    
    tracing::info!("Starting Gemini CLI MCP server");
    
    use rmcp::transport::io::stdio;
    
    let service = GeminiServer::new()
        .serve(stdio())
        .await
        .map_err(|e| McpError::internal_error(format!("Failed to start server: {:?}", e), None))?;
    
    service.waiting().await.map_err(|e| McpError::internal_error(format!("Server error: {:?}", e), None))?;
    
    Ok(())
}