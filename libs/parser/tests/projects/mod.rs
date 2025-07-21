// Project-based integration tests
pub mod startup_monorepo;
pub mod single_app;
pub mod edge_cases; 
pub mod performance;

use std::fs;
use std::path::{Path, PathBuf};

/// Helper function to create test file structures for project scenarios
pub fn create_test_structure<P: AsRef<Path>>(base_path: P, files: Vec<(&str, &str)>) {
    for (path, content) in files {
        let full_path = base_path.as_ref().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full_path, content).unwrap();
    }
}

/// Fixture helpers for project scenarios
pub mod fixtures {
    use super::*;
    
    pub fn express_basic() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/projects/single_app/express_basic")
    }
    
    pub fn express_dual_monorepo() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/projects/startup_monorepo/express_dual")
    }
    
    pub fn mixed_tech_monorepo() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/projects/startup_monorepo/mixed_tech")
    }
    
    pub fn ambiguous_detection() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/projects/edge_cases/ambiguous_detection")
    }
    
    pub fn massive_monorepo() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/projects/performance/massive_monorepo")
    }
}
