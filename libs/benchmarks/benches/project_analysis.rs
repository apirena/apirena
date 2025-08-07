use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::path::PathBuf;
use std::fs;
use reqsmith_parser::{LanguageParser, languages::javascript::JavaScriptParser, languages::python::PythonParser, languages::php::PhpParser, detect_language};
use walkdir::WalkDir;

fn get_test_projects() -> Vec<(String, PathBuf)> {
    // Try multiple possible locations since cargo bench can run from different directories
    let possible_roots = vec![
        std::env::current_dir().expect("Failed to get current directory"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("../.."),
        PathBuf::from("../../.."),
    ];
    
    eprintln!("🔍 Searching for test projects...");
    
    for root in possible_roots {
        let projects_dir = root.join("libs/benchmarks/projects");
        eprintln!("  Checking: {}", projects_dir.display());
        
        if projects_dir.exists() {
            eprintln!("  ✅ Found projects directory!");
            let mut projects = Vec::new();

            // Collect all projects from all categories
            for category in &["small", "medium", "large"] {
                let category_path = projects_dir.join(category);
                eprintln!("    Checking category: {}", category_path.display());
                
                if !category_path.exists() {
                    eprintln!("      ❌ Category doesn't exist");
                    continue;
                }

                if let Ok(entries) = fs::read_dir(&category_path) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            let project_name = format!("{}_{}", 
                                category,
                                entry.file_name().to_string_lossy()
                            );
                            eprintln!("      📂 Found project: {}", project_name);
                            projects.push((project_name, entry.path()));
                        }
                    }
                }
            }
            
            if !projects.is_empty() {
                eprintln!("🎉 Found {} projects total", projects.len());
                return projects;
            }
        } else {
            eprintln!("  ❌ Directory doesn't exist");
        }
    }

    eprintln!("⚠️  No test projects found in any location");
    Vec::new()
}

#[derive(Debug)]
struct ProjectAnalysis {
    total_files: usize,
    parseable_files: usize,
    endpoints_found: usize,
    languages_detected: Vec<String>,
    parse_time_ms: f64,
}

fn analyze_project(project_path: &PathBuf) -> Result<ProjectAnalysis, Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    
    let js_parser = JavaScriptParser;
    let py_parser = PythonParser;
    let php_parser = PhpParser;
    
    let mut total_files = 0;
    let mut parseable_files = 0;
    let mut endpoints_found = 0;
    let mut languages_detected = Vec::new();
    
    for entry in WalkDir::new(project_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            total_files += 1;
            let file_path = entry.path();
            
            if let Some(language) = detect_language(file_path) {
                if !languages_detected.contains(&language.to_string()) {
                    languages_detected.push(language.to_string());
                }
                
                if let Ok(content) = fs::read_to_string(file_path) {
                    parseable_files += 1;
                    
                    let parser_result = match language {
                        "javascript" => js_parser.parse(&content),
                        "python" => py_parser.parse(&content),
                        "php" => php_parser.parse(&content),
                        _ => continue,
                    };
                    
                    if let Ok(endpoints) = parser_result {
                        endpoints_found += endpoints.len();
                    }
                }
            }
        }
    }
    
    let parse_time_ms = start.elapsed().as_millis() as f64;
    
    Ok(ProjectAnalysis {
        total_files,
        parseable_files,
        endpoints_found,
        languages_detected,
        parse_time_ms,
    })
}

fn benchmark_project_analysis(c: &mut Criterion) {
    let projects = get_test_projects();
    
    if projects.is_empty() {
        eprintln!("No test projects found. Skipping benchmark.");
        return;
    }
    
    let mut group = c.benchmark_group("project_analysis");
    
    for (project_name, project_path) in projects {
        group.bench_with_input(
            BenchmarkId::new("analyze", &project_name),
            &project_path,
            |b, path| {
                b.iter(|| {
                    black_box(analyze_project(path).unwrap_or_else(|e| {
                        eprintln!("Error analyzing {}: {}", project_name, e);
                        ProjectAnalysis {
                            total_files: 0,
                            parseable_files: 0,
                            endpoints_found: 0,
                            languages_detected: vec![],
                            parse_time_ms: 0.0,
                        }
                    }))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_file_discovery(c: &mut Criterion) {
    let projects = get_test_projects();
    
    if projects.is_empty() {
        return;
    }
    
    let mut group = c.benchmark_group("file_discovery");
    
    for (project_name, project_path) in projects {
        group.bench_with_input(
            BenchmarkId::new("discover_files", &project_name),
            &project_path,
            |b, path| {
                b.iter(|| {
                    let mut file_count = 0;
                    for entry in WalkDir::new(black_box(path))
                        .follow_links(false)
                        .into_iter()
                        .filter_map(|e| e.ok())
                    {
                        if entry.file_type().is_file() {
                            file_count += 1;
                        }
                    }
                    black_box(file_count)
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_parsing_only(c: &mut Criterion) {
    let projects = get_test_projects();
    
    if projects.is_empty() {
        return;
    }
    
    let mut group = c.benchmark_group("parsing_only");
    
    // Collect sample JavaScript files for parsing benchmarks
    let mut js_files = Vec::new();
    for (_project_name, project_path) in &projects {
        for entry in WalkDir::new(project_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .take(5) // Limit to first 5 files per project for benchmarking
        {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                if let Some(ext) = file_path.extension() {
                    if ext == "js" || ext == "mjs" {
                        if let Ok(content) = fs::read_to_string(file_path) {
                            let file_id = file_path.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            js_files.push((file_id, content));
                        }
                    }
                }
            }
        }
    }
    
    let parser = JavaScriptParser;
    
    for (file_name, content) in js_files {
        group.bench_with_input(
            BenchmarkId::new("parse_js_file", &file_name),
            &content,
            |b, content| {
                b.iter(|| {
                    black_box(parser.parse(black_box(content)).unwrap_or_default())
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_project_analysis,
    benchmark_file_discovery,
    benchmark_parsing_only
);
criterion_main!(benches);
