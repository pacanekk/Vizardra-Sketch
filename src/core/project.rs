use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::core::document::Document;

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
        std::fs::write(path, json)
            .map_err(|e| ProjectError::IoError(e.to_string()))?;
        Ok(())
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self, ProjectError> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| ProjectError::IoError(e.to_string()))?;
        let project: ProjectFile = serde_json::from_str(&json)
            .map_err(|e| ProjectError::DeserializationError(e.to_string()))?;
        Ok(project)
    }
}

#[derive(Debug)]
pub enum ProjectError {
    SerializationError(String),
    DeserializationError(String),
    IoError(String),
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ProjectError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            ProjectError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ProjectError {}

#[allow(dead_code)]
pub const FILE_EXTENSION: &str = "vizardra";
