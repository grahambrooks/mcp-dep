# Contributing

Contributions are welcome! Here's how you can help.

## Reporting Issues

- Check existing issues before creating a new one
- Include reproduction steps and environment details
- For bugs, include the error message and expected behavior

## Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Format code (`make fmt`)
5. Run the same checks CI runs (`make check`: fmt, clippy, tests)
6. Commit with a clear message
7. Push and open a pull request

## Code Style

- Follow Rust conventions
- Run `cargo fmt` before committing
- Address `cargo clippy` warnings
- Add tests for new functionality

## Development Setup

```bash
git clone https://github.com/grahambrooks/mcp-dep.git
cd mcp-dep
cargo build
cargo test
```

## Questions?

Open an issue for questions or discussion.
