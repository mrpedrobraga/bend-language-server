//! The module responsible for understanding bend code, and giving useful information back.

use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceKnowledge {
    file_knowledge: Vec<FileKnowledge>,
}

impl Default for WorkspaceKnowledge {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceKnowledge {
    pub fn new() -> Self {
        Self {
            file_knowledge: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileKnowledge {}

impl FileKnowledge {
    pub fn parse_file(file_path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(file_path).into_diagnostic()?;
        FileKnowledge::parse_source(raw.as_str())
    }

    pub fn parse_source(file_source: &str) -> Result<Self> {
        Ok(FileKnowledge {})
    }
}
