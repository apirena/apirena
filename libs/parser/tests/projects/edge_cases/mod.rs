// Edge case project tests
// Tests for ambiguous patterns, conflicting frameworks, and unusual project structures

// TODO: Implement specific edge case scenarios:
// pub mod ambiguous_detection;
// pub mod conflicting_frameworks;
// pub mod unusual_structures;

use std::fs;
use std::path::{Path, PathBuf};

/// Helper function to create test file structures for edge case scenarios
pub fn create_test_structure<P: AsRef<Path>>(base_path: P, files: Vec<(&str, &str)>) {
    for (path, content) in files {
        let full_path = base_path.as_ref().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full_path, content).unwrap();
    }
}
