pub mod metrics;
pub mod scenarios;
pub mod reports;

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Core benchmark result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Total time for the operation
    pub total_duration: Duration,
    /// Time spent on file discovery
    pub discovery_time: Duration,
    /// Time spent parsing files
    pub parse_time: Duration,
    /// Time spent generating configuration
    pub config_gen_time: Duration,
    
    /// Files processed per second
    pub files_per_second: f64,
    /// Endpoints extracted per second
    pub endpoints_per_second: f64,
    /// Lines of code processed per second
    pub lines_per_second: f64,
    
    /// Peak memory usage in MB
    pub peak_memory_mb: f64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    
    /// Number of endpoints found
    pub endpoints_found: usize,
    /// Frameworks detected
    pub frameworks_detected: Vec<String>,
    /// Accuracy of detection (0.0 to 1.0)
    pub accuracy: f64,
}

/// Benchmark timing helper
pub struct BenchmarkTimer {
    start: Instant,
    discovery_start: Option<Instant>,
    parse_start: Option<Instant>,
    config_start: Option<Instant>,
}

impl BenchmarkTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            discovery_start: None,
            parse_start: None,
            config_start: None,
        }
    }
    
    pub fn start_discovery(&mut self) {
        self.discovery_start = Some(Instant::now());
    }
    
    pub fn start_parse(&mut self) {
        self.parse_start = Some(Instant::now());
    }
    
    pub fn start_config(&mut self) {
        self.config_start = Some(Instant::now());
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    pub fn discovery_elapsed(&self) -> Duration {
        self.discovery_start.map_or(Duration::ZERO, |start| start.elapsed())
    }
    
    pub fn parse_elapsed(&self) -> Duration {
        self.parse_start.map_or(Duration::ZERO, |start| start.elapsed())
    }
    
    pub fn config_elapsed(&self) -> Duration {
        self.config_start.map_or(Duration::ZERO, |start| start.elapsed())
    }
}

impl Default for BenchmarkTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory measurement utilities
pub fn measure_memory_usage<F, R>(f: F) -> (R, f64)
where
    F: FnOnce() -> R,
{
    let start_memory = memory_stats::memory_stats()
        .map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0)
        .unwrap_or(0.0);
    
    let result = f();
    
    let end_memory = memory_stats::memory_stats()
        .map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0)
        .unwrap_or(0.0);
    
    (result, end_memory - start_memory)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benchmark_timer() {
        let mut timer = BenchmarkTimer::new();
        std::thread::sleep(Duration::from_millis(1));
        
        timer.start_discovery();
        std::thread::sleep(Duration::from_millis(1));
        
        assert!(timer.elapsed() >= Duration::from_millis(1));
        assert!(timer.discovery_elapsed() >= Duration::from_millis(1));
    }
    
    #[test]
    fn test_memory_measurement() {
        let (result, _memory_used) = measure_memory_usage(|| {
            42
        });
        
        assert_eq!(result, 42);
    }
}
