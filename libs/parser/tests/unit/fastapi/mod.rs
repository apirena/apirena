// FastAPI unit tests module
pub mod detection;

use std::path::{Path, PathBuf};

/// FastAPI-specific test fixtures
pub mod fixtures {
    use super::*;
    
    pub fn basic_app() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/unit/fastapi/fixtures/basic_app")
    }
}
