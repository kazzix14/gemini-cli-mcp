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


async fn run_gemini_command(args: Vec<String>) -> Result<String> {
    use tokio::process::Command;
    
    tracing::debug!("Running gemini command with args: {:?}", args);
    
    let mut cmd = Command::new("gemini");
    
    // Set environment variables from .env if they exist
    if let Ok(project) = std::env::var("GOOGLE_CLOUD_PROJECT") {
        cmd.env("GOOGLE_CLOUD_PROJECT", project);
    }
    
    let mut child = cmd
        .args(&args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("Failed to spawn gemini command")?;
    
    // Close stdin to signal EOF
    if let Some(stdin) = child.stdin.take() {
        drop(stdin);
    }
    
    let output = child.wait_with_output().await
        .context("Failed to wait for gemini command")?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    
    tracing::debug!("Command stdout: {}", stdout);
    tracing::debug!("Command stderr: {}", stderr);
    
    if output.status.success() {
        Ok(stdout)
    } else {
        anyhow::bail!(
            "Gemini command failed: {}",
            stderr
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
        Parameters(GeminiPromptArgs { prompt, model, max_tokens: _max_tokens, temperature: _temperature }): Parameters<GeminiPromptArgs>,
    ) -> Result<String, McpError> {
        let mut cmd_args = vec![];
        
        // Add prompt
        cmd_args.push("--prompt".to_string());
        cmd_args.push(prompt);
        
        // Add optional model
        if let Some(model_str) = model {
            cmd_args.push("--model".to_string());
            cmd_args.push(model_str);
        }
        
        // Note: gemini CLI doesn't seem to support max_tokens or temperature directly
        // but keeping them here for potential future support
        
        tracing::info!("Calling gemini with prompt");
        
        run_gemini_command(cmd_args).await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }
    
    
    #[tool(description = "Configure Gemini CLI settings")]
    async fn gemini_config(
        &self,
        Parameters(GeminiConfigArgs { api_key }): Parameters<GeminiConfigArgs>,
    ) -> Result<String, McpError> {
        // Note: gemini CLI configuration is typically done through environment variables
        if let Some(_key) = api_key {
            Ok("Note: Gemini API key should be set via GOOGLE_API_KEY environment variable".to_string())
        } else {
            Ok("Gemini CLI configuration:\n- API key: Set via GOOGLE_API_KEY environment variable\n- Model: Use --model flag (default: gemini-2.5-pro)".to_string())
        }
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
    // Load .env file
    dotenv::dotenv().ok();
    
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