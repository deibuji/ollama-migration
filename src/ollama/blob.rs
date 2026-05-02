use std::path::{Path, PathBuf};

use crate::error::{MigrationError, Result};

#[derive(Debug, Clone)]
pub struct BlobRef {
    pub digest: String,
    pub algorithm: String,
    pub hash: String,
    pub size: u64,
    pub path: PathBuf,
}

impl BlobRef {
    pub fn from_digest(digest: &str, blobs_dir: &Path) -> Result<Self> {
        let parts: Vec<_> = digest.split(':').collect();
        if parts.len() != 2 {
            return Err(MigrationError::InvalidDigest(digest.to_string()));
        }

        let algorithm = parts[0].to_string();
        let hash = parts[1].to_string();

        let path = blobs_dir.join(format!("{}-{}", algorithm, hash));

        let size = if path.exists() {
            path.metadata()?.len()
        } else {
            0
        };

        Ok(Self {
            digest: digest.to_string(),
            algorithm,
            hash,
            size,
            path,
        })
    }

    pub fn exists(&self) -> bool {
        self.path.exists() && self.path.is_file()
    }

    pub fn validate(&self, expected_size: u64) -> Result<()> {
        if !self.exists() {
            return Err(MigrationError::BlobNotFound(self.digest.clone()));
        }
        if self.size != expected_size {
            return Err(MigrationError::InvalidDigest(format!(
                "Size mismatch: expected {}, got {}",
                expected_size, self.size
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_digest_valid() {
        let blob = BlobRef::from_digest("sha256:abc123", Path::new("/blobs")).unwrap();
        assert_eq!(blob.algorithm, "sha256");
        assert_eq!(blob.hash, "abc123");
        assert_eq!(blob.path, Path::new("/blobs/sha256-abc123"));
    }

    #[test]
    fn test_from_digest_invalid() {
        let result = BlobRef::from_digest("invalid-digest", Path::new("/blobs"));
        assert!(result.is_err());
    }

    #[test]
    fn test_from_digest_no_colon() {
        let result = BlobRef::from_digest("sha256-abc123", Path::new("/blobs"));
        assert!(result.is_err());
    }
}
