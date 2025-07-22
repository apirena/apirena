use std::path::PathBuf;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use hallwatch_parser::{LanguageParser, languages::javascript::JavaScriptParser, detect_language};
use hallwatch_benchmarks::measure_memory_usage;
use walkdir::WalkDir;

#[derive(Debug)]
struct BenchmarkResult {
    total_files: usize,
    endpoints_found: usize,
    cold_start_ms: f64,
    hot_reload_ms: f64,
    incremental_ms: f64,
    memory_mb: f64,
}

fn main() {
    println!("🚀 HallWatch Performance Analysis: Cold Start vs Hot Reload");
    println!("===========================================================");
    println!("Testing real-world development workflow performance\n");
    
    let projects = discover_test_projects();
    
    if projects.is_empty() {
        println!("❌ No test projects found");
        return;
    }
    
    println!("📂 Analyzing {} projects for development workflow performance:\n", projects.len());
    
    for (project_name, project_path) in projects {
        println!("🔍 Project: {}", project_name);
        
        let result = benchmark_development_workflow(&project_path);
        
        match result {
            Ok(result) => {
                println!("  📊 Files: {} | Endpoints: {}", result.total_files, result.endpoints_found);
                println!("  🥶 Cold start: {:.2}ms", result.cold_start_ms);
                println!("  🔥 Hot reload: {:.2}ms", result.hot_reload_ms);
                println!("  ⚡ Incremental: {:.2}ms", result.incremental_ms);
                println!("  💾 Memory: {:.2}MB", result.memory_mb);
                
                // Validate against realistic development targets
                validate_performance(&result);
                println!();
            }
            Err(e) => {
                println!("  ❌ Error: {}", e);
            }
        }
    }
}

fn benchmark_development_workflow(project_path: &PathBuf) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
    let js_parser = JavaScriptParser;
    
    // 1. Cold start: Full project analysis (like opening HallWatch first time)
    let cold_start_time = Instant::now();
    let (file_cache, endpoints_found) = parse_all_files(project_path, &js_parser)?;
    let cold_start_ms = cold_start_time.elapsed().as_millis() as f64;
    
    // 2. Hot reload: Simulate file change and re-parse just that file
    let hot_reload_time = Instant::now();
    let changed_file = find_sample_js_file(project_path);
    if let Some(ref file_path) = changed_file {
        simulate_file_change(file_path, &js_parser)?;
    }
    let hot_reload_ms = hot_reload_time.elapsed().as_millis() as f64;
    
    // 3. Incremental: Parse only what changed (realistic development workflow)
    let incremental_time = Instant::now();
    let files_to_reparse = vec![changed_file.clone().unwrap_or_else(|| PathBuf::from("dummy"))];
    let _incremental_endpoints = parse_specific_files(&files_to_reparse, &js_parser)?;
    let incremental_ms = incremental_time.elapsed().as_millis() as f64;
    
    // Measure memory with full cache
    let (_result, memory_mb) = measure_memory_usage(|| {
        file_cache.len()
    });
    
    Ok(BenchmarkResult {
        total_files: file_cache.len(),
        endpoints_found,
        cold_start_ms,
        hot_reload_ms,
        incremental_ms,
        memory_mb,
    })
}

fn parse_all_files(project_path: &PathBuf, parser: &JavaScriptParser) -> Result<(HashMap<PathBuf, String>, usize), Box<dyn std::error::Error>> {
    let mut file_cache = HashMap::new();
    let mut total_endpoints = 0;
    
    for entry in WalkDir::new(project_path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            
            if let Some(language) = detect_language(file_path) {
                if language == "javascript" {
                    if let Ok(content) = fs::read_to_string(file_path) {
                        file_cache.insert(file_path.to_path_buf(), content.clone());
                        
                        if let Ok(endpoints) = parser.parse(&content) {
                            total_endpoints += endpoints.len();
                        }
                    }
                }
            }
        }
    }
    
    Ok((file_cache, total_endpoints))
}

fn find_sample_js_file(project_path: &PathBuf) -> Option<PathBuf> {
    for entry in WalkDir::new(project_path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if let Some(language) = detect_language(file_path) {
                if language == "javascript" {
                    return Some(file_path.to_path_buf());
                }
            }
        }
    }
    None
}

fn simulate_file_change(file_path: &PathBuf, parser: &JavaScriptParser) -> Result<usize, Box<dyn std::error::Error>> {
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(endpoints) = parser.parse(&content) {
            return Ok(endpoints.len());
        }
    }
    Ok(0)
}

fn parse_specific_files(file_paths: &[PathBuf], parser: &JavaScriptParser) -> Result<usize, Box<dyn std::error::Error>> {
    let mut total_endpoints = 0;
    
    for file_path in file_paths {
        if file_path.exists() {
            if let Ok(content) = fs::read_to_string(file_path) {
                if let Ok(endpoints) = parser.parse(&content) {
                    total_endpoints += endpoints.len();
                }
            }
        }
    }
    
    Ok(total_endpoints)
}

fn validate_performance(result: &BenchmarkResult) {
    // Cold start can be slower (user expectation: initial load)
    if result.cold_start_ms < 100.0 {
        println!("  🎯 Cold start: EXCELLENT (<100ms)");
    } else if result.cold_start_ms < 500.0 {
        println!("  ✅ Cold start: ACCEPTABLE (<500ms)");
    } else {
        println!("  ⚠️  Cold start: SLOW (>500ms)");
    }
    
    // Hot reload should be fast (user expectation: instant feedback)
    if result.hot_reload_ms < 10.0 {
        println!("  🚀 Hot reload: EXCELLENT (<10ms)");
    } else if result.hot_reload_ms < 50.0 {
        println!("  ✅ Hot reload: GOOD (<50ms)");
    } else {
        println!("  ❌ Hot reload: TOO SLOW (>50ms) - affects dev experience");
    }
    
    // Incremental should be near-instant (user expectation: real-time)
    if result.incremental_ms < 5.0 {
        println!("  ⚡ Incremental: PERFECT (<5ms)");
    } else if result.incremental_ms < 20.0 {
        println!("  ✅ Incremental: GOOD (<20ms)");
    } else {
        println!("  ❌ Incremental: TOO SLOW (>20ms) - blocks productivity");
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
