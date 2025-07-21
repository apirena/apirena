// Performance testing for large-scale projects
// Tests parser performance with massive monorepos, many files, deep nesting

// TODO: Implement specific performance scenarios:
// pub mod massive_monorepo;
// pub mod deep_nesting;
// pub mod many_files;

use std::fs;
use std::path::Path;

/// Helper function to create test file structures for performance testing
pub fn create_test_structure<P: AsRef<Path>>(base_path: P, files: Vec<(&str, &str)>) {
    for (path, content) in files {
        let full_path = base_path.as_ref().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full_path, content).unwrap();
    }
}

/// Benchmark helpers for performance testing
pub mod benchmarks {
    use std::time::Instant;
    
    pub fn time_operation<F, R>(operation: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();
        (result, duration)
    }
}
