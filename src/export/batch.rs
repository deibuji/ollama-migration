use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::ollama::OllamaModel;
use crate::paths::sanitize_model_name;

use super::job::MigrationJob;
use super::single::{ProgressCallback, export_model};

#[derive(Debug, Clone)]
pub struct ExportSummary {
    pub jobs: Vec<Arc<MigrationJob>>,
    pub successful: Vec<uuid::Uuid>,
    pub failed: Vec<(uuid::Uuid, String)>,
    pub total_bytes: u64,
    pub total_duration: Duration,
}

impl ExportSummary {
    pub fn is_complete(&self) -> bool {
        self.successful.len() + self.failed.len() == self.jobs.len()
    }

    pub fn success_count(&self) -> usize {
        self.successful.len()
    }

    pub fn failure_count(&self) -> usize {
        self.failed.len()
    }

    pub fn overall_progress(&self) -> f64 {
        if self.jobs.is_empty() {
            return 100.0;
        }

        let total_progress: f64 = self.jobs.iter().map(|j| j.progress()).sum();
        total_progress / self.jobs.len() as f64
    }
}

pub fn export_all(
    models: &[OllamaModel],
    output_dir: &Path,
    _progress_callback: Option<ProgressCallback>,
) -> ExportSummary {
    let start = Instant::now();
    let mut jobs = Vec::new();
    let mut successful = Vec::new();
    let mut failed = Vec::new();
    let mut total_bytes = 0u64;

    // Create jobs
    for model in models {
        let filename = format!("{}.gguf", sanitize_model_name(&model.name));
        let destination = output_dir.join(filename);
        let job = MigrationJob::new(model.clone(), destination);
        total_bytes += model.model_blob.size;
        jobs.push(job);
    }

    // Execute jobs sequentially
    for job in &jobs {
        let model = &job.source;
        let result = export_model(model, &job.destination, job.clone());

        match result {
            Ok(()) => {
                successful.push(job.id);
            }
            Err(e) => {
                failed.push((job.id, e.to_string()));
                // Continue to next model (partial failure handling)
            }
        }
    }

    ExportSummary {
        jobs,
        successful,
        failed,
        total_bytes,
        total_duration: start.elapsed(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ollama::BlobRef;
    use std::fs::File;
    use std::io::Write;

    fn create_test_model(temp: &Path, name: &str) -> OllamaModel {
        let blob_path = temp.join(format!("{}.bin", name));
        let mut f = File::create(&blob_path).unwrap();
        f.write_all(b"test data").unwrap();

        OllamaModel {
            name: name.to_string(),
            manifest_path: temp.join("manifest.json"),
            total_size: 9,
            model_blob: BlobRef {
                digest: format!("sha256:{}", name),
                algorithm: "sha256".into(),
                hash: name.into(),
                size: 9,
                path: blob_path,
            },
            config_blob: None,
        }
    }

    #[test]
    fn test_export_all_summary() {
        let temp = std::env::temp_dir().join("batch_test");
        std::fs::create_dir_all(&temp).unwrap();

        let model1 = create_test_model(&temp, "model1");
        let model2 = create_test_model(&temp, "model2");

        let out_dir = temp.join("output");
        std::fs::create_dir_all(&out_dir).unwrap();

        let summary = export_all(&[model1, model2], &out_dir, None);

        assert!(summary.is_complete());
        assert_eq!(summary.jobs.len(), 2);

        let _ = std::fs::remove_dir_all(&temp);
    }
}
