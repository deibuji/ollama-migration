use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::Arc;

use memmap2::Mmap;

use crate::error::{MigrationError, Result};
use crate::ollama::OllamaModel;
use crate::paths::get_parent_or_create;

use super::job::MigrationJob;

pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send>;

pub fn export_model(model: &OllamaModel, destination: &Path, job: Arc<MigrationJob>) -> Result<()> {
    export_model_with_progress(model, destination, job, None)
}

pub fn export_model_with_progress(
    model: &OllamaModel,
    destination: &Path,
    job: Arc<MigrationJob>,
    progress_callback: Option<ProgressCallback>,
) -> Result<()> {
    // Validate source exists
    if !model.model_blob.path.exists() {
        return Err(MigrationError::BlobNotFound(
            model.model_blob.digest.clone(),
        ));
    }

    // Create destination directory
    get_parent_or_create(destination)?;

    // Remove existing file if it exists (will be replaced)
    if destination.exists() {
        std::fs::remove_file(destination)?;
    }

    job.start();

    // Use memory mapping for efficient large file handling
    let result = copy_with_mmap(
        &model.model_blob.path,
        destination,
        &job,
        progress_callback.as_ref(),
    );

    match result {
        Ok(()) => {
            job.complete();
            Ok(())
        }
        Err(e) => {
            job.fail(e.to_string());
            // Cleanup partial file
            let _ = std::fs::remove_file(destination);
            Err(e)
        }
    }
}

fn copy_with_mmap(
    source: &Path,
    destination: &Path,
    job: &MigrationJob,
    progress_cb: Option<&ProgressCallback>,
) -> Result<()> {
    let source_file = File::open(source)?;
    let metadata = source_file.metadata()?;
    let source_len = metadata.len();

    if source_len == 0 {
        File::create(destination)?;
        return Ok(());
    }

    // Use mmap for large files (> 1MB)
    if source_len > 1024 * 1024 {
        let mmap = unsafe { Mmap::map(&source_file)? };
        let mut dest_file = File::create(destination)?;

        // Copy in chunks to allow progress updates
        const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks

        for chunk in mmap.chunks(CHUNK_SIZE) {
            dest_file.write_all(chunk)?;
            job.progress_bytes
                .fetch_add(chunk.len() as u64, std::sync::atomic::Ordering::Relaxed);

            if let Some(cb) = progress_cb {
                cb(
                    job.progress_bytes
                        .load(std::sync::atomic::Ordering::Relaxed),
                    source_len,
                );
            }
        }

        dest_file.flush()?;
    } else {
        // Small files: use std::fs::copy
        std::fs::copy(source, destination)?;
        job.progress_bytes
            .store(source_len, std::sync::atomic::Ordering::Relaxed);
    }

    // Verify file size matches
    let dest_len = destination.metadata()?.len();
    if dest_len != source_len {
        return Err(MigrationError::ExportFailed(format!(
            "Size mismatch: expected {} bytes, got {}",
            source_len, dest_len
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ollama::BlobRef;
    use std::io::Write;
    use std::path::PathBuf;

    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }

    fn mock_model(path: PathBuf) -> OllamaModel {
        OllamaModel {
            name: "test:latest".into(),
            manifest_path: PathBuf::from("/test"),
            total_size: path.metadata().map(|m| m.len()).unwrap_or(100),
            model_blob: BlobRef {
                digest: "sha256:abc123".into(),
                algorithm: "sha256".into(),
                hash: "abc123".into(),
                size: path.metadata().map(|m| m.len()).unwrap_or(100),
                path,
            },
            config_blob: None,
        }
    }

    #[test]
    fn test_export_small_file() {
        let temp = std::env::temp_dir().join("export_test");
        std::fs::create_dir_all(&temp).unwrap();

        let source = create_test_file(&temp, "source.gguf", b"small test content for export");
        let dest = temp.join("exported.gguf");

        let model = mock_model(source);
        let job = MigrationJob::new(model.clone(), dest.clone());

        let result = export_model(&model, &dest, job.clone());
        assert!(result.is_ok());
        assert!(dest.exists());

        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_export_missing_source() {
        let temp = std::env::temp_dir().join("export_test2");
        std::fs::create_dir_all(&temp).unwrap();

        let model = mock_model(PathBuf::from("/nonexistent/path"));
        let dest = temp.join("out.gguf");
        let job = MigrationJob::new(model.clone(), dest.clone());

        let result = export_model(&model, &dest, job);
        assert!(result.is_err());

        let _ = std::fs::remove_dir_all(&temp);
    }
}
