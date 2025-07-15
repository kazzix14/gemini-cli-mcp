# Gemini CLI MCP Server

This is an MCP (Model Context Protocol) server that exposes Gemini CLI functionality through tools.

## Features

The MCP server provides three tools:

1. **gemini_prompt** - Send a prompt to the Gemini CLI
   - Parameters:
     - `prompt` (required): The prompt to send to Gemini
     - `model` (optional): The model to use
     - `max_tokens` (optional): Maximum number of tokens
     - `temperature` (optional): Temperature for sampling

2. **gemini_list_models** - List available Gemini models

3. **gemini_config** - Configure Gemini CLI settings
   - Parameters:
     - `api_key` (optional): API key for Gemini

## Prerequisites

- Rust (for building)
- Gemini CLI installed and available in PATH

## Building

```bash
cargo build --release
```

## Usage

The MCP server communicates via stdio. You can integrate it with any MCP-compatible client.

### Testing

You can test the server with a simple initialize request:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}' | ./target/release/gemini-cli-mcp
```

### Integration with Claude

To use with Claude, add the following to your Claude settings.json:

```json
{
  "mcpServers": {
    "gemini-cli": {
      "command": "/path/to/gemini-cli-mcp/target/release/gemini-cli-mcp"
    }
  }
}
```

## Development

The server is built using the rmcp Rust SDK and uses the MCP protocol for communication.

## License

[Your license here]