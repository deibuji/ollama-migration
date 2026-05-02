# CLI Interface Contract

## Command: `ollama-migrator`

A Rust CLI tool for extracting Ollama models to standard GGUF files.

### Global Options

| Option | Short | Argument | Default | Description |
|--------|-------|----------|---------|-------------|
| `--ollama-dir` | `-o` | `PATH` | Auto-detect | Path to Ollama models directory |
| `--format` | `-f` | `FORMAT` | `table` | Output format: `table` or `json` |
| `--verbose` | `-v` | - | false | Enable verbose logging |
| `--help` | `-h` | - | - | Print help message |
| `--version` | `-V` | - | - | Print version |

### Commands

#### `list` - List installed models

```
ollama-migrator list [OPTIONS]
```

Display a table of models installed in Ollama.

**Options:**
| Option | Short | Argument | Description |
|--------|-------|----------|-------------|
| `--quiet` | `-q` | - | Only output names, no headers |

**Output (table format):**
```
NAME                    SIZE         ARCHITECTURE    QUANTIZATION
llama3.2:latest         2.03 GB      llama           Q4_K_M
mistral-nemo:latest     7.11 GB      command-r       Q4_K_M
... (truncated)
```

**Output (JSON format):**
```json
[
  {
    "name": "llama3.2:latest",
    "size": 2176187392,
    "size_human": "2.03 GB",
    "architecture": "llama",
    "quantization": "Q4_K_M"
  }
]
```

**Exit codes:**
- `0` - Success, at least one model found
- `1` - Error (Ollama not found, permission denied)

---

#### `export` - Export one or more models

```
ollama-migrator export [OPTIONS] <MODELS>...
```

Export specified models to GGUF files.

**Arguments:**
- `MODELS` - One or more model names to export (e.g., `llama3.2:latest`)

**Options:**
| Option | Short | Argument | Default | Description |
|--------|-------|----------|---------|-------------|
| `--output` | `-O` | `PATH` | Current dir | Output directory or file pattern |
| `--verify` | - | - | false | Verify GGUF after export |
| `--dry-run` | `-n` | - | false | Show what would be exported without copying |

**Output file naming:**
- If `--output` is a directory: `{name}-{quantization}.gguf` (e.g., `llama3.2-latest-Q4_K_M.gguf`)
- If `--output` is a file: Use exact filename (only for single model)

**Output (table format):**
```
Exporting llama3.2:latest ...
[████████████████████] 2.03 GB / 2.03 GB  (100%)  ETA: 0s
Successfully exported to ./llama3.2-latest-Q4_K_M.gguf
```

**Output (JSON format):**
```json
{
  "results": [
    {
      "model": "llama3.2:latest",
      "success": true,
      "source_size": 2176187392,
      "destination": "./llama3.2-latest-Q4_K_M.gguf",
      "duration_ms": 1452,
      "bytes_per_second": 1498748203
    }
  ]
}
```

**Exit codes:**
- `0` - All exports successful
- `1` - One or more exports failed

---

#### `export-all` - Export all models

```
ollama-migrator export-all [OPTIONS]
```

Export all installed models.

**Options:**
| Option | Short | Argument | Default | Description |
|--------|-------|----------|---------|-------------|
| `--output-dir` | `-d` | `PATH` | Current dir | Output directory for all models |
| `--verify` | - | - | false | Verify each GGUF after export |
| `--parallel` | `-p` | `N` | 1 | Number of parallel exports |

**Exit codes:** Same as `export`

---

#### `info` - Show model information

```
ollama-migrator info [OPTIONS] <MODEL>
```

Display detailed metadata about a model (without exporting).

**Arguments:**
- `MODEL` - Model name (e.g., `llama3.2:latest`) or path to GGUF file

**Output (table format):**
```
Model: llama3.2:latest
Architecture: llama
Parameters: 3.21B
Context length: 128000
Quantization: Q4_K_M
File size: 2.03 GB
Blob digest: sha256:abc123...
```

**Exit codes:**
- `0` - Success
- `1` - Model not found or file invalid

---

#### `verify` - Verify a GGUF file

```
ollama-migrator verify [OPTIONS] <FILE>
```

Validate that a GGUF file is not corrupted and readable.

**Arguments:**
- `FILE` - Path to GGUF file to verify

**Output (table format):**
```
✓ GGUF header valid
✓ Tensor count: 291
✓ Metadata entries: 26
✓ File structure appears valid
```

**Exit codes:**
- `0` - File is valid
- `1` - File is corrupted or invalid

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `OLLAMA_MODELS` | Override Ollama models directory |
| `OLLAMA_MIGRATOR_LOG` | Set log level (error, warn, info, debug, trace) |

## Error Format

All errors are written to stderr in the format:

```
Error: {description}

Suggestion: {helpful suggestion}
```

In JSON mode, errors are structured:
```json
{
  "error": "Model not found",
  "details": "llama3.2:latest does not exist",
  "available_models": ["mistral:latest", "orca-mini:3b"]
}
```
