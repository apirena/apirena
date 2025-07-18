use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub method: HttpMethod,
    pub path: String,
    pub handler: String,
    pub line: usize,
    pub column: usize,
    pub documentation: Option<String>,
}

pub trait LanguageParser: Send + Sync {
    fn parse(&self, content: &str) -> Result<Vec<Endpoint>>;
    fn supports_extension(&self, extension: &str) -> bool;
}

pub fn detect_language(path: &Path) -> Option<&'static str> {
    match path.extension()?.to_str()? {
        "js" | "mjs" => Some("javascript"),
        "ts" | "tsx" => Some("typescript"),
        "py" => Some("python"),
        "go" => Some("go"),
        "rs" => Some("rust"),
        "rb" => Some("ruby"),
        "php" => Some("php"),
        "java" => Some("java"),
        _ => None,
    }
}

pub mod languages {
    pub mod javascript;
    pub mod python;
    pub mod php;
}
