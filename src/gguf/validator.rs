use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::Result;

use super::header::GGUFHeader;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.valid && self.errors.is_empty()
    }
}

pub fn validate_gguf(path: &Path) -> Result<ValidationResult> {
    let mut errors = Vec::new();

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            errors.push(format!("Cannot open file: {}", e));
            return Ok(ValidationResult {
                valid: false,
                errors,
            });
        }
    };

    let mut header_buf = [0u8; 24];
    if let Err(e) = file.read_exact(&mut header_buf) {
        errors.push(format!("Cannot read header: {}", e));
        return Ok(ValidationResult {
            valid: false,
            errors,
        });
    }

    match GGUFHeader::from_bytes(&header_buf) {
        Ok(header) => {
            if header.tensor_count == 0 {
                errors.push("Model has 0 tensors".into());
            }
        }
        Err(e) => {
            errors.push(format!("Header validation failed: {}", e));
        }
    }

    Ok(ValidationResult {
        valid: errors.is_empty(),
        errors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::header::GGUF_MAGIC;
    use std::io::Write;

    #[test]
    fn test_validate_valid_gguf() {
        let temp = std::env::temp_dir().join("test_valid.gguf");
        let mut file = File::create(&temp).unwrap();

        let mut data = Vec::new();
        data.extend_from_slice(&GGUF_MAGIC.to_le_bytes());
        data.extend_from_slice(&2u32.to_le_bytes());
        data.extend_from_slice(&100u64.to_le_bytes());
        data.extend_from_slice(&200u64.to_le_bytes());

        file.write_all(&data).unwrap();
        drop(file);

        let result = validate_gguf(&temp).unwrap();
        assert!(result.is_valid());

        let _ = std::fs::remove_file(&temp);
    }

    #[test]
    fn test_validate_invalid_gguf() {
        let temp = std::env::temp_dir().join("test_invalid.gguf");
        let mut file = File::create(&temp).unwrap();
        file.write_all(b"NOT A GGUF FILE").unwrap();
        drop(file);

        let result = validate_gguf(&temp).unwrap();
        assert!(!result.is_valid());

        let _ = std::fs::remove_file(&temp);
    }
}
