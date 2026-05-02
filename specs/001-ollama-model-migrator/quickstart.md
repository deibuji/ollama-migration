# Quick Start Guide

## Installation

### From Source (requires Rust)

```bash
git clone https://github.com/example/ollama-migrator
cd ollama-migrator
cargo build --release
# Binary at: target/release/ollama-migrator
```

### Pre-built Binaries

Download from [Releases](https://github.com/example/ollama-migrator/releases):

```bash
# Linux/macOS
curl -L https://github.com/example/ollama-migrator/releases/download/v1.0.0/ollama-migrator-x86_64-linux.tar.gz | tar xz
sudo mv ollama-migrator /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/example/ollama-migrator/releases/download/v1.0.0/ollama-migrator-x86_64-windows.zip" -OutFile "ollama-migrator.zip"
Expand-Archive ollama-migrator.zip -DestinationPath $env:ProgramFiles
```

## Basic Usage

### List your models

```bash
ollama-migrator list
```

Output:
```
NAME                    SIZE         ARCHITECTURE    QUANTIZATION
llama3.2:latest         2.03 GB      llama           Q4_K_M
mistral-nemo:latest     7.11 GB      command-r       Q4_K_M
orca-mini:3b            1.95 GB      llama           Q4_0
```

### Export a single model

```bash
# Export to current directory
ollama-migrator export llama3.2:latest

# Export to specific directory
ollama-migrator export llama3.2:latest --output ~/models/

# Export with custom filename
ollama-migrator export llama3.2:latest --output ~/models/llama3.2.gguf
```

### Export multiple models

```bash
ollama-migrator export llama3.2:latest mistral-nemo:latest --output ~/models/
```

### Export all models

```bash
ollama-migrator export-all --output-dir ~/migrated-models/
```

### Verify an exported model

```bash
ollama-migrator verify ~/models/llama3.2.gguf
```

### Get model info

```bash
ollama-migrator info llama3.2:latest
```

Output:
```
Model: llama3.2:latest
Architecture: llama
Parameters: 3.21B
Context length: 128000
Quantization: Q4_K_M
File size: 2.03 GB
```

## Using with vLLM

```bash
# Export your model
ollama-migrator export llama3.2:latest --output ~/models/

# Run with vLLM
python -m vllm.entrypoints.openai.api_server \
    --model ~/models/llama3.2-latest-Q4_K_M.gguf \
    --quantization gguf
```

## Using with llama.cpp

```bash
# Export your model
ollama-migrator export llama3.2:latest --output ~/models/

# Run with llama.cpp
./llama-server -m ~/models/llama3.2-latest-Q4_K_M.gguf
```

## Custom Ollama Directory

If Ollama is installed in a non-standard location:

```bash
ollama-migrator --ollama-dir /custom/ollama/models list
```

Or set the environment variable:

```bash
export OLLAMA_MODELS=/custom/ollama/models
ollama-migrator list
```

## JSON Output for Scripting

All commands support JSON output for programmatic use:

```bash
# Get model list as JSON
ollama-migrator list --format json | jq '.[0].name'

# Export with JSON progress
ollama-migrator export llama3.2:latest --format json

# Parse success status
ollama-migrator export llama3.2:latest --format json | jq -e '.results[0].success'
```

## Troubleshooting

### "Ollama installation not found"

- Verify Ollama is installed and run `ollama list` to confirm
- Specify the path manually: `ollama-migrator --ollama-dir ~/.ollama/models list`

### "Permission denied"

- Ensure you have read access to the Ollama models directory
- On macOS/Linux: `ls -la ~/.ollama/models/`

### Exported model won't load in vLLM/llama.cpp

- Verify the export: `ollama-migrator verify model.gguf`
- Some models may use unsupported architectures
- Check Ollama's format is standard GGUF (most are)
