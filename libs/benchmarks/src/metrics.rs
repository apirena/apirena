use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Performance targets based on project milestones
#[derive(Debug, Clone)]
pub struct PerformanceTargets {
    /// Single file parse should be < 10ms
    pub single_file_parse_ms: u64,
    /// Full project discovery should be < 50ms for small projects
    pub small_project_discovery_ms: u64,
    /// Medium project discovery should be < 500ms
    pub medium_project_discovery_ms: u64,
    /// Large project discovery should be < 10s
    pub large_project_discovery_ms: u64,
    /// File change to update should be < 50ms
    pub file_change_latency_ms: u64,
    /// Memory usage should be < 100MB for 1000 files
    pub memory_per_1000_files_mb: f64,
}

impl Default for PerformanceTargets {
    fn default() -> Self {
        Self {
            single_file_parse_ms: 10,
            small_project_discovery_ms: 50,
            medium_project_discovery_ms: 500,
            large_project_discovery_ms: 10_000,
            file_change_latency_ms: 50,
            memory_per_1000_files_mb: 100.0,
        }
    }
}

/// Detailed performance metrics for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedMetrics {
    /// Basic timing metrics
    pub timing: TimingMetrics,
    /// Memory usage metrics
    pub memory: MemoryMetrics,
    /// Throughput metrics
    pub throughput: ThroughputMetrics,
    /// Quality metrics
    pub quality: QualityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingMetrics {
    pub total_duration: Duration,
    pub discovery_time: Duration,
    pub parse_time: Duration,
    pub framework_detection_time: Duration,
    pub config_generation_time: Duration,
    pub file_watching_setup_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Peak memory usage in MB
    pub peak_memory_mb: f64,
    /// Memory per file in KB
    pub memory_per_file_kb: f64,
    /// Memory per endpoint in bytes
    pub memory_per_endpoint_bytes: f64,
    /// Memory growth rate
    pub memory_growth_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    /// Files processed per second
    pub files_per_second: f64,
    /// Endpoints extracted per second
    pub endpoints_per_second: f64,
    /// Lines of code processed per second
    pub lines_per_second: f64,
    /// Bytes processed per second
    pub bytes_per_second: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Total endpoints found
    pub endpoints_found: usize,
    /// Frameworks detected
    pub frameworks_detected: Vec<String>,
    /// Detection accuracy (0.0 to 1.0)
    pub accuracy: f64,
    /// False positive rate
    pub false_positive_rate: f64,
    /// Coverage percentage
    pub coverage_percentage: f64,
}

/// Project size categories for benchmarking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectSize {
    Small,   // 10-50 files
    Medium,  // 100-500 files
    Large,   // 1000+ files
}

impl ProjectSize {
    pub fn target_discovery_time(&self, targets: &PerformanceTargets) -> Duration {
        match self {
            ProjectSize::Small => Duration::from_millis(targets.small_project_discovery_ms),
            ProjectSize::Medium => Duration::from_millis(targets.medium_project_discovery_ms),
            ProjectSize::Large => Duration::from_millis(targets.large_project_discovery_ms),
        }
    }
    
    pub fn expected_file_count(&self) -> (usize, usize) {
        match self {
            ProjectSize::Small => (10, 50),
            ProjectSize::Medium => (100, 500),
            ProjectSize::Large => (1000, 5000),
        }
    }
}

/// Benchmark result validator
pub struct MetricsValidator {
    targets: PerformanceTargets,
}

impl MetricsValidator {
    pub fn new(targets: PerformanceTargets) -> Self {
        Self { targets }
    }
    
    pub fn validate_single_file_parse(&self, duration: Duration) -> bool {
        duration.as_millis() as u64 <= self.targets.single_file_parse_ms
    }
    
    pub fn validate_project_discovery(&self, size: ProjectSize, duration: Duration) -> bool {
        duration <= size.target_discovery_time(&self.targets)
    }
    
    pub fn validate_memory_usage(&self, file_count: usize, memory_mb: f64) -> bool {
        let expected_memory = (file_count as f64 / 1000.0) * self.targets.memory_per_1000_files_mb;
        memory_mb <= expected_memory
    }
    
    pub fn validate_file_change_latency(&self, duration: Duration) -> bool {
        duration.as_millis() as u64 <= self.targets.file_change_latency_ms
    }
}

impl Default for MetricsValidator {
    fn default() -> Self {
        Self::new(PerformanceTargets::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_targets() {
        let targets = PerformanceTargets::default();
        assert_eq!(targets.single_file_parse_ms, 10);
        assert_eq!(targets.memory_per_1000_files_mb, 100.0);
    }
    
    #[test]
    fn test_project_size_targets() {
        let targets = PerformanceTargets::default();
        
        assert_eq!(
            ProjectSize::Small.target_discovery_time(&targets),
            Duration::from_millis(50)
        );
        
        assert_eq!(
            ProjectSize::Large.target_discovery_time(&targets),
            Duration::from_millis(10_000)
        );
    }
    
    #[test]
    fn test_metrics_validator() {
        let validator = MetricsValidator::default();
        
        // Should pass
        assert!(validator.validate_single_file_parse(Duration::from_millis(5)));
        
        // Should fail
        assert!(!validator.validate_single_file_parse(Duration::from_millis(15)));
        
        // Memory validation
        assert!(validator.validate_memory_usage(1000, 90.0));
        assert!(!validator.validate_memory_usage(1000, 120.0));
    }
}
