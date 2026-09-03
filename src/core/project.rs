use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::core::document::Document;

const MAGIC: &[u8] = b"VZD1\n";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectFile {
    pub version: String,
    pub document: Document,
}

impl ProjectFile {
    pub fn from_document(document: &Document) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            document: document.clone(),
        }
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), ProjectError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| ProjectError::SerializationError(e.to_string()))?;
        let mut content = MAGIC.to_vec();
        content.extend_from_slice(json.as_bytes());
        std::fs::write(path, content)
            .map_err(|e| ProjectError::IoError(e.to_string()))?;
        Ok(())
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self, ProjectError> {
        let bytes = std::fs::read(path)
            .map_err(|e| ProjectError::IoError(e.to_string()))?;
        if bytes.len() < MAGIC.len() || &bytes[..MAGIC.len()] != MAGIC {
            return Err(ProjectError::InvalidFormat);
        }
        let json = std::str::from_utf8(&bytes[MAGIC.len()..])
            .map_err(|e| ProjectError::DeserializationError(e.to_string()))?;
        let project: ProjectFile = serde_json::from_str(json)
            .map_err(|e| ProjectError::DeserializationError(e.to_string()))?;
        Ok(project)
    }
}

#[derive(Debug)]
pub enum ProjectError {
    SerializationError(String),
    DeserializationError(String),
    IoError(String),
    InvalidFormat,
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ProjectError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            ProjectError::IoError(msg) => write!(f, "IO error: {}", msg),
            ProjectError::InvalidFormat => write!(f, "Invalid file format: not a Vizardra project"),
        }
    }
}

impl std::error::Error for ProjectError {}

#[allow(dead_code)]
pub const FILE_EXTENSION: &str = "vzd";
