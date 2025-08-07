use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use reqsmith_core::watcher::Watcher;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

/// Micro-benchmarks for file watching performance
/// These tests focus on file system monitoring and change detection

fn benchmark_file_discovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_discovery");
    
    // Create test directories with different file counts
    let small_dir = create_test_project(ProjectSize::Small);
    let medium_dir = create_test_project(ProjectSize::Medium);
    
    group.bench_with_input(
        BenchmarkId::new("discover", "small_project"),
        &small_dir.path(),
        |b, path| {
            b.iter(|| {
                let watcher = Watcher::new(path);
                black_box(watcher.discover_files())
            });
        },
    );
    
    group.bench_with_input(
        BenchmarkId::new("discover", "medium_project"),
        &medium_dir.path(),
        |b, path| {
            b.iter(|| {
                let watcher = Watcher::new(path);
                black_box(watcher.discover_files())
            });
        },
    );
    
    group.finish();
}

fn benchmark_change_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_detection");
    group.sample_size(20);
    
    let test_dir = create_test_project(ProjectSize::Small);
    let test_file = test_dir.path().join("src/routes/dynamic.js");
    
    group.bench_function("single_file_change", |b| {
        b.iter(|| {
            // Simulate file change
            let content = format!(
                "app.get('/test/{}', (req, res) => res.json({{}}));",
                chrono::Utc::now().timestamp_nanos()
            );
            
            fs::write(&test_file, content).unwrap();
            
            let watcher = Watcher::new(test_dir.path());
            black_box(watcher.check_for_changes())
        });
    });
    
    group.finish();
}

fn benchmark_watch_initialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("watch_init");
    
    let small_dir = create_test_project(ProjectSize::Small);
    let medium_dir = create_test_project(ProjectSize::Medium);
    
    group.bench_with_input(
        BenchmarkId::new("init", "small_project"),
        &small_dir.path(),
        |b, path| {
            b.iter(|| {
                black_box(Watcher::new(path))
            });
        },
    );
    
    group.bench_with_input(
        BenchmarkId::new("init", "medium_project"),
        &medium_dir.path(),
        |b, path| {
            b.iter(|| {
                black_box(Watcher::new(path))
            });
        },
    );
    
    group.finish();
}

fn benchmark_file_filtering(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_filtering");
    
    // Create directory with mixed file types
    let mixed_dir = create_mixed_file_project();
    
    group.bench_function("filter_relevant_files", |b| {
        b.iter(|| {
            let watcher = Watcher::new(mixed_dir.path());
            black_box(watcher.get_relevant_files())
        });
    });
    
    group.finish();
}

fn benchmark_batch_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_changes");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(15));
    
    let test_dir = create_test_project(ProjectSize::Medium);
    
    group.bench_function("simultaneous_changes", |b| {
        b.iter(|| {
            // Simulate multiple files changing at once (like git branch switch)
            let files_to_change = vec![
                "src/routes/users.js",
                "src/routes/posts.js", 
                "src/routes/auth.js",
                "src/middleware/cors.js",
                "src/config/database.js",
            ];
            
            for file in &files_to_change {
                let file_path = test_dir.path().join(file);
                let content = format!(
                    "// Modified at {}\nmodule.exports = {{}};",
                    chrono::Utc::now().timestamp_nanos()
                );
                fs::write(file_path, content).unwrap();
            }
            
            let watcher = Watcher::new(test_dir.path());
            black_box(watcher.check_for_changes())
        });
    });
    
    group.finish();
}

// Helper types and functions
#[derive(Clone)]
enum ProjectSize {
    Small,  // ~25 files
    Medium, // ~150 files
}

fn create_test_project(size: ProjectSize) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create directory structure
    fs::create_dir_all(base_path.join("src/routes")).unwrap();
    fs::create_dir_all(base_path.join("src/middleware")).unwrap();
    fs::create_dir_all(base_path.join("src/models")).unwrap();
    fs::create_dir_all(base_path.join("src/config")).unwrap();
    fs::create_dir_all(base_path.join("tests")).unwrap();
    
    let file_count = match size {
        ProjectSize::Small => 25,
        ProjectSize::Medium => 150,
    };
    
    // Create route files
    for i in 0..(file_count / 3) {
        let route_content = format!(
            r#"
const express = require('express');
const router = express.Router();

router.get('/resource{}/items', (req, res) => {{
    res.json({{ message: 'Resource {} items' }});
}});

router.post('/resource{}/items', (req, res) => {{
    res.status(201).json(req.body);
}});

module.exports = router;
"#,
            i, i, i
        );
        
        fs::write(
            base_path.join(format!("src/routes/resource{}.js", i)),
            route_content,
        ).unwrap();
    }
    
    // Create middleware files
    for i in 0..(file_count / 6) {
        let middleware_content = format!(
            r#"
module.exports = (req, res, next) => {{
    // Middleware {} logic
    console.log('Middleware {} executed');
    next();
}};
"#,
            i, i
        );
        
        fs::write(
            base_path.join(format!("src/middleware/middleware{}.js", i)),
            middleware_content,
        ).unwrap();
    }
    
    // Create model files
    for i in 0..(file_count / 6) {
        let model_content = format!(
            r#"
const {{ DataTypes }} = require('sequelize');

module.exports = (sequelize) => {{
    return sequelize.define('Model{}', {{
        id: {{
            type: DataTypes.INTEGER,
            primaryKey: true,
            autoIncrement: true,
        }},
        name: {{
            type: DataTypes.STRING,
            allowNull: false,
        }},
    }});
}};
"#,
            i
        );
        
        fs::write(
            base_path.join(format!("src/models/model{}.js", i)),
            model_content,
        ).unwrap();
    }
    
    // Create package.json
    let package_json = r#"
{
  "name": "test-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0"
  }
}
"#;
    fs::write(base_path.join("package.json"), package_json).unwrap();
    
    temp_dir
}

fn create_mixed_file_project() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create directories
    fs::create_dir_all(base_path.join("src")).unwrap();
    fs::create_dir_all(base_path.join("node_modules")).unwrap();
    fs::create_dir_all(base_path.join("dist")).unwrap();
    fs::create_dir_all(base_path.join(".git")).unwrap();
    
    // Create relevant files (should be included)
    let relevant_files = vec![
        ("src/app.js", "const express = require('express'); const app = express();"),
        ("src/routes.py", "from flask import Flask\n@app.route('/')"),
        ("src/api.php", "<?php\nRoute::get('/', function() {});"),
        ("package.json", r#"{"name": "test"}"#),
        ("requirements.txt", "flask==2.0.0"),
        ("composer.json", r#"{"name": "test"}"#),
    ];
    
    for (file, content) in relevant_files {
        fs::write(base_path.join(file), content).unwrap();
    }
    
    // Create irrelevant files (should be filtered out)
    let irrelevant_files = vec![
        ("node_modules/express/index.js", "module.exports = {};"),
        ("dist/bundle.js", "// Generated code"),
        (".git/config", "[core]"),
        ("README.md", "# Test Project"),
        ("image.png", "fake image content"),
        (".env", "SECRET=test"),
        ("yarn.lock", "# yarn lockfile"),
    ];
    
    for (file, content) in irrelevant_files {
        let file_path = base_path.join(file);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(file_path, content).unwrap();
    }
    
    temp_dir
}

criterion_group!(
    benches,
    benchmark_file_discovery,
    benchmark_change_detection,
    benchmark_watch_initialization,
    benchmark_file_filtering,
    benchmark_batch_changes
);

criterion_main!(benches);
