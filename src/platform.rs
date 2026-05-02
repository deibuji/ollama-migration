use std::path::PathBuf;

use crate::error::{MigrationError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
}

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "linux")]
        return Platform::Linux;
        #[cfg(target_os = "macos")]
        return Platform::MacOS;
        #[cfg(target_os = "windows")]
        return Platform::Windows;
    }

    pub fn ollama_default_dir(&self) -> Result<PathBuf> {
        if let Ok(env_dir) = std::env::var("OLLAMA_MODELS") {
            return Ok(PathBuf::from(env_dir));
        }

        match self {
            Platform::Linux | Platform::MacOS => {
                let home = dirs::home_dir().ok_or_else(|| {
                    MigrationError::Platform("Could not determine home directory".into())
                })?;
                Ok(home.join(".ollama").join("models"))
            }
            Platform::Windows => {
                let home = dirs::home_dir().ok_or_else(|| {
                    MigrationError::Platform("Could not determine home directory".into())
                })?;
                Ok(home.join(".ollama").join("models"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_platform() {
        let p = Platform::current();
        #[cfg(target_os = "linux")]
        assert_eq!(p, Platform::Linux);
        #[cfg(target_os = "macos")]
        assert_eq!(p, Platform::MacOS);
        #[cfg(target_os = "windows")]
        assert_eq!(p, Platform::Windows);
    }

    #[test]
    fn test_ollama_default_dir_env_override() {
        unsafe {
            std::env::set_var("OLLAMA_MODELS", "/custom/path");
        }
        let p = Platform::current();
        let dir = p.ollama_default_dir().unwrap();
        assert_eq!(dir, PathBuf::from("/custom/path"));
        unsafe {
            std::env::remove_var("OLLAMA_MODELS");
        }
    }
}
