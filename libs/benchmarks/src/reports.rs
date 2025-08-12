use crate::metrics::DetailedMetrics;
use crate::scenarios::ScenarioExpectations;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Complete benchmark report with all results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    /// Metadata about the benchmark run
    pub metadata: BenchmarkMetadata,
    /// Results for each scenario
    pub results: HashMap<String, ScenarioResult>,
    /// Summary statistics
    pub summary: BenchmarkSummary,
    /// Performance regression analysis
    pub regression_analysis: Option<Box<RegressionAnalysis>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetadata {
    /// Timestamp of the benchmark run
    pub timestamp: SystemTime,
    /// Git commit hash
    pub commit_hash: Option<String>,
    /// Branch name
    pub branch: Option<String>,
    /// System information
    pub system_info: SystemInfo,
    /// PinPath version
    pub pinpath_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating system
    pub os: String,
    /// CPU information
    pub cpu: String,
    /// Total system memory in GB
    pub memory_gb: f64,
    /// Rust version used for compilation
    pub rust_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    /// The scenario that was benchmarked
    pub scenario_name: String,
    /// Expected performance characteristics
    pub expectations: ScenarioExpectations,
    /// Actual measured metrics
    pub metrics: DetailedMetrics,
    /// Whether the scenario passed all expectations
    pub passed: bool,
    /// Specific failures if any
    pub failures: Vec<String>,
    /// Samples collected (for statistical analysis)
    pub samples: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    /// Total scenarios run
    pub total_scenarios: usize,
    /// Number of scenarios that passed
    pub passed_scenarios: usize,
    /// Number of scenarios that failed
    pub failed_scenarios: usize,
    /// Overall pass rate
    pub pass_rate: f64,
    /// Average performance across all scenarios
    pub average_metrics: AverageMetrics,
    /// Performance score (0-100)
    pub performance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AverageMetrics {
    /// Average total duration across all scenarios
    pub avg_duration_ms: f64,
    /// Average memory usage
    pub avg_memory_mb: f64,
    /// Average throughput
    pub avg_files_per_second: f64,
    /// Average accuracy
    pub avg_accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysis {
    /// Baseline benchmark to compare against
    pub baseline: Option<Box<BenchmarkReport>>,
    /// Performance changes per scenario
    pub changes: HashMap<String, PerformanceChange>,
    /// Overall regression status
    pub has_regression: bool,
    /// Threshold for considering a change significant (e.g., 0.05 for 5%)
    pub regression_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceChange {
    /// Scenario name
    pub scenario: String,
    /// Duration change percentage (-0.1 = 10% faster, 0.1 = 10% slower)
    pub duration_change_percent: f64,
    /// Memory change percentage
    pub memory_change_percent: f64,
    /// Throughput change percentage
    pub throughput_change_percent: f64,
    /// Whether this represents a regression
    pub is_regression: bool,
}

/// Report generator for different output formats
pub struct ReportGenerator;

impl ReportGenerator {
    /// Generate a comprehensive HTML report
    pub fn generate_html_report(report: &BenchmarkReport) -> Result<String, Box<dyn std::error::Error>> {
        let html = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <title>PinPath Benchmark Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ background: #f4f4f4; padding: 20px; border-radius: 8px; }}
        .summary {{ background: #e8f5e8; padding: 15px; margin: 20px 0; border-radius: 8px; }}
        .regression {{ background: #ffe8e8; padding: 15px; margin: 20px 0; border-radius: 8px; }}
        .scenario {{ margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 8px; }}
        .passed {{ border-left: 5px solid #4CAF50; }}
        .failed {{ border-left: 5px solid #f44336; }}
        table {{ width: 100%; border-collapse: collapse; margin: 10px 0; }}
        th, td {{ padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }}
        th {{ background-color: #f2f2f2; }}
        .metric {{ font-family: monospace; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>PinPath Benchmark Report</h1>
        <p><strong>Generated:</strong> {:?}</p>
        <p><strong>Commit:</strong> {}</p>
        <p><strong>System:</strong> {} - {}</p>
    </div>
    
    <div class="summary">
        <h2>Summary</h2>
        <p><strong>Pass Rate:</strong> {:.1}% ({}/{} scenarios passed)</p>
        <p><strong>Performance Score:</strong> {:.1}/100</p>
        <p><strong>Average Duration:</strong> {:.2}ms</p>
        <p><strong>Average Memory:</strong> {:.2}MB</p>
    </div>
    
    {}
    
    <h2>Scenario Results</h2>
    {}
</body>
</html>
"#,
            report.metadata.timestamp,
            report.metadata.commit_hash.as_deref().unwrap_or("unknown"),
            report.metadata.system_info.os,
            report.metadata.system_info.cpu,
            report.summary.pass_rate * 100.0,
            report.summary.passed_scenarios,
            report.summary.total_scenarios,
            report.summary.performance_score,
            report.summary.average_metrics.avg_duration_ms,
            report.summary.average_metrics.avg_memory_mb,
            Self::generate_regression_html(&report.regression_analysis),
            Self::generate_scenarios_html(&report.results)
        );
        
        Ok(html)
    }
    
    /// Generate JSON report for programmatic consumption
    pub fn generate_json_report(report: &BenchmarkReport) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(report)?)
    }
    
    /// Generate concise text summary for CI/CD
    pub fn generate_summary_text(report: &BenchmarkReport) -> String {
        let status = if report.summary.pass_rate >= 0.9 {
            "✅ PASS"
        } else if report.summary.pass_rate >= 0.7 {
            "⚠️  DEGRADED"
        } else {
            "❌ FAIL"
        };
        
        let regression_text = if let Some(analysis) = &report.regression_analysis {
            if analysis.has_regression {
                "\n🚨 Performance regression detected!"
            } else {
                "\n✅ No performance regression"
            }
        } else {
            ""
        };
        
        format!(
            "{} PinPath Benchmarks\n\
            Pass Rate: {:.1}% ({}/{})\n\
            Performance Score: {:.1}/100\n\
            Avg Duration: {:.2}ms\n\
            Avg Memory: {:.2}MB{}",
            status,
            report.summary.pass_rate * 100.0,
            report.summary.passed_scenarios,
            report.summary.total_scenarios,
            report.summary.performance_score,
            report.summary.average_metrics.avg_duration_ms,
            report.summary.average_metrics.avg_memory_mb,
            regression_text
        )
    }
    
    fn generate_regression_html(analysis: &Option<Box<RegressionAnalysis>>) -> String {
        if let Some(analysis) = analysis {
            if analysis.has_regression {
                format!(
                    r#"<div class="regression">
                        <h2>⚠️ Performance Regression Detected</h2>
                        <p>Threshold: {:.1}%</p>
                    </div>"#,
                    analysis.regression_threshold * 100.0
                )
            } else {
                "<div class=\"summary\"><h2>✅ No Performance Regression</h2></div>".to_string()
            }
        } else {
            String::new()
        }
    }
    
    fn generate_scenarios_html(results: &HashMap<String, ScenarioResult>) -> String {
        let mut html = String::new();
        
        for (name, result) in results {
            let status_class = if result.passed { "passed" } else { "failed" };
            let status_icon = if result.passed { "✅" } else { "❌" };
            
            html.push_str(&format!(
                r#"<div class="scenario {}">
                    <h3>{} {}</h3>
                    <table>
                        <tr><th>Metric</th><th>Expected</th><th>Actual</th><th>Status</th></tr>
                        <tr>
                            <td>Duration</td>
                            <td class="metric">≤ {}ms</td>
                            <td class="metric">{:.2}ms</td>
                            <td>{}</td>
                        </tr>
                        <tr>
                            <td>Memory</td>
                            <td class="metric">≤ {:.1}MB</td>
                            <td class="metric">{:.2}MB</td>
                            <td>{}</td>
                        </tr>
                        <tr>
                            <td>Accuracy</td>
                            <td class="metric">≥ {:.1}%</td>
                            <td class="metric">{:.1}%</td>
                            <td>{}</td>
                        </tr>
                    </table>
                </div>"#,
                status_class,
                status_icon,
                name,
                result.expectations.max_duration_ms,
                result.metrics.timing.total_duration.as_millis(),
                if result.metrics.timing.total_duration.as_millis() as u64 <= result.expectations.max_duration_ms { "✅" } else { "❌" },
                result.expectations.max_memory_mb,
                result.metrics.memory.peak_memory_mb,
                if result.metrics.memory.peak_memory_mb <= result.expectations.max_memory_mb { "✅" } else { "❌" },
                result.expectations.min_accuracy * 100.0,
                result.metrics.quality.accuracy * 100.0,
                if result.metrics.quality.accuracy >= result.expectations.min_accuracy { "✅" } else { "❌" }
            ));
        }
        
        html
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::*;
    use std::time::Duration;
    
    #[test]
    fn test_benchmark_summary() {
        let summary = BenchmarkSummary {
            total_scenarios: 10,
            passed_scenarios: 8,
            failed_scenarios: 2,
            pass_rate: 0.8,
            average_metrics: AverageMetrics {
                avg_duration_ms: 25.0,
                avg_memory_mb: 45.0,
                avg_files_per_second: 100.0,
                avg_accuracy: 0.95,
            },
            performance_score: 85.0,
        };
        
        assert_eq!(summary.pass_rate, 0.8);
        assert_eq!(summary.performance_score, 85.0);
    }
    
    #[test]
    fn test_summary_text_generation() {
        let report = create_test_report();
        let summary = ReportGenerator::generate_summary_text(&report);
        
        assert!(summary.contains("PinPath Benchmarks"));
        assert!(summary.contains("Pass Rate"));
    }
    
    fn create_test_report() -> BenchmarkReport {
        BenchmarkReport {
            metadata: BenchmarkMetadata {
                timestamp: SystemTime::now(),
                commit_hash: Some("abc123".to_string()),
                branch: Some("main".to_string()),
                system_info: SystemInfo {
                    os: "Linux".to_string(),
                    cpu: "x86_64".to_string(),
                    memory_gb: 16.0,
                    rust_version: "1.70.0".to_string(),
                },
                pinpath_version: "0.1.0".to_string(),
            },
            results: HashMap::new(),
            summary: BenchmarkSummary {
                total_scenarios: 5,
                passed_scenarios: 4,
                failed_scenarios: 1,
                pass_rate: 0.8,
                average_metrics: AverageMetrics {
                    avg_duration_ms: 30.0,
                    avg_memory_mb: 50.0,
                    avg_files_per_second: 80.0,
                    avg_accuracy: 0.92,
                },
                performance_score: 78.0,
            },
            regression_analysis: None,
        }
    }
}
