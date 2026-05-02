# Data Model: Ollama Model Migration

## Entity Definitions

### OllamaModel

Represents a model installed in Ollama.

| Field | Type | Description |
|-------|------|-------------|
| name | String | Model reference (e.g., "llama3.2:latest") |
| manifest_path | PathBuf | Absolute path to manifest JSON |
| tags | Vec<String> | Available tags for this model |
| total_size | u64 | Total size in bytes (sum of all blobs) |
| model_blob | BlobRef | Reference to the GGUF data blob |
| config_blob | Option<BlobRef> | Optional config metadata blob |

### BlobRef

Reference to a content-addressable blob.

| Field | Type | Description |
|-------|------|-------------|
| digest | String | Full digest string (e.g., "sha256:abc123...") |
| algorithm | String | Hash algorithm, typically "sha256" |
| hash | String | Raw hex hash without prefix |
| size | u64 | Size in bytes |
| path | PathBuf | Absolute path to blob file |

### GGUFInfo

Metadata extracted from a GGUF file (Ollama blob or exported file).

| Field | Type | Description |
|-------|------|-------------|
| version | u32 | GGUF version number |
| tensor_count | u64 | Number of tensors in model |
| metadata_kv_count | u64 | Number of metadata key-value pairs |
| architecture | Option<String> | Model architecture from metadata |
| quantization | Option<String> | Quantization type |
| parameters | Option<u64> | Approximate parameter count |
| context_length | Option<u32> | Maximum context length |

### MigrationJob

Represents an in-progress or completed export operation.

| Field | Type | Description |
|-------|------|-------------|
| id | Uuid | Unique job identifier |
| source | OllamaModel | Source model to export |
| destination | PathBuf | Target file path |
| status | JobStatus | Current status |
| progress_bytes | Arc<AtomicU64> | Bytes copied so far (thread-safe) |
| total_bytes | u64 | Total bytes to copy |
| started_at | Option<Instant> | Start time |
| completed_at | Option<Instant> | Completion time |
| error_message | Option<String> | Error details if failed |

### JobStatus

Enumeration of possible job states.

```rust
pub enum JobStatus {
    Pending,
    Running,
    Completed { duration: Duration },
    Failed { error: String },
    Cancelled,
}
```

### ExportSummary

Result of batch export operations.

| Field | Type | Description |
|-------|------|-------------|
| jobs | Vec<MigrationJob> | All jobs in the batch |
| successful | Vec<uuid> | IDs of successful exports |
| failed | Vec<(uuid, String)> | IDs and errors of failed exports |
| total_bytes | u64 | Total bytes processed |
| total_duration | Duration | Total elapsed time |

## Relationships

```
OllamaModel *--1 BlobRef : model_blob
OllamaModel *--0..1 BlobRef : config_blob
OllamaModel --* MigrationJob : source
MigrationJob --* ExportSummary : part_of
```

## Type Implementations

### Digest Parsing

```rust
impl BlobRef {
    /// Parse a digest string like "sha256:abc123..."
    pub fn from_digest(digest: &str, blobs_dir: &Path) -> Result<Self> {
        let parts: Vec<_> = digest.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidDigest(digest.to_string()));
        }
        
        let algorithm = parts[0];
        let hash = parts[1];
        
        let path = blobs_dir.join(format!("{}-{}", algorithm, hash));
        
        Ok(Self {
            digest: digest.to_string(),
            algorithm: algorithm.to_string(),
            hash: hash.to_string(),
            size: path.metadata()?.len(),
            path,
        })
    }
}
```

### Size Display

Implementing human-readable sizes:

```rust
impl OllamaModel {
    pub fn size_human_readable(&self) -> String {
        let size = self.total_size as f64;
        const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
        
        let mut unit_idx = 0;
        let mut display_size = size;
        
        while display_size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            display_size /= 1024.0;
            unit_idx += 1;
        }
        
        format!("{:.2} {}", display_size, UNITS[unit_idx])
    }
}
```

## Validation Rules

1. **OllamaModel**:
   - name must be non-empty
   - model_blob must exist and be readable

2. **BlobRef**:
   - digest must match pattern `{algorithm}:{hex}`
   - path must exist
   - file size must match expected size (if known)

3. **MigrationJob**:
   - destination parent directory must exist or be creatable
   - source must be valid before job starts

4. **GGUFInfo**:
   - version must be 2 or 3 (current GGUF versions)
   - tensor_count must be > 0 for valid models
