use std::path::PathBuf;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use hallwatch_parser::{LanguageParser, languages::javascript::JavaScriptParser, detect_language};
use hallwatch_benchmarks::measure_memory_usage;
use walkdir::WalkDir;

#[derive(Debug)]
struct StressTestResult {
    project_name: String,
    file_count: usize,
    endpoint_count: usize,
    lines_of_code: usize,
    cold_start_ms: f64,
    single_file_parse_ms: f64,
    batch_parse_ms: f64,
    memory_mb: f64,
    throughput_files_per_sec: f64,
    throughput_endpoints_per_sec: f64,
}

fn main() {
    println!("🚀 HallWatch Stress Test & Scalability Analysis");
    println!("================================================");
    println!("Testing limits and edge cases for development workflows\n");
    
    let projects = discover_test_projects();
    
    if projects.is_empty() {
        println!("❌ No test projects found");
        return;
    }
    
    println!("📊 Stress testing {} projects:\n", projects.len());
    
    let mut all_results = Vec::new();
    
    for (project_name, project_path) in projects {
        println!("🔥 Stress testing: {}", project_name);
        
        let result = run_stress_tests(&project_name, &project_path);
        
        match result {
            Ok(result) => {
                println!("  📁 Files: {} | 🎯 Endpoints: {} | 📝 LoC: {}", 
                    result.file_count, result.endpoint_count, result.lines_of_code);
                println!("  🥶 Cold start: {:.2}ms | 📄 Single file: {:.2}ms | 📦 Batch: {:.2}ms", 
                    result.cold_start_ms, result.single_file_parse_ms, result.batch_parse_ms);
                println!("  🚀 Throughput: {:.1} files/sec | {:.1} endpoints/sec", 
                    result.throughput_files_per_sec, result.throughput_endpoints_per_sec);
                println!("  💾 Memory: {:.2}MB\n", result.memory_mb);
                
                validate_scalability(&result);
                all_results.push(result);
            }
            Err(e) => {
                println!("  ❌ Error: {}", e);
            }
        }
    }
    
    if !all_results.is_empty() {
        print_summary_analysis(&all_results);
    }
}

fn run_stress_tests(project_name: &str, project_path: &PathBuf) -> Result<StressTestResult, Box<dyn std::error::Error>> {
    let js_parser = JavaScriptParser;
    
    // Collect all JavaScript files for analysis
    let js_files = collect_js_files(project_path);
    
    if js_files.is_empty() {
        return Ok(StressTestResult {
            project_name: project_name.to_string(),
            file_count: 0,
            endpoint_count: 0,
            lines_of_code: 0,
            cold_start_ms: 0.0,
            single_file_parse_ms: 0.0,
            batch_parse_ms: 0.0,
            memory_mb: 0.0,
            throughput_files_per_sec: 0.0,
            throughput_endpoints_per_sec: 0.0,
        });
    }
    
    // Test 1: Cold start (parse all files from scratch)
    let cold_start_time = Instant::now();
    let ((endpoint_count, lines_of_code), memory_mb) = measure_memory_usage(|| {
        parse_all_js_files(&js_files, &js_parser)
    });
    let cold_start_ms = cold_start_time.elapsed().as_millis() as f64;
    
    // Test 2: Single file parsing (hot reload simulation)
    let single_file_time = Instant::now();
    if let Some((_, content)) = js_files.first() {
        let _ = js_parser.parse(content);
    }
    let single_file_parse_ms = single_file_time.elapsed().as_millis() as f64;
    
    // Test 3: Batch parsing (multiple files changed)
    let batch_time = Instant::now();
    let batch_files = js_files.iter().take(3).collect::<Vec<_>>();
    for (_, content) in batch_files {
        let _ = js_parser.parse(content);
    }
    let batch_parse_ms = batch_time.elapsed().as_millis() as f64;
    
    // Calculate throughput metrics
    let throughput_files_per_sec = if cold_start_ms > 0.0 {
        (js_files.len() as f64) / (cold_start_ms / 1000.0)
    } else {
        0.0
    };
    
    let throughput_endpoints_per_sec = if cold_start_ms > 0.0 {
        (endpoint_count as f64) / (cold_start_ms / 1000.0)
    } else {
        0.0
    };
    
    Ok(StressTestResult {
        project_name: project_name.to_string(),
        file_count: js_files.len(),
        endpoint_count,
        lines_of_code,
        cold_start_ms,
        single_file_parse_ms,
        batch_parse_ms,
        memory_mb,
        throughput_files_per_sec,
        throughput_endpoints_per_sec,
    })
}

fn collect_js_files(project_path: &PathBuf) -> Vec<(PathBuf, String)> {
    let mut js_files = Vec::new();
    
    for entry in WalkDir::new(project_path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            
            if let Some(language) = detect_language(file_path) {
                if language == "javascript" {
                    if let Ok(content) = fs::read_to_string(file_path) {
                        js_files.push((file_path.to_path_buf(), content));
                    }
                }
            }
        }
    }
    
    js_files
}

fn parse_all_js_files(js_files: &[(PathBuf, String)], parser: &JavaScriptParser) -> (usize, usize) {
    let mut total_endpoints = 0;
    let mut total_lines = 0;
    
    for (_, content) in js_files {
        total_lines += content.lines().count();
        
        if let Ok(endpoints) = parser.parse(content) {
            total_endpoints += endpoints.len();
        }
    }
    
    (total_endpoints, total_lines)
}

fn validate_scalability(result: &StressTestResult) {
    // Scalability thresholds based on project size
    let is_large_project = result.file_count > 10 || result.lines_of_code > 1000;
    
    if is_large_project {
        // Large project expectations
        if result.cold_start_ms < 200.0 {
            println!("  🎯 EXCELLENT: Large project cold start < 200ms");
        } else if result.cold_start_ms < 1000.0 {
            println!("  ✅ GOOD: Large project cold start acceptable");
        } else {
            println!("  ⚠️  SLOW: Large project cold start needs optimization");
        }
        
        if result.throughput_files_per_sec > 20.0 {
            println!("  🚀 HIGH: File processing throughput excellent");
        } else if result.throughput_files_per_sec > 10.0 {
            println!("  ✅ GOOD: File processing throughput acceptable");
        } else {
            println!("  ❌ LOW: File processing throughput needs improvement");
        }
    } else {
        // Small project expectations
        if result.cold_start_ms < 100.0 {
            println!("  🎯 EXCELLENT: Small project cold start excellent");
        } else {
            println!("  ⚠️  Check: Small project slower than expected");
        }
    }
    
    // Hot reload should always be fast regardless of project size
    if result.single_file_parse_ms < 10.0 {
        println!("  ⚡ PERFECT: Single file parsing optimal");
    } else if result.single_file_parse_ms < 50.0 {
        println!("  ✅ GOOD: Single file parsing acceptable");
    } else {
        println!("  ❌ SLOW: Single file parsing affects dev experience");
    }
}

fn print_summary_analysis(results: &[StressTestResult]) {
    println!("📈 COMPREHENSIVE PERFORMANCE SUMMARY");
    println!("=====================================");
    
    let total_files: usize = results.iter().map(|r| r.file_count).sum();
    let total_endpoints: usize = results.iter().map(|r| r.endpoint_count).sum();
    let total_lines: usize = results.iter().map(|r| r.lines_of_code).sum();
    
    let avg_cold_start: f64 = results.iter().map(|r| r.cold_start_ms).sum::<f64>() / results.len() as f64;
    let avg_hot_reload: f64 = results.iter().map(|r| r.single_file_parse_ms).sum::<f64>() / results.len() as f64;
    let avg_throughput: f64 = results.iter().map(|r| r.throughput_files_per_sec).sum::<f64>() / results.len() as f64;
    
    println!("📊 Overall Statistics:");
    println!("  • Total files processed: {}", total_files);
    println!("  • Total endpoints found: {}", total_endpoints);
    println!("  • Total lines of code: {}", total_lines);
    println!("  • Average cold start: {:.2}ms", avg_cold_start);
    println!("  • Average hot reload: {:.2}ms", avg_hot_reload);
    println!("  • Average throughput: {:.1} files/sec", avg_throughput);
    
    println!("\n🎯 Performance Assessment:");
    
    // Development workflow assessment
    if avg_hot_reload < 20.0 {
        println!("  ✅ EXCELLENT: Development workflow is responsive");
    } else {
        println!("  ⚠️  WARNING: Development workflow may feel sluggish");
    }
    
    // Scalability assessment
    if avg_throughput > 15.0 {
        println!("  🚀 EXCELLENT: System scales well with project size");
    } else if avg_throughput > 8.0 {
        println!("  ✅ GOOD: System handles typical projects well");
    } else {
        println!("  ❌ CONCERN: System may struggle with large projects");
    }
    
    // Find best and worst performers
    if let Some(best) = results.iter().max_by(|a, b| a.throughput_files_per_sec.partial_cmp(&b.throughput_files_per_sec).unwrap()) {
        println!("  🏆 Best performer: {} ({:.1} files/sec)", best.project_name, best.throughput_files_per_sec);
    }
    
    if let Some(worst) = results.iter().min_by(|a, b| a.throughput_files_per_sec.partial_cmp(&b.throughput_files_per_sec).unwrap()) {
        if worst.throughput_files_per_sec > 0.0 {
            println!("  📉 Needs attention: {} ({:.1} files/sec)", worst.project_name, worst.throughput_files_per_sec);
        }
    }
}

fn discover_test_projects() -> Vec<(String, PathBuf)> {
    let possible_roots = vec![
        std::env::current_dir().expect("Failed to get current directory"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("../.."),
        PathBuf::from("../../.."),
    ];
    
    for root in possible_roots {
        let projects_dir = root.join("libs/benchmarks/projects");
        
        if projects_dir.exists() {
            let mut projects = Vec::new();

            for category in &["small", "medium", "large"] {
                let category_path = projects_dir.join(category);
                
                if category_path.exists() {
                    if let Ok(entries) = fs::read_dir(&category_path) {
                        for entry in entries.flatten() {
                            if entry.path().is_dir() {
                                let project_name = format!("{}_{}", 
                                    category,
                                    entry.file_name().to_string_lossy()
                                );
                                projects.push((project_name, entry.path()));
                            }
                        }
                    }
                }
            }
            
            if !projects.is_empty() {
                return projects;
            }
        }
    }

    Vec::new()
}
