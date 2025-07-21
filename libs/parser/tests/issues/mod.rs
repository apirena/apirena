// Issue reproduction tests
// This module contains tests that reproduce specific reported issues
// Tests are organized by issue number for easy tracking and reference

// TODO: Add specific issue reproduction tests as they are reported
// Example structure:
// pub mod issue_001; // Description of the issue
// pub mod issue_002; // Another issue description

use std::fs;
use std::path::Path;

/// Helper function to create test file structures for issue reproduction
pub fn create_test_structure<P: AsRef<Path>>(base_path: P, files: Vec<(&str, &str)>) {
    for (path, content) in files {
        let full_path = base_path.as_ref().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full_path, content).unwrap();
    }
}
