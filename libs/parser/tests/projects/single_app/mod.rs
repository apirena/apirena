// Single application project tests
// Tests for projects that contain a single application/service

// TODO: Implement specific single app test scenarios:
// pub mod express_basic;
// pub mod flask_basic;
// pub mod nextjs_basic;
// pub mod laravel_basic;

use std::fs;
use std::path::{Path, PathBuf};

/// Helper function to create test file structures for single app scenarios
pub fn create_test_structure<P: AsRef<Path>>(base_path: P, files: Vec<(&str, &str)>) {
    for (path, content) in files {
        let full_path = base_path.as_ref().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full_path, content).unwrap();
    }
}
