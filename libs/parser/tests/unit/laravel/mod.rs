// Laravel unit tests module  
pub mod detection;

use std::path::{Path, PathBuf};

/// Laravel-specific test fixtures
pub mod fixtures {
    use super::*;
    
    pub fn basic_app() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/unit/laravel/fixtures/basic_app")
    }
}
