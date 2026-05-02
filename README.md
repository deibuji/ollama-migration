# ollama-migrator

Extract GGUF models from Ollama's internal storage to standard GGUF files. Zero-copy streaming extraction handles 100GB+ models with minimal memory.

[![CI](https://github.com/yourname/ollama-migrator/actions/workflows/ci.yml/badge.svg)](https://github.com/yourname/ollama-migrator/actions)

## Why

Ollama stores models in its own content-addressable format. This tool extracts them to standard GGUF files you can use with:
- **vLLM** — for high-throughput serving
- **llama.cpp** — for local inference
- **Hugging Face** — for upload and sharing

Zero-copy extraction means no re-quantization — your model weights remain unchanged.

## Features

- 🚀 **Streaming extraction** — 500+ MB/s, <100MB RAM even for 100GB models
- 📦 **Zero-copy** — extracts original GGUF without modification
- 🔍 **Auto-discovery** — finds Ollama installation automatically
- 📊 **Progress bars** — track large exports in real-time
- ✓ **Built-in verification** — validate GGUF headers after export
- 🖥️ **Cross-platform** — Linux, macOS, Windows

## Installation

```bash
# Via cargo
cargo install ollama-migrator

# Or download prebuilt binary from releases
```

## Usage

```bash
# List installed models
ollama-migrator list

# Export a single model
ollama-migrator export llama3.2:latest ./llama3.2.gguf

# Export with automatic filename
ollama-migrator export llama3.2:latest --output-dir ./models/

# Export multiple models
ollama-migrator export llama3.2 mistral-nemo --output-dir ./models/

# Export all models
ollama-migrator export-all ./backup/

# Show model metadata
ollama-migrator info llama3.2:latest

# Verify a GGUF file
ollama-migrator verify ./model.gguf

# Custom Ollama directory
ollama-migrator --ollama-dir /custom/path list

# JSON output for scripting
ollama-migrator list --format json
```

## Platform Support

| Platform | x86_64 | ARM64 |
|----------|--------|-------|
| Linux    | ✅     | ✅    |
| macOS    | ✅     | ✅    |
| Windows  | ✅     | —     |

## License

MIT or Apache-2.0
