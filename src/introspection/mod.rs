//! The module responsible for understanding bend code, and giving useful information back.

use bend::fun::load_book::do_parse_book;
use miette::{Diagnostic, Error, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use thiserror::Error;

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

    pub fn append_file(&mut self, path: &Path) -> Result<()> {
        let file_knowledge = FileKnowledge::parse_file(path)?;
        self.file_knowledge.push(file_knowledge);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileKnowledge {
    uri: PathBuf,
}

#[derive(Error, Diagnostic, Debug)]
#[error("Could not parse bend file.")]
pub struct BendParseError(String);

impl FileKnowledge {
    pub fn parse_file(file_path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(file_path).into_diagnostic()?;
        FileKnowledge::parse_source(raw.as_str(), file_path)
    }

    pub fn parse_source(file_source: &str, file_path: &Path) -> Result<Self> {
        let book_res = do_parse_book(file_source, file_path, bend::fun::Book::builtins());

        // TODO: Use the book to populate file knowledge.
        // With the bend official parser, we also might want to parse this file using tree-sitter.
        // This is necessary, I believe, to know about positions.

        Ok(FileKnowledge {
            uri: PathBuf::from_str(file_source).unwrap(),
        })
    }
}
