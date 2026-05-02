# Implementation Tasks: Ollama Model Migration Tool

**Branch**: `001-ollama-model-migrator` | **Date**: 2026-05-01 | **Plan**: [plan.md](./plan.md)

## Phase 2.1: Project Setup

- [X] **T2.1.1** Initialize Rust project with Cargo workspace
  - Create `Cargo.toml` with project metadata, version 0.1.0
  - Configure dependencies: clap, serde, anyhow, indicatif, memmap2, sha2
  - Set up `src/main.rs` with basic structure
  - Create `rustfmt.toml` with project formatting rules
  - Estimated: 30 min

- [X] **T2.1.2** Set up CI/CD configuration
  - GitHub Actions workflow for build, test, clippy, fmt
  - Add `cargo-audit` security check
  - Configure release builds with PGO
  - Estimated: 45 min

- [X] **T2.1.3** Create project documentation scaffold
  - `README.md` with project description and badges
  - `ARCHITECTURE.md` with module overview
  - `LICENSE` (MIT or Apache-2.0)
  - `CHANGELOG.md` template
  - Estimated: 30 min

## Phase 2.2: Core Infrastructure

- [X] **T2.2.1** Implement error types module (`src/error.rs`)
  - Define `MigrationError` enum with variants: IoError, InvalidDigest, ModelNotFound, InvalidGGUF, etc.
  - Implement `From` traits for common errors (std::io::Error, serde_json::Error)
  - Add `thiserror` derive macros
  - Write unit tests for error conversions
  - Estimated: 45 min

- [X] **T2.2.2** Create platform detection module (`src/platform.rs`)
  - Define `Platform` enum: Linux, MacOS, Windows
  - Implement `ollama_default_dir()` function for each platform
  - Handle environment variable `OLLAMA_MODELS` override
  - Write tests with mock environment variables
  - Estimated: 45 min

- [X] **T2.2.3** Implement path utilities module (`src/paths.rs`)
  - Function to sanitize model names for filenames
  - Path joining with validation
  - Extension detection (.gguf)
  - Estimated: 30 min

## Phase 2.3: Ollama Integration

- [X] **T2.3.1** Implement manifest parser (`src/ollama/manifest.rs`)
  - Define `Manifest` struct matching Ollama's JSON schema
  - Implement `serde::Deserialize` for manifest format
  - Parse `layers` array to find model blob (mediaType: application/vnd.ollama.image.model)
  - Extract config blob if present
  - Write tests with sample manifest fixtures
  - Estimated: 60 min

- [X] **T2.3.2** Implement blob reference module (`src/ollama/blob.rs`)
  - Define `BlobRef` struct with digest parsing
  - Implement `from_digest()` method validating sha256:xxx format
  - Compute blob file path from digest
  - Add file existence and size validation
  - Write unit tests for digest parsing
  - Estimated: 45 min

- [X] **T2.3.3** Implement model discovery (`src/ollama/discovery.rs`)
  - Define `OllamaInstallation` struct
  - Implement `discover()` to find and parse all manifests
  - Read all manifest files under `manifests/` directory
  - Build `OllamaModel` instances from manifests
  - Handle errors for corrupted manifests
  - Write integration tests with mock Ollama directory
  - Estimated: 90 min

- [X] **T2.3.4** Create Ollama module exports (`src/ollama/mod.rs`)
  - Re-export public types: `OllamaInstallation`, `OllamaModel`, `Manifest`, `BlobRef`
  - Module-level documentation
  - Estimated: 15 min

## Phase 2.4: GGUF Handling

- [X] **T2.4.1** Implement GGUF header parser (`src/gguf/header.rs`)
  - Define `GGUFHeader` struct with version, tensor_count, metadata_kv_count
  - Read magic number (`GGUF` or `GGUF` + version)
  - Parse little-endian integers per GGUF spec
  - Support GGUF versions 2 and 3
  - Write tests with minimal GGUF fixtures
  - Estimated: 75 min

- [X] **T2.4.2** Implement metadata extractor (`src/gguf/metadata.rs`)
  - Define `GGUFMetadata` struct with key-value pairs
  - Extract architecture, quantization, parameters from metadata
  - Handle string/integer/float/bool metadata types
  - Create `From` trait to convert header to metadata info
  - Write tests for metadata extraction
  - Estimated: 90 min

- [X] **T2.4.3** Implement GGUF validator (`src/gguf/validator.rs`)
  - Validate magic number
  - Check version compatibility
  - Verify tensor offsets are within bounds
  - Return validation result with detailed errors
  - Write tests for valid/invalid GGUF files
  - Estimated: 60 min

- [X] **T2.4.4** Create GGUF module exports (`src/gguf/mod.rs`)
  - Re-export public types: `GGUFHeader`, `GGUFMetadata`, `validate_gguf()`
  - Estimated: 15 min

## Phase 2.5: Export Engine

- [X] **T2.5.1** Implement MigrationJob (`src/export/job.rs`)
  - Define `MigrationJob` struct with all fields
  - Implement `JobStatus` enum with state transitions
  - Create `new()` constructor validating inputs
  - Implement `get_progress()` returning percentage
  - Write unit tests for state transitions
  - Estimated: 60 min

- [X] **T2.5.2** Implement single model export (`src/export/single.rs`)
  - Define `export_model()` function
  - Open source blob file with `memmap2` for efficient reading
  - Create destination file with `BufWriter`
  - Copy data in chunks with progress callback
  - Handle disk full errors and cleanup partial files
  - Verify file size after copy
  - Write integration tests with mock files
  - Estimated: 90 min

- [X] **T2.5.3** Implement batch export (`src/export/batch.rs`)
  - Define `ExportSummary` struct
  - Implement `export_all()` with sequential processing
  - Track multiple jobs, collect results
  - Handle partial failures (continue on error)
  - Generate summary report with timings
  - Write tests for batch operations
  - Estimated: 75 min

- [X] **T2.5.4** Create export module exports (`src/export/mod.rs`)
  - Re-export public types: `MigrationJob`, `ExportSummary`, `export_model()`, `export_all()`
  - Estimated: 15 min

## Phase 2.6: CLI Commands

- [X] **T2.6.1** Implement CLI argument definitions (`src/cli.rs`)
  - Define `Cli` struct with `clap` derive macros
  - Subcommands: list, export, export-all, info, verify
  - Global options: --ollama-dir, --format, --verbose
  - Subcommand-specific options
  - Generate shell completions (optional)
  - Estimated: 60 min

- [X] **T2.6.2** Implement list command handler
  - Call `ollama::discovery::discover()`
  - Format output as table or JSON
  - Handle empty model list
  - Exit code 0 on success, 1 on error
  - Estimated: 45 min

- [X] **T2.6.3** Implement export command handler
  - Parse model names from args
  - Resolve output path from args
  - Call `export::export_model()` for each model
  - Display progress bars using `indicatif`
  - Summary output with success/failure counts
  - Exit codes: 0 all success, 1 any failure
  - Estimated: 60 min

- [X] **T2.6.4** Implement export-all command handler
  - Discover all models
  - Filter by optional pattern
  - Parallel export preparation (if implementing parallelism)
  - Progress reporting for batch
  - Estimated: 30 min

- [X] **T2.6.5** Implement info command handler
  - Accept model name or file path
  - Display formatted metadata
  - JSON output support
  - Estimated: 30 min

- [X] **T2.6.6** Implement verify command handler
  - Accept file path
  - Call `gguf::validate_gguf()`
  - Display validation results
  - Exit code 0 if valid, 1 if invalid
  - Estimated: 30 min

## Phase 2.7: Output Formatters

- [X] **T2.7.1** Implement table formatter (`src/output/table.rs`)
  - Define `TableOutput` trait
  - Format model list as aligned columns
  - Format export progress/results
  - Respect terminal width
  - Handle long names gracefully
  - Estimated: 60 min

- [X] **T2.7.2** Implement JSON formatter (`src/output/json.rs`)
  - Define `JsonOutput` trait
  - Serialize all output types with serde
  - Pretty-print option for human readability
  - Handle JSON escaping
  - Estimated: 45 min

- [X] **T2.7.3** Create output dispatcher (`src/output/mod.rs`)
  - Define `OutputFormat` enum
  - Implement `Output` trait combining table and JSON
  - Dispatch to appropriate formatter based on CLI flag
  - Estimated: 30 min

## Phase 2.8: Main Entry Point

- [X] **T2.8.1** Implement main.rs
  - Parse CLI args
  - Initialize logging with `env_logger` or `tracing`
  - Dispatch to command handlers
  - Set exit codes
  - Top-level error handling with human-friendly messages
  - Estimated: 45 min

- [X] **T2.8.2** Create lib.rs
  - Re-export public API for library use
  - Module organization
  - Crate-level documentation
  - Estimated: 30 min

## Phase 2.9: Testing

- [X] **T2.9.1** Create test fixtures
  - Sample Ollama manifest JSON files
  - Minimal valid GGUF files
  - Corrupted GGUF files for validation testing
  - Mock Ollama directory structure
  - Estimated: 45 min

- [X] **T2.9.2** Write unit tests
  - Test each module independently
  - Mock external dependencies (filesystem)
  >90% code coverage target
  - Estimated: 120 min

- [X] **T2.9.3** Write integration tests
  - End-to-end export flow
  - Command-line interface testing
  - Test with real Ollama installation (optional/manual)
  - Estimated: 60 min

- [X] **T2.9.4** Write benchmarks
  - File copy throughput with `criterion.rs`
  - Compare `std::fs::copy` vs `memmap2` vs `std::io::copy`
  - Memory usage validation
  - Estimated: 60 min

## Phase 2.10: Documentation

- [X] **T2.10.1** Write inline documentation
  - rustdoc comments for all public APIs
  - Module-level documentation with examples
  - Estimated: 60 min

- [X] **T2.10.2** Finalize README.md
  - Installation instructions
  - Usage examples
  - Platform support matrix
  - Performance benchmarks
  - FAQ section
  - Estimated: 45 min

- [X] **T2.10.3** Create man page
  - Full command reference
  - Examples section
  - Environment variables
  - Exit codes
  - Estimated: 30 min

- [X] **T2.10.4** Finalize ARCHITECTURE.md
  - Module dependency graph
  - Design decisions
  - Constitution compliance notes
  - Estimated: 30 min

## Summary

| Phase | Tasks | Est. Time | Priority |
|-------|-------|-----------|----------|
| Project Setup | 3 | 1h 45m | P1 |
| Core Infrastructure | 3 | 2h 00m | P1 |
| Ollama Integration | 4 | 3h 45m | P1 |
| GGUF Handling | 4 | 4h 00m | P1 |
| Export Engine | 4 | 4h 00m | P1 |
| CLI Commands | 6 | 3h 15m | P1 |
| Output Formatters | 3 | 2h 15m | P2 |
| Main Entry Point | 2 | 1h 15m | P1 |
| Testing | 4 | 5h 45m | P1 |
| Documentation | 4 | 2h 45m | P2 |

**Total**: 40 tasks, ~31 hours estimated

**Key Dependencies**:
- T2.2.1 (Error types) → All other tasks
- T2.3.2 (BlobRef) → T2.3.3
- T2.3.3 (Discovery) → T2.6.2, T2.6.4
- T2.4.1 (Header) → T2.4.2, T2.4.3
- T2.5.2 (Single export) → T2.5.3
- T2.7.1, T2.7.2 → T2.6.x (CLI handlers)
