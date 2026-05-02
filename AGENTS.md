<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan at:
.specs/001-ollama-model-migrator/plan.md
<!-- SPECKIT END -->

## Development Commands

This project uses [Just](https://github.com/casey/just) as a task runner.
Common commands:

```bash
just build          # Debug build
just release        # Release build
just test           # Run tests
just check          # Cargo check
just clippy         # Run lints
just fmt            # Format code
just dev            # fmt + check + test
just ci             # Full CI check
```

The main development cycle uses `just dev`.

See `just --list` for all available commands.
