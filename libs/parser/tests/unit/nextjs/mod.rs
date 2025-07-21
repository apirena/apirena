pub mod detection;
pub mod patterns;
pub mod config_generation;

use std::path::{Path, PathBuf};

/// Next.js-specific test fixtures
pub mod fixtures {
    use super::*;
    
    pub fn app_router() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/unit/nextjs/fixtures/app_router")
    }
    
    pub fn pages_router() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/unit/nextjs/fixtures/pages_router")
    }
}
