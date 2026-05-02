# Architecture

## Module Structure

```
ollama-migrator/
├── src/
│   ├── main.rs          # CLI entry
│   ├── cli.rs           # Clap argument definitions
│   ├── lib.rs           # Library exports
│   ├── error.rs         # Error types
│   ├── platform.rs      # Platform detection
│   ├── paths.rs         # Path utilities
│   ├── ollama/          # Ollama integration
│   ├── gguf/            # GGUF parsing
│   ├── export/          # Export engine
│   └── output/          # Formatters
```

## Key Design Decisions

- Zero-copy extraction: GGUF data streamed from blob to output
- Memory-mapped I/O via memmap2 for large files
- Streaming prevents loading 100GB+ models into RAM
