use thiserror::Error;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid digest: {0}")]
    InvalidDigest(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Invalid GGUF: {0}")]
    InvalidGGUF(String),

    #[error("Blob not found: {0}")]
    BlobNotFound(String),

    #[error("Platform error: {0}")]
    Platform(String),

    #[error("Export failed: {0}")]
    ExportFailed(String),
}

pub type Result<T> = std::result::Result<T, MigrationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: MigrationError = io_err.into();
        assert!(matches!(err, MigrationError::Io(_)));
    }

    #[test]
    fn test_invalid_digest_display() {
        let err = MigrationError::InvalidDigest("bad:digest".into());
        assert_eq!(err.to_string(), "Invalid digest: bad:digest");
    }
}
