use std::path::{Path, PathBuf};

use crate::error::{MigrationError, Result};

pub fn sanitize_model_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

pub fn ensure_extension(path: &Path, ext: &str) -> PathBuf {
    if path.extension().and_then(|e| e.to_str()) == Some(ext) {
        path.to_path_buf()
    } else {
        let mut new_path = path.to_path_buf();
        new_path.set_extension(ext);
        new_path
    }
}

pub fn validate_is_file(path: &Path) -> Result<&Path> {
    if !path.exists() {
        return Err(MigrationError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Path does not exist: {}", path.display()),
        )));
    }
    if !path.is_file() {
        return Err(MigrationError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a file: {}", path.display()),
        )));
    }
    Ok(path)
}

pub fn get_parent_or_create(path: &Path) -> Result<PathBuf> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
        Ok(parent.to_path_buf())
    } else {
        Err(MigrationError::Platform("Path has no parent directory".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_model_name() {
        assert_eq!(sanitize_model_name("llama3.2:latest"), "llama3_2_latest");
        assert_eq!(sanitize_model_name("mixtral-8x22b"), "mixtral-8x22b");
    }

    #[test]
    fn test_ensure_extension() {
        assert_eq!(ensure_extension(Path::new("input"), "gguf"), PathBuf::from("input.gguf"));
        assert_eq!(ensure_extension(Path::new("input.gguf"), "gguf"), PathBuf::from("input.gguf"));
    }
}
