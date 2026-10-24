# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Upgrade `rmcp` from 0.17 to 3.7 (`ServerConfig` builder, `ContentBlock`, explicit tool router)

## [0.1.0] - 2025-01-05

### Added

- Initial release
- MCP server with stdio transport
- `ping` tool that responds with "pong"
- Multi-platform release builds (macOS, Linux, Windows)
- mcpb packaging support
- registry.json generation for package discovery
