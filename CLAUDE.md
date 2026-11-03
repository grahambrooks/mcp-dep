# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

mcp-ping is a minimal MCP (Model Context Protocol) server written in Rust that demonstrates how to package and distribute MCP servers using MCP Bundles (MCPB). It provides a single `ping` tool that responds with "pong".

**Specification**: https://github.com/modelcontextprotocol/mcpb

## Build Commands

```bash
make check               # The gate: fmt-check + clippy + test, exactly as CI runs them
make hooks               # Install the prek git hooks (make check runs before push)
cargo build              # Development build
cargo build --release    # Release build
cargo test               # Run tests
cargo clippy             # Lint
cargo fmt                # Format code
```

## Git Hooks

`prek.toml` defines them; install with `make hooks`. Commit runs whitespace/YAML/TOML
checks, actionlint, gitleaks and `cargo fmt --check`; push runs `make check`. To fix
what they report: `make fmt`, `cargo clippy --fix`.

## Architecture

**Entry point**: `src/main.rs` (50 lines)

The server uses the `rmcp` crate with procedural macros:
- `#[tool_router]` on impl block enables tool routing
- `#[tool]` attribute defines individual tools with schemas
- `#[tool_handler]` implements the `ServerHandler` trait

**Key components**:
- `PingServer` struct holds a `ToolRouter<PingServer>` for dispatching tool calls
- `ServerHandler::get_info()` returns protocol version, capabilities, and instructions
- `main()` creates the server and connects it to stdio transport

**Communication**: JSON-RPC over stdio, protocol version `2024-11-05`

## MCPB Bundle Structure

- `manifest.json` - Bundle metadata following MCPB spec v0.3
- Binary executable - The compiled Rust server
- Release workflow creates `.mcpb` packages for all platforms

## Release Process

Tag-based releases via GitHub Actions:
```bash
git tag v0.1.0
git push origin v0.1.0
```

Builds for: darwin (x86_64, aarch64), linux (x86_64, aarch64), win32 (x86_64)

## Testing the Server Manually

```bash
# Send initialize request
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0.0"}}}' | ./target/release/mcp-ping
```
