# Feature Specification: Ollama Model Migration Tool

**Feature Branch**: `001-ollama-model-migrator`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: User description: "Create a Rust application which migrates Ollama models to models which can be used by other LLM hosted instances, such as vllm or llama.cpp"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - List Ollama Models (Priority: P1)

As a user, I want to see all models stored in my local Ollama installation so I can identify which ones to migrate.

**Why this priority**: Discovery is the first step in any migration workflow. Without knowing what models are available, users cannot proceed with migration.

**Independent Test**: Can be fully tested by running `ollama-migrator list` and delivers a table of model names, sizes, and modification dates.

**Acceptance Scenarios**:

1. **Given** Ollama is installed with models in the default location, **When** the user runs `ollama-migrator list`, **Then** a formatted table displays all model names, sizes, and quantized formats
2. **Given** Ollama is not installed or the models directory is missing, **When** the user runs `ollama-migrator list`, **Then** a clear error message indicates the Ollama installation was not found
3. **Given** the user specifies a custom Ollama models path via `--ollama-dir`, **When** the user runs `ollama-migrator list --ollama-dir /custom/path`, **Then** the tool reads from the specified directory

---

### User Story 2 - Export Single Model (Priority: P1)

As a user, I want to export a specific Ollama model to a standard GGUF file so I can use it with vLLM or llama.cpp.

**Why this priority**: This is the core functionality. Users need to extract models from Ollama's internal storage format to standard GGUF.

**Independent Test**: Can be fully tested by running `ollama-migrator export <model>` and produces a valid GGUF file loadable by llama.cpp or vLLM.

**Acceptance Scenarios**:

1. **Given** a model named "llama3.2" exists in Ollama, **When** the user runs `ollama-migrator export llama3.2`, **Then** a GGUF file is created in the current directory
2. **Given** the user specifies an output path, **When** the user runs `ollama-migrator export llama3.2 --output ~/models/llama3.2.gguf`, **Then** the file is created at the specified path
3. **Given** the specified model does not exist, **When** the user runs `ollama-migrator export nonexistent`, **Then** an error message lists available models
4. **Given** the export operation is running, **When** the system reports progress via a progress bar showing bytes processed and estimated time remaining

---

### User Story 3 - Batch Export Multiple Models (Priority: P2)

As a user, I want to export multiple models at once so I can migrate my entire Ollama library efficiently.

**Why this priority**: Users often have multiple models. Batch operations save time and reduce repetitive commands.

**Independent Test**: Can be fully tested by running `ollama-migrator export-all` and produces GGUF files for all models.

**Acceptance Scenarios**:

1. **Given** multiple models exist in Ollama, **When** the user runs `ollama-migrator export-all --output-dir ~/migrated-models`, **Then** all models are exported as individual GGUF files
2. **Given** specific models are specified, **When** the user runs `ollama-migrator export llama3.2 mistral-nemo`, **Then** only those models are exported
3. **Given** a model fails to export, **When** batch export runs, **Then** the error is logged but other models continue processing
4. **Given** batch export completes, **Then** a summary report shows successful exports, failed exports, and total bytes migrated

---

### User Story 4 - Verify Exported Model (Priority: P2)

As a user, I want to verify the exported GGUF file is valid so I can be confident it will work with other tools.

**Why this priority**: Silent corruption during export would waste user time when the model fails to load in the target tool.

**Independent Test**: Can be fully tested by running `ollama-migrator verify <file.gguf>` and validates the GGUF structure.

**Acceptance Scenarios**:

1. **Given** a valid GGUF file, **When** the user runs `ollama-migrator verify model.gguf`, **Then** the tool reports the model is valid with metadata details
2. **Given** a corrupted GGUF file, **When** the user runs `ollama-migrator verify corrupted.gguf`, **Then** the tool reports validation errors with specific issue details
3. **Given** the user exports with `--verify` flag, **When** export completes, **Then** the output is automatically verified before reporting success

---

### User Story 5 - Show Model Metadata (Priority: P3)

As a user, I want to see detailed metadata about a model before and after export so I can confirm the correct quantization and architecture.

**Why this priority**: Users need to confirm they're exporting the right variant (e.g., Q4_K_M vs Q8_0) for their use case.

**Independent Test**: Can be fully tested by running `ollama-migrator info <model>` and displays metadata without exporting.

**Acceptance Scenarios**:

1. **Given** a model exists, **When** the user runs `ollama-migrator info llama3.2`, **Then** the tool displays architecture, quantization format, parameters count, and other metadata
2. **Given** the user runs `ollama-migrator info` on an exported GGUF file, **When** the path points to a GGUF, **Then** the tool displays the same metadata from the file

---

### Edge Cases

- What happens when the Ollama models directory uses symlinks?
- How does the system handle models with custom quantization not in standard GGUF format?
- What happens when disk space is insufficient for the export?
- How does the system handle models downloaded from Ollama Hub vs custom imported models?
- What happens if the model manifest references multiple blobs (multi-file models)?
- How does the system handle concurrent exports of the same model?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST detect the default Ollama installation location on Linux (~/.ollama/models), macOS (~/.ollama/models), and Windows
- **FR-002**: System MUST allow custom Ollama models directory path via CLI argument
- **FR-003**: System MUST parse Ollama's manifest.json format to locate model blobs
- **FR-004**: System MUST extract the GGUF data from Ollama's content-addressable storage
- **FR-005**: System MUST preserve the original filename or generate one in format `{model-name}-{quantization}.gguf`
- **FR-006**: System MUST display progress information during export operations
- **FR-007**: System MUST validate exported GGUF files using GGUF header parsing
- **FR-008**: System MUST support single model export, multiple model export, and batch export-all modes
- **FR-009**: System MUST handle errors gracefully with descriptive messages
- **FR-010**: System MUST support JSON output format for programmatic consumption (e.g., `--output-format json`)

### Key Entities

- **OllamaModel**: Represents a model installed in Ollama. Attributes: name (e.g., "llama3.2:latest"), manifest path, blob digests (config, layers), size, modification time.
- **GGUFModel**: Represents the extracted/converted model. Attributes: file path, header info (version, tensor count, metadata KV pairs), architecture, quantization type.
- **MigrationJob**: Represents an export operation. Attributes: source model, destination path, status (pending, running, completed, failed), progress bytes, total bytes, error message.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can list Ollama models in under 1 second for up to 100 models
- **SC-002**: Exported GGUF files produce identical SHA256 hashes to known-good GGUFs when available
- **SC-003**: Export throughput achieves at least 500 MB/s on SSD storage (streaming copy, no re-quantization)
- **SC-004**: Exported models load successfully in both llama.cpp (llama-cli) and vLLM
- **SC-005**: CLI has <100ms startup time
- **SC-006**: Memory usage remains under 100MB during export (streaming operation)

## Assumptions

- Users have read access to Ollama's models directory
- Ollama models are stored in GGUF format internally (Ollama's standard for most models)
- Models fit in available disk space (no partial migration)
- Target tools (vLLM, llama.cpp) are installed separately by the user
- No re-quantization is performed; the tool only extracts existing GGUF data
