# ollama-migrator

Extract GGUF models from Ollama's internal storage to standard GGUF files.

[![CI](https://github.com/yourname/ollama-migrator/actions/workflows/ci.yml/badge.svg)](https://github.com/yourname/ollama-migrator/actions)

## Usage

```bash
# List installed models
ollama-migrator list

# Export a model
ollama-migrator export llama3.2:latest ./output.gguf

# Export all models
ollama-migrator export-all ./output/

# Show model info
ollama-migrator info llama3.2:latest

# Verify GGUF file
ollama-migrator verify ./model.gguf
```

## Installation

```bash
cargo install ollama-migrator
```

## Platform Support

- Linux (x86_64, ARM64)
- macOS (x86_64, ARM64)
- Windows (x86_64)

## License

MIT or Apache-2.0
