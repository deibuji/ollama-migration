use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::ollama::OllamaModel;

#[derive(Debug, Clone)]
pub enum JobStatus {
    Pending,
    Running,
    Completed { duration: Duration },
    Failed { error: String },
    Cancelled,
}

impl JobStatus {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }
}

#[derive(Debug)]
pub struct MigrationJob {
    pub id: uuid::Uuid,
    pub source: OllamaModel,
    pub destination: PathBuf,
    pub status: std::sync::Mutex<JobStatus>,
    pub progress_bytes: AtomicU64,
    pub total_bytes: u64,
    pub started_at: std::sync::Mutex<Option<Instant>>,
    pub completed_at: std::sync::Mutex<Option<Instant>>,
}

impl MigrationJob {
    pub fn new(source: OllamaModel, destination: PathBuf) -> Arc<Self> {
        let total_bytes = source.model_blob.size;
        Arc::new(Self {
            id: uuid::Uuid::new_v4(),
            source,
            destination,
            status: std::sync::Mutex::new(JobStatus::Pending),
            progress_bytes: AtomicU64::new(0),
            total_bytes,
            started_at: std::sync::Mutex::new(None),
            completed_at: std::sync::Mutex::new(None),
        })
    }

    pub fn start(&self) {
        let mut status = self.status.lock().unwrap();
        *status = JobStatus::Running;
        *self.started_at.lock().unwrap() = Some(Instant::now());
    }

    pub fn complete(&self) {
        let mut status = self.status.lock().unwrap();
        let started = self.started_at.lock().unwrap();
        let duration = started.map(|s| s.elapsed()).unwrap_or_default();
        *status = JobStatus::Completed { duration };
        *self.completed_at.lock().unwrap() = Some(Instant::now());
    }

    pub fn fail(&self, error: String) {
        let mut status = self.status.lock().unwrap();
        *status = JobStatus::Failed { error };
        *self.completed_at.lock().unwrap() = Some(Instant::now());
    }

    pub fn progress(&self) -> f64 {
        let progress = self.progress_bytes.load(Ordering::Relaxed);
        if self.total_bytes == 0 {
            0.0
        } else {
            (progress as f64 / self.total_bytes as f64) * 100.0
        }
    }

    pub fn status(&self) -> JobStatus {
        self.status.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ollama::BlobRef;
    use std::path::PathBuf;

    fn mock_model() -> OllamaModel {
        OllamaModel {
            name: "test:latest".into(),
            manifest_path: PathBuf::from("/test/manifest"),
            total_size: 1000,
            model_blob: BlobRef {
                digest: "sha256:test".into(),
                algorithm: "sha256".into(),
                hash: "test".into(),
                size: 1000,
                path: PathBuf::from("/test/blob"),
            },
            config_blob: None,
        }
    }

    #[test]
    fn test_job_lifecycle() {
        let job = MigrationJob::new(mock_model(), PathBuf::from("/out.gguf"));

        assert!(matches!(job.status(), JobStatus::Pending));

        job.start();
        assert!(matches!(job.status(), JobStatus::Running));

        job.progress_bytes.store(500, Ordering::Relaxed);
        assert_eq!(job.progress(), 50.0);

        job.complete();
        assert!(matches!(job.status(), JobStatus::Completed { .. }));
    }

    #[test]
    fn test_job_fail() {
        let job = MigrationJob::new(mock_model(), PathBuf::from("/out.gguf"));
        job.start();
        job.fail("Disk full".into());

        assert!(job.status().is_failed());
    }
}
