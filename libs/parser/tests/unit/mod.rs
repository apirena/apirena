// Unit tests - framework-specific testing with dedicated fixtures
pub mod express;
pub mod nextjs;
pub mod flask;
pub mod fastapi;
pub mod laravel;
pub mod core;

use std::fs;
use std::path::Path;

/// Helper function to create test file structures (shared across unit tests)
pub fn create_test_structure<P: AsRef<Path>>(base_path: P, files: Vec<(&str, &str)>) {
    for (path, content) in files {
        let full_path = base_path.as_ref().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full_path, content).unwrap();
    }
}
