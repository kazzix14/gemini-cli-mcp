# Gemini CLI MCP Server

This is an MCP (Model Context Protocol) server that exposes Gemini CLI functionality through tools.

## Features

The MCP server provides two tools:

1. **gemini_prompt** - Send a prompt to the Gemini CLI
   - Parameters:
     - `prompt` (required): The prompt to send to Gemini
     - `model` (optional): The model to use
     - `max_tokens` (optional): Maximum number of tokens
     - `temperature` (optional): Temperature for sampling

2. **gemini_config** - Configure Gemini CLI settings
   - Parameters:
     - `api_key` (optional): API key for Gemini

## Prerequisites

- Rust (for building)
- Gemini CLI installed and available in PATH

## Configuration

Create a `.env` file in the project root with your Google Cloud project ID:

```
GOOGLE_CLOUD_PROJECT=your-project-id
```

## Building

```bash
cargo build --release
```

## Usage

The MCP server communicates via stdio. You can integrate it with any MCP-compatible client.

### Including File Contents in Prompts

When using the `gemini_prompt` tool, you can reference files that should be included in the context. For example:

```
"Please analyze the code in src/main.rs and suggest improvements"
"Based on the error in logs/error.log, what's the issue?"
"Review the implementation in lib/parser.js and optimize it"
```

**How it works:**
- The MCP client (like Claude) will read the referenced files
- The file contents will be included in the prompt sent to Gemini
- You can reference multiple files in a single prompt

**Tips:**
- Be specific about file paths to help the client find the right files
- You can ask Gemini to compare multiple files or analyze relationships between them
- File contents are included automatically by the MCP client, not by this server

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

### Example Usage in Claude Code

Once integrated, you can use commands like:

```
Ask Gemini to analyze the performance bottlenecks in src/api/handlers.ts
```

```
Using Gemini, refactor the database connection logic in db/connection.go to use connection pooling
```

```
Have Gemini review the test coverage in test/unit/*.spec.js and suggest missing test cases
```

Claude will automatically:
1. Read the referenced files
2. Include their contents in the context
3. Send the complete prompt to Gemini via this MCP server
4. Return Gemini's response

## Development

The server is built using the rmcp Rust SDK and uses the MCP protocol for communication.

## License

AGPLv3
