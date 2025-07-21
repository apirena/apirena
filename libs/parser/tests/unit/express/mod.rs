pub mod detection;
pub mod patterns;
pub mod config_generation;

use std::path::{Path, PathBuf};

/// Express-specific test fixtures
pub mod fixtures {
    use super::*;
    
    pub fn basic_app() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/unit/express/fixtures/basic_app")
    }
    
    pub fn monorepo() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/unit/express/fixtures/monorepo")
    }
}
