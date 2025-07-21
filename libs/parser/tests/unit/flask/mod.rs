pub mod detection;
pub mod patterns;
pub mod config_generation;

use std::path::{Path, PathBuf};

/// Flask-specific test fixtures
pub mod fixtures {
    use super::*;
    
    pub fn basic_app() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/unit/flask/fixtures/basic_app")
    }
}
