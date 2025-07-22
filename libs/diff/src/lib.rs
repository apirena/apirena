use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// Represents different sources of changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeSource {
    FileSystem { path: PathBuf, modified: SystemTime },
    GitCommit { hash: String, files: Vec<PathBuf> },
    GitDiff { from: String, to: String },
    Manual { description: String },
}

/// A comprehensive change event with optional diff information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEvent {
    pub source: ChangeSource,
    pub diffs: Vec<FileDiff>,
    pub timestamp: SystemTime,
}

/// File-level diff with line-by-line changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub path: PathBuf,
    pub old_content: Option<String>,
    pub new_content: String,
    pub changes: Vec<LineChange>,
}

/// Individual line change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineChange {
    pub line_number: usize,
    pub change_type: LineChangeType,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineChangeType {
    Added,
    Removed,
    Modified { old_content: String },
    Context, // Unchanged lines for context
}

/// A region of code that contains changes, with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeRegion {
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub has_changes: bool,
    pub change_types: Vec<LineChangeType>,
}

pub mod processor;
pub mod git;
pub mod text;

pub use processor::DiffProcessor;
pub use git::GitIntegration;
pub use text::TextDiffer;
