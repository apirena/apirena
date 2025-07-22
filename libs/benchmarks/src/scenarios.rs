use crate::metrics::ProjectSize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Different benchmark scenarios that match real-world usage
#[derive(Debug, Clone)]
pub enum BenchmarkScenario {
    /// Cold start - first time opening a project
    ColdStart { project_path: PathBuf, size: ProjectSize },
    /// Warm start - project already opened, using cached data
    WarmStart { project_path: PathBuf, size: ProjectSize },
    /// Single file change - typical development workflow
    FileChange { project_path: PathBuf, file_path: PathBuf },
    /// Branch switch - many files changed at once
    BranchSwitch { project_path: PathBuf, changed_files: Vec<PathBuf> },
    /// Large file - performance with huge files
    LargeFile { file_path: PathBuf, lines: usize },
    /// Many small files - lots of tiny route files
    ManySmallFiles { project_path: PathBuf, file_count: usize },
    /// Deep nesting - deeply nested route structures
    DeepNesting { project_path: PathBuf, nesting_depth: usize },
    /// Rapid changes - stress test for file watching
    RapidChanges { project_path: PathBuf, changes_per_second: usize },
    /// Mixed languages - monorepo with multiple frameworks
    MixedLanguages { project_path: PathBuf, languages: Vec<String> },
    /// Memory stress - test memory growth patterns
    MemoryStress { project_path: PathBuf, target_memory_mb: f64 },
}

impl BenchmarkScenario {
    /// Get the expected performance characteristics for this scenario
    pub fn expected_performance(&self) -> ScenarioExpectations {
        match self {
            BenchmarkScenario::ColdStart { size, .. } => ScenarioExpectations {
                max_duration_ms: match size {
                    ProjectSize::Small => 50,
                    ProjectSize::Medium => 500,
                    ProjectSize::Large => 10_000,
                },
                max_memory_mb: 50.0,
                min_accuracy: 0.95,
                description: "Cold start performance".to_string(),
            },
            BenchmarkScenario::WarmStart { size, .. } => ScenarioExpectations {
                max_duration_ms: match size {
                    ProjectSize::Small => 10,
                    ProjectSize::Medium => 50,
                    ProjectSize::Large => 200,
                },
                max_memory_mb: 30.0,
                min_accuracy: 0.98,
                description: "Warm start with caching".to_string(),
            },
            BenchmarkScenario::FileChange { .. } => ScenarioExpectations {
                max_duration_ms: 50,
                max_memory_mb: 10.0,
                min_accuracy: 0.99,
                description: "Single file change latency".to_string(),
            },
            BenchmarkScenario::BranchSwitch { changed_files, .. } => ScenarioExpectations {
                max_duration_ms: (changed_files.len() as u64 * 10).max(100),
                max_memory_mb: 100.0,
                min_accuracy: 0.95,
                description: "Branch switch with multiple changes".to_string(),
            },
            BenchmarkScenario::LargeFile { lines, .. } => ScenarioExpectations {
                max_duration_ms: (*lines as u64 / 100).max(10),
                max_memory_mb: (*lines as f64 / 1000.0).max(5.0),
                min_accuracy: 0.90,
                description: "Large file parsing".to_string(),
            },
            BenchmarkScenario::ManySmallFiles { file_count, .. } => ScenarioExpectations {
                max_duration_ms: (*file_count as u64 * 2).max(50),
                max_memory_mb: (*file_count as f64 * 0.1).max(20.0),
                min_accuracy: 0.98,
                description: "Many small files".to_string(),
            },
            BenchmarkScenario::DeepNesting { nesting_depth, .. } => ScenarioExpectations {
                max_duration_ms: (*nesting_depth as u64 * 5).max(20),
                max_memory_mb: (*nesting_depth as f64 * 2.0).max(10.0),
                min_accuracy: 0.85,
                description: "Deep nesting complexity".to_string(),
            },
            BenchmarkScenario::RapidChanges { changes_per_second, .. } => ScenarioExpectations {
                max_duration_ms: (1000 / *changes_per_second as u64).max(10),
                max_memory_mb: 200.0,
                min_accuracy: 0.80,
                description: "Rapid file changes".to_string(),
            },
            BenchmarkScenario::MixedLanguages { languages, .. } => ScenarioExpectations {
                max_duration_ms: (languages.len() as u64 * 100).max(200),
                max_memory_mb: (languages.len() as f64 * 50.0).max(100.0),
                min_accuracy: 0.90,
                description: "Mixed language support".to_string(),
            },
            BenchmarkScenario::MemoryStress { target_memory_mb, .. } => ScenarioExpectations {
                max_duration_ms: 30_000, // 30 seconds for stress tests
                max_memory_mb: *target_memory_mb,
                min_accuracy: 0.85,
                description: "Memory stress testing".to_string(),
            },
        }
    }
    
    /// Get a human-readable name for the scenario
    pub fn name(&self) -> String {
        match self {
            BenchmarkScenario::ColdStart { size, .. } => {
                format!("cold_start_{:?}", size).to_lowercase()
            }
            BenchmarkScenario::WarmStart { size, .. } => {
                format!("warm_start_{:?}", size).to_lowercase()
            }
            BenchmarkScenario::FileChange { .. } => "file_change".to_string(),
            BenchmarkScenario::BranchSwitch { changed_files, .. } => {
                format!("branch_switch_{}_files", changed_files.len())
            }
            BenchmarkScenario::LargeFile { lines, .. } => {
                format!("large_file_{}_lines", lines)
            }
            BenchmarkScenario::ManySmallFiles { file_count, .. } => {
                format!("many_small_files_{}", file_count)
            }
            BenchmarkScenario::DeepNesting { nesting_depth, .. } => {
                format!("deep_nesting_{}_levels", nesting_depth)
            }
            BenchmarkScenario::RapidChanges { changes_per_second, .. } => {
                format!("rapid_changes_{}_per_sec", changes_per_second)
            }
            BenchmarkScenario::MixedLanguages { languages, .. } => {
                format!("mixed_languages_{}", languages.join("_"))
            }
            BenchmarkScenario::MemoryStress { target_memory_mb, .. } => {
                format!("memory_stress_{}mb", *target_memory_mb as u32)
            }
        }
    }
}

/// Expected performance characteristics for a benchmark scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioExpectations {
    /// Maximum duration in milliseconds
    pub max_duration_ms: u64,
    /// Maximum memory usage in MB
    pub max_memory_mb: f64,
    /// Minimum accuracy (0.0 to 1.0)
    pub min_accuracy: f64,
    /// Human-readable description
    pub description: String,
}

/// Factory for creating common benchmark scenarios
pub struct ScenarioFactory;

impl ScenarioFactory {
    /// Create a standard set of development workflow scenarios
    pub fn development_workflows(project_path: PathBuf) -> Vec<BenchmarkScenario> {
        vec![
            BenchmarkScenario::ColdStart {
                project_path: project_path.clone(),
                size: ProjectSize::Small,
            },
            BenchmarkScenario::WarmStart {
                project_path: project_path.clone(),
                size: ProjectSize::Small,
            },
            BenchmarkScenario::FileChange {
                project_path: project_path.clone(),
                file_path: project_path.join("src/routes/users.js"),
            },
            BenchmarkScenario::BranchSwitch {
                project_path: project_path.clone(),
                changed_files: vec![
                    project_path.join("src/routes/users.js"),
                    project_path.join("src/routes/auth.js"),
                    project_path.join("src/middleware/cors.js"),
                ],
            },
        ]
    }
    
    /// Create scalability test scenarios
    pub fn scalability_tests(project_path: PathBuf) -> Vec<BenchmarkScenario> {
        vec![
            BenchmarkScenario::ColdStart {
                project_path: project_path.clone(),
                size: ProjectSize::Medium,
            },
            BenchmarkScenario::ColdStart {
                project_path: project_path.clone(),
                size: ProjectSize::Large,
            },
            BenchmarkScenario::ManySmallFiles {
                project_path: project_path.clone(),
                file_count: 1000,
            },
            BenchmarkScenario::MemoryStress {
                project_path: project_path.clone(),
                target_memory_mb: 500.0,
            },
        ]
    }
    
    /// Create edge case scenarios
    pub fn edge_cases(project_path: PathBuf) -> Vec<BenchmarkScenario> {
        vec![
            BenchmarkScenario::LargeFile {
                file_path: project_path.join("src/routes/generated.js"),
                lines: 10000,
            },
            BenchmarkScenario::DeepNesting {
                project_path: project_path.clone(),
                nesting_depth: 20,
            },
            BenchmarkScenario::RapidChanges {
                project_path: project_path.clone(),
                changes_per_second: 10,
            },
            BenchmarkScenario::MixedLanguages {
                project_path: project_path.clone(),
                languages: vec!["javascript".to_string(), "python".to_string(), "php".to_string()],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_scenario_expectations() {
        let scenario = BenchmarkScenario::ColdStart {
            project_path: PathBuf::from("/test"),
            size: ProjectSize::Small,
        };
        
        let expectations = scenario.expected_performance();
        assert_eq!(expectations.max_duration_ms, 50);
        assert_eq!(expectations.min_accuracy, 0.95);
    }
    
    #[test]
    fn test_scenario_names() {
        let scenario = BenchmarkScenario::FileChange {
            project_path: PathBuf::from("/test"),
            file_path: PathBuf::from("/test/file.js"),
        };
        
        assert_eq!(scenario.name(), "file_change");
    }
    
    #[test]
    fn test_scenario_factory() {
        let project_path = PathBuf::from("/test/project");
        let scenarios = ScenarioFactory::development_workflows(project_path);
        
        assert_eq!(scenarios.len(), 4);
        assert!(matches!(scenarios[0], BenchmarkScenario::ColdStart { .. }));
        assert!(matches!(scenarios[1], BenchmarkScenario::WarmStart { .. }));
    }
}
