use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use pinpath_parser::{Endpoint, incremental::EndpointState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointManifest {
    pub version: String,
    pub last_updated: DateTime<Utc>,
    pub endpoints: Vec<EndpointEntry>,
    pub statistics: ManifestStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointEntry {
    pub id: String,
    pub method: String,
    pub path: String,
    pub file: String,
    pub line: usize,
    pub framework: String,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManifestStats {
    pub total: usize,
    pub by_method: BTreeMap<String, usize>,
}

#[derive(Debug, Clone)]
pub struct PinPathStorage {
    project_root: PathBuf,
    pinpath_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct EndpointRecord {
    pub endpoint: Endpoint,
    pub file: PathBuf,
}

impl EndpointRecord {
    pub fn from_endpoint(endpoint: Endpoint, file: &Path) -> Self {
        Self { endpoint, file: file.to_path_buf() }
    }
}

impl PinPathStorage {
    pub fn new(project_root: PathBuf) -> Result<Self> {
        let pinpath_dir = project_root.join(".pinpath");
        // Create directory structure
        fs::create_dir_all(pinpath_dir.join("endpoints/by-file"))?;
        fs::create_dir_all(pinpath_dir.join("cache/ast"))?;

        // Ensure .gitignore exists and ignores cache/
        let gitignore_path = pinpath_dir.join(".gitignore");
        if !gitignore_path.exists() {
            fs::write(&gitignore_path, "cache/\n*.log\nwatch-state.json\n")?;
        }

        Ok(Self { project_root, pinpath_dir })
    }

    pub fn save_endpoints(&self, endpoints: &[EndpointRecord]) -> Result<()> {
        // Build manifest entries
        let now = Utc::now();
        let mut by_file: BTreeMap<String, Vec<EndpointEntry>> = BTreeMap::new();
        let mut by_method: BTreeMap<String, usize> = BTreeMap::new();
        let mut manifest_entries: Vec<EndpointEntry> = Vec::new();

        for rec in endpoints {
            let method = format!("{:?}", rec.endpoint.method);
            *by_method.entry(method.clone()).or_default() += 1;

            let rel_file = pathdiff::diff_paths(&rec.file, &self.project_root)
                .unwrap_or(rec.file.clone())
                .to_string_lossy()
                .to_string();

            let entry = EndpointEntry {
                id: format!("{}:{}", method.to_uppercase(), rec.endpoint.path),
                method,
                path: rec.endpoint.path.clone(),
                file: rel_file.clone(),
                line: rec.endpoint.line,
                framework: "auto".to_string(),
                last_seen: now,
            };

            by_file.entry(rel_file).or_default().push(entry.clone());
            manifest_entries.push(entry);
        }

        let manifest = EndpointManifest {
            version: "1.0.0".into(),
            last_updated: now,
            statistics: ManifestStats { total: manifest_entries.len(), by_method },
            endpoints: manifest_entries,
        };

        // Write manifest.json
        let manifest_path = self.pinpath_dir.join("endpoints/manifest.json");
        write_json_atomic(&manifest_path, &manifest)?;

        // Write by-file jsons
        for (file, entries) in by_file {
            let safe = file.replace('/', "-");
            let p = self.pinpath_dir.join(format!("endpoints/by-file/{}.json", safe));
            write_json_atomic(&p, &entries)?;
        }

        Ok(())
    }

    /// Load parser state from cache
    pub fn load_parser_state(&self) -> Result<EndpointState> {
        let state_path = self.pinpath_dir.join("cache/parser-state.json");
        if !state_path.exists() {
            return Ok(EndpointState {
                endpoints: std::collections::HashMap::new(),
                file_hashes: std::collections::HashMap::new(),
                last_updated: std::time::SystemTime::now(),
            });
        }
        
        let content = std::fs::read_to_string(&state_path)?;
        let state = serde_json::from_str(&content)?;
        Ok(state)
    }

    /// Save parser state to cache
    pub fn save_parser_state(&self, state: &EndpointState) -> Result<()> {
        let state_path = self.pinpath_dir.join("cache/parser-state.json");
        write_json_atomic(&state_path, state)?;
        Ok(())
    }
}

fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let dir = path.parent().ok_or_else(|| anyhow!("Invalid path"))?;
    fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(".tmp-{}.json", uuid::Uuid::new_v4()));
    let data = serde_json::to_string_pretty(value)?;
    fs::write(&tmp, data)?;
    // Atomic rename where supported
    std::fs::rename(&tmp, path)?;
    Ok(())
}
