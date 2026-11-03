# mcp-ping

A simple MCP (Model Context Protocol) server written in Rust with a single `ping` tool that responds with "pong". The
purpose of this repository is to provide an example of how to package and distribute MCP Servers using MCP Bundles (
MCPB).

MCPB Project and specification https://github.com/modelcontextprotocol/mcpb

## Installation

### Using MCPB Bundle (Recommended)

Download the `.mcpb` package for your platform from the [Releases](https://github.com/grahambrooks/mcp-dep/releases) page:

| Platform | Architecture  | Package                           |
|----------|---------------|-----------------------------------|
| macOS    | Intel         | `mcp-ping-*-darwin-x86_64.mcpb`   |
| macOS    | Apple Silicon | `mcp-ping-*-darwin-aarch64.mcpb`  |
| Linux    | x86_64        | `mcp-ping-*-linux-x86_64.mcpb`    |
| Linux    | ARM64         | `mcp-ping-*-linux-aarch64.mcpb`   |
| Windows  | x86_64        | `mcp-ping-*-win32-x86_64.mcpb`    |

**One-click install (Claude Desktop)**: Double-click the `.mcpb` file to install directly into Claude Desktop on macOS or Windows.

**CLI install**: Use the [mcpb CLI](https://github.com/modelcontextprotocol/mcpb) to install:

```bash
npm install -g @anthropic-ai/mcpb
mcpb install mcp-ping-*-darwin-aarch64.mcpb
```

**Manual install**: Extract the `.mcpb` archive and place the binary in your desired location, then configure your MCP client (see [Client Configuration](#client-configuration) below).

### From Source

```bash
git clone https://github.com/grahambrooks/mcp-dep.git
cd mcp-dep
cargo build --release
```

The binary will be at `target/release/mcp-ping` (or `mcp-ping.exe` on Windows).

## Client Configuration

### Claude Desktop

Add to your Claude Desktop configuration:
- **macOS/Linux**: `~/.config/claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "ping": {
      "command": "/path/to/mcp-ping"
    }
  }
}
```

### Claude Code (CLI)

Add the server using the CLI:

```bash
claude mcp add ping /path/to/mcp-ping --scope user
```

Or edit `~/.claude.json` directly:

```json
{
  "mcpServers": {
    "ping": {
      "type": "stdio",
      "command": "/path/to/mcp-ping"
    }
  }
}
```

Verify with `/mcp` command inside Claude Code to check connection status.

### VS Code (GitHub Copilot)

Create `.vscode/mcp.json` in your workspace or configure globally:

```json
{
  "servers": {
    "ping": {
      "type": "stdio",
      "command": "/path/to/mcp-ping"
    }
  }
}
```

Requires VS Code 1.102+ with GitHub Copilot extension.

### Cursor

Add to Cursor's MCP configuration (`~/.cursor/mcp.json`):

```json
{
  "mcpServers": {
    "ping": {
      "command": "/path/to/mcp-ping"
    }
  }
}
```

Enable MCP in Cursor settings if not already enabled.

### JetBrains IDEs

For IntelliJ IDEA 2025.2+, RustRover, and other JetBrains IDEs:

1. Go to **Settings | Tools | AI Assistant | MCP Servers**
2. Click **Add** and configure:
   - **Name**: `ping`
   - **Command**: `/path/to/mcp-ping`
   - **Transport**: `stdio`

Or add to your IDE's MCP configuration file:

```json
{
  "servers": {
    "ping": {
      "command": "/path/to/mcp-ping",
      "args": []
    }
  }
}
```

## Manual Testing

The server communicates over stdio using JSON-RPC. You can test it by running:

```bash
./target/release/mcp-ping
```

Then send JSON-RPC requests via stdin:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "test",
      "version": "1.0.0"
    }
  }
}
```

## Tools

| Tool   | Description                                  |
|--------|----------------------------------------------|
| `ping` | A simple ping tool that responds with "pong" |

## Development

### Prerequisites

- Rust 1.70 or later

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

## Release Process

Releases use [release-kit](https://github.com/grahambrooks/release-kit) with
CalVer tags (`vYYYY.M.D`: today's UTC date, the last number bumped if that tag
exists). The tag is the version; nothing is bumped by hand:

```bash
make release-dry   # show the tag that would be pushed
make release       # tag origin/main with the next CalVer and push only the tag
```

The tag runs `.github/workflows/release.yml` (identical in every release-kit
repo; configured by `.release.env`), which builds `mcp-ping` for macOS
(x86_64, aarch64), Linux (x86_64, aarch64) and Windows (x86_64), publishes the
archives and `SHA256SUMS` on a GitHub Release, and merges a PR that updates the
Homebrew formula (`Formula/mcp-ping.rb`) and the version in `Cargo.toml`.
`.github/workflows/release-extras.yml` then adds to the same release:

- Platform-specific `.mcpb` packages
- `registry.json` for package discovery

Nothing is published to crates.io.

## License

MIT License - see [LICENSE](LICENSE) for details.
