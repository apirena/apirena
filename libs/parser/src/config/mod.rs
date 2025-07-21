pub mod detector;
pub mod generator;
pub mod patterns;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveredConfig {
    pub version: String,
    pub generated_at: String,
    pub debug_mode: bool,
    pub project_structure: ProjectStructure,
    pub frameworks: Vec<FrameworkDetection>,
    pub performance: PerformanceConfig,
    pub overrides: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectStructure {
    pub project_type: String, // 'monorepo' | 'multi-app' | 'single'
    pub file_count: usize,
    pub total_size: u64,
    pub roots: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameworkDetection {
    pub path: String,
    pub framework: String,
    pub confidence: f32,
    pub signals: Vec<DetectionSignal>,
    pub patterns: Vec<RoutePattern>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectionSignal {
    pub signal_type: String,
    pub value: String,
    pub confidence_boost: f32,
    pub source: String, // file path or detection method
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoutePattern {
    pub name: String,
    pub files: String,
    pub routes: Vec<String>,
    pub convention: Option<String>,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceConfig {
    pub threads: String,
    pub cache_strategy: String,
    pub max_file_size: String,
    pub estimated_scan_time: String,
}

/// Main entry point for config discovery
pub struct ConfigDiscovery {
    debug_mode: bool,
}

impl ConfigDiscovery {
    pub fn new(debug_mode: bool) -> Self {
        Self { debug_mode }
    }

    pub async fn discover(&self, project_root: &Path) -> Result<DiscoveredConfig> {
        let detector = detector::FrameworkDetector::new();
        let generator = generator::ConfigGenerator::new(self.debug_mode);
        
        // Detect frameworks and project structure
        let frameworks = detector.detect_all(project_root).await?;
        let project_structure = self.analyze_project_structure(project_root).await?;
        
        // Generate config
        let config = generator.generate(project_root, frameworks, project_structure).await?;
        
        Ok(config)
    }

    async fn analyze_project_structure(&self, root: &Path) -> Result<ProjectStructure> {
        use std::fs;
        
        let mut file_count = 0;
        let mut total_size = 0;
        let mut roots = Vec::new();
        
        // Check for monorepo indicators
        let is_monorepo = self.detect_monorepo_structure(root)?;
        
        // Recursively count files and calculate size
        self.count_files_recursive(root, &mut file_count, &mut total_size)?;
        
        // Find potential app roots
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let dir_name = entry.file_name().to_string_lossy().to_string();
                        if self.is_potential_app_root(&dir_name) {
                            roots.push(format!("./{}", dir_name));
                        }
                    }
                }
            }
        }

        let project_type = if is_monorepo {
            "monorepo"
        } else if roots.len() > 1 {
            "multi-app"
        } else {
            "single"
        }.to_string();

        Ok(ProjectStructure {
            project_type,
            file_count,
            total_size,
            roots,
        })
    }

    fn count_files_recursive(&self, dir: &Path, file_count: &mut usize, total_size: &mut u64) -> Result<()> {
        use std::fs;
        
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        *file_count += 1;
                        *total_size += metadata.len();
                    } else if metadata.is_dir() {
                        // Skip common directories that shouldn't be counted
                        let dir_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        
                        if !matches!(dir_name, "node_modules" | ".git" | "target" | "dist" | "build" | ".next" | "__pycache__") {
                            self.count_files_recursive(&path, file_count, total_size)?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    fn detect_monorepo_structure(&self, root: &Path) -> Result<bool> {
        use std::fs;
        
        // Check for common monorepo indicators
        let indicators = [
            "package.json",     // Check for workspaces
            "pnpm-workspace.yaml",
            "lerna.json",
            "nx.json",
            "rush.json",
            "Cargo.toml",       // Check for workspace members
        ];

        for indicator in indicators {
            let path = root.join(indicator);
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if content.contains("workspace") || content.contains("apps") || content.contains("packages") {
                        return Ok(true);
                    }
                }
            }
        }

        // Check for common monorepo directory structure
        let monorepo_dirs = ["apps", "packages", "services", "libs"];
        let mut found_dirs = 0;
        
        for dir in monorepo_dirs {
            if root.join(dir).is_dir() {
                found_dirs += 1;
            }
        }

        Ok(found_dirs >= 2)
    }

    fn is_potential_app_root(&self, dir_name: &str) -> bool {
        matches!(dir_name, "apps" | "packages" | "services" | "libs" | "src" | "backend" | "frontend" | "api" | "web")
    }
}
