use std::path::PathBuf;
use std::fs;
use std::time::Instant;
use pinpath_parser::{LanguageParser, languages::javascript::JavaScriptParser, detect_language};
use pinpath_benchmarks::measure_memory_usage;
use walkdir::WalkDir;

fn main() {
    println!("🚀 PinPath Performance Benchmark Suite");
    println!("==========================================");
    println!("Testing real-world endpoint discovery performance\n");
    
    let projects = discover_test_projects();
    
    if projects.is_empty() {
        println!("❌ No test projects found");
        println!("📍 Please ensure test projects exist in libs/benchmarks/projects/");
        return;
    }
    
    println!("📂 Found {} test projects:\n", projects.len());
    
    // Test each project and collect results
    let mut all_results = Vec::new();
    
    for (project_name, project_path) in projects {
        println!("🔍 Analyzing: {}", project_name);
        
        let (analysis_result, memory_mb) = measure_memory_usage(|| {
            analyze_project_performance(&project_path)
        });
        
        match analysis_result {
            Ok(result) => {
                println!("  ✅ Files: {} | Endpoints: {} | Time: {:.2}ms | Memory: {:.2}MB", 
                    result.total_files, 
                    result.endpoints_found, 
                    result.parse_time_ms,
                    memory_mb
                );
                
                // Performance validation against targets from architecture docs
                if result.parse_time_ms < 10.0 {
                    println!("  🎯 EXCELLENT: Parse time under 10ms target");
                } else if result.parse_time_ms < 50.0 {
                    println!("  ✅ GOOD: Parse time acceptable");
                } else {
                    println!("  ⚠️  SLOW: Parse time above target");
                }
                
                all_results.push((project_name, result, memory_mb));
            }
            Err(e) => {
                println!("  ❌ Analysis failed: {}", e);
            }
        }
        println!();
    }
    
    // Summary report
    print_performance_summary(&all_results);
}

fn discover_test_projects() -> Vec<(String, PathBuf)> {
    let workspace_root = std::env::current_dir()
        .expect("Failed to get current directory");
    
    let projects_dir = workspace_root.join("libs/benchmarks/projects");
    
    if !projects_dir.exists() {
        return Vec::new();
    }

    let mut projects = Vec::new();

    for category in &["small", "medium", "large"] {
        let category_path = projects_dir.join(category);
        if !category_path.exists() {
            continue;
        }

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

    projects
}

#[derive(Debug)]
struct ProjectPerformance {
    total_files: usize,
    endpoints_found: usize,
    parse_time_ms: f64,
}

fn analyze_project_performance(project_path: &PathBuf) -> Result<ProjectPerformance, Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    let js_parser = JavaScriptParser;
    let mut total_files = 0;
    let mut endpoints_found = 0;
    
    for entry in WalkDir::new(project_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            total_files += 1;
            let file_path = entry.path();
            
            if let Some(language) = detect_language(file_path) {
                if let Ok(content) = fs::read_to_string(file_path) {
                    if language == "javascript" {
                        if let Ok(endpoints) = js_parser.parse(&content) {
                            endpoints_found += endpoints.len();
                        }
                    }
                }
            }
        }
    }
    
    let parse_time_ms = start.elapsed().as_millis() as f64;
    
    Ok(ProjectPerformance {
        total_files,
        endpoints_found,
        parse_time_ms,
    })
}

fn print_performance_summary(results: &[(String, ProjectPerformance, f64)]) {
    println!("📊 PERFORMANCE SUMMARY");
    println!("======================");
    
    if results.is_empty() {
        println!("No results to summarize");
        return;
    }
    
    let total_files: usize = results.iter().map(|(_, r, _)| r.total_files).sum();
    let total_endpoints: usize = results.iter().map(|(_, r, _)| r.endpoints_found).sum();
    let avg_parse_time: f64 = results.iter().map(|(_, r, _)| r.parse_time_ms).sum::<f64>() / results.len() as f64;
    let avg_memory: f64 = results.iter().map(|(_, _, m)| *m).sum::<f64>() / results.len() as f64;
    
    println!("📈 Total files processed: {}", total_files);
    println!("🎯 Total endpoints found: {}", total_endpoints);
    println!("⏱️  Average parse time: {:.2}ms", avg_parse_time);
    println!("💾 Average memory usage: {:.2}MB", avg_memory);
    
    // Architecture target validation
    println!("\n🏆 ARCHITECTURE TARGET VALIDATION");
    if avg_parse_time < 10.0 {
        println!("✅ Parse time target (<10ms): PASSED ({:.2}ms)", avg_parse_time);
    } else {
        println!("❌ Parse time target (<10ms): FAILED ({:.2}ms)", avg_parse_time);
    }
    
    if avg_memory < 50.0 {
        println!("✅ Memory usage target (<50MB): PASSED ({:.2}MB)", avg_memory);
    } else {
        println!("❌ Memory usage target (<50MB): FAILED ({:.2}MB)", avg_memory);
    }
    
    println!("\n✨ Benchmark complete! All projects analyzed successfully.");
}
