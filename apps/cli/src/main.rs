use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use std::fs;
use reqsmith_core::{FileWatcher, FileEvent, FileEventType, EnhancedWatcher};
use reqsmith_parser::{detect_language, languages::{javascript::JavaScriptParser, python::PythonParser, php::PhpParser}, LanguageParser, IncrementalParser, EndpointChanges};
use reqsmith_parser::config::ConfigDiscovery;
use reqsmith_diff::{ChangeSource, DiffProcessor};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Watch a directory for API changes
    Watch {
        /// Path to watch
        path: PathBuf,
        /// Port to serve on
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Discover APIs in a directory
    Discover {
        /// Path to scan
        path: PathBuf,
        /// Output format (json, table)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    /// Generate configuration for a project
    Config {
        /// Path to analyze
        path: PathBuf,
        /// Enable debug mode for detailed output
        #[arg(short, long)]
        debug: bool,
        /// Output format (json, js)
        #[arg(short, long, default_value = "js")]
        format: String,
    },
    /// Incremental watching with diff-based parsing
    WatchIncremental {
        /// Path to watch
        path: PathBuf,
        /// State file for persistence
        #[arg(long)]
        state_file: Option<PathBuf>,
        /// Enable git integration
        #[arg(long)]
        git: bool,
    },
    /// Analyze git commit changes
    GitDiff {
        /// From commit (default: HEAD~1)
        #[arg(short, long, default_value = "HEAD~1")]
        from: String,
        /// To commit (default: HEAD)
        #[arg(short, long, default_value = "HEAD")]
        to: String,
        /// Repository path
        #[arg(short, long)]
        repo: Option<PathBuf>,
    },
    /// Detect breaking API changes between commits
    Breaking {
        /// Base commit (default: main)
        #[arg(short, long, default_value = "main")]
        base: String,
        /// Head commit (default: HEAD)
        #[arg(short, long, default_value = "HEAD")]
        head: String,
        /// Repository path
        #[arg(short, long)]
        repo: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Watch { path, port } => {
            info!("Starting Reqsmith watcher on {} (port {})", path.display(), port);
            println!("🔍 Watching {} for API changes...", path.display());
            println!("🌐 Web interface available at http://localhost:{}", port);
            
            let mut watcher = FileWatcher::new();
            let mut rx = watcher.watch(&path).await?;
            
            println!("👁️  File watcher started. Press Ctrl+C to stop.");
            
            loop {
                tokio::select! {
                    file_event = rx.recv() => {
                        if let Some(event) = file_event {
                            handle_file_event(event).await?;
                        }
                    }
                    _ = tokio::signal::ctrl_c() => {
                        break;
                    }
                }
            }
            
            println!("\n👋 Goodbye!");
        }
        Commands::Discover { path, format } => {
            if format == "json" {
                // For JSON output, suppress logging to avoid invalid JSON
                let endpoints = discover_endpoints(&path)?;
                println!("{}", serde_json::to_string_pretty(&endpoints)?);
            } else {
                info!("Discovering APIs in {}", path.display());
                println!("🔍 Scanning {} for API endpoints...", path.display());
                
                let endpoints = discover_endpoints(&path)?;
                
                if endpoints.is_empty() {
                    println!("No API endpoints found.");
                } else {
                    println!("\n📋 Found {} endpoint(s):", endpoints.len());
                    println!("{:<8} {:<30} {:<20} {:<10}", "METHOD", "PATH", "HANDLER", "LINE");
                    println!("{:-<70}", "");
                    for endpoint in endpoints {
                        println!("{:<8} {:<30} {:<20} {:<10}", 
                            format!("{:?}", endpoint.method),
                            endpoint.path,
                            endpoint.handler,
                            endpoint.line
                        );
                    }
                }
            }
        }
        Commands::Config { path, debug, format } => {
            info!("Generating configuration for {}", path.display());
            println!("🔧 Analyzing project structure in {}...", path.display());
            
            let discovery = ConfigDiscovery::new(debug);
            let config = discovery.discover(&path).await?;
            
            match format.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&config)?);
                }
                "js" => {
                    let config_path = path.join(".reqsmith/discovered.config.js");
                    if config_path.exists() {
                        println!("✅ Configuration generated at: {}", config_path.display());
                        println!("📊 Project Analysis:");
                        println!("   Type: {}", config.project_structure.project_type);
                        println!("   Files: {}", config.project_structure.file_count);
                        println!("   Frameworks detected: {}", config.frameworks.len());
                        
                        for framework in &config.frameworks {
                            println!("     - {} (confidence: {:.1}%)", framework.framework, framework.confidence * 100.0);
                        }
                        
                        if debug {
                            println!("\n🐛 Debug files created:");
                            println!("   - {}", config_path.display());
                            let json_path = path.join(".reqsmith/discovered.config.json");
                            if json_path.exists() {
                                println!("   - {}", json_path.display());
                            }
                        }
                        
                        println!("\n💡 You can now:");
                        println!("   1. Review the generated configuration");
                        println!("   2. Add custom patterns in the 'overrides' section");
                        println!("   3. Run 'reqsmith discover {}' to test endpoint detection", path.display());
                    } else {
                        println!("❌ Failed to create configuration file");
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!("Invalid format: {}. Use 'json' or 'js'", format));
                }
            }
        }
        Commands::WatchIncremental { path, state_file, git } => {
            info!("Starting incremental watcher on {}", path.display());
            println!("🔍 Watching {} with incremental parsing...", path.display());
            
            let repo_path = if git { Some(path.as_path()) } else { None };
            let mut watcher = EnhancedWatcher::new(repo_path)?;
            let mut change_rx = watcher.watch_changes(&path).await?;
            
            // Load previous state if specified
            let mut incremental_parser = if let Some(state_path) = &state_file {
                load_parser_state(state_path)?
            } else {
                IncrementalParser::new()
            };
            
            println!("👁️  Incremental watcher started. Press Ctrl+C to stop.");
            
            loop {
                tokio::select! {
                    change_event = change_rx.recv() => {
                        if let Some(event) = change_event {
                            let changes = incremental_parser.parse_changes(event).await?;
                            print_endpoint_changes(&changes);
                            
                            // Save state if specified
                            if let Some(state_path) = &state_file {
                                save_parser_state(&incremental_parser, state_path)?;
                            }
                        }
                    }
                    _ = tokio::signal::ctrl_c() => {
                        break;
                    }
                }
            }
            
            println!("\n👋 Goodbye!");
        }
        Commands::GitDiff { from, to, repo } => {
            let repo_path = repo.as_deref().unwrap_or_else(|| std::path::Path::new("."));
            println!("🔍 Analyzing git diff {}..{} in {}", from, to, repo_path.display());
            
            let diff_processor = DiffProcessor::new(Some(repo_path))?;
            let change_event = diff_processor.process_change(ChangeSource::GitDiff {
                from: from.clone(),
                to: to.clone(),
            }).await?;
            
            let mut incremental_parser = IncrementalParser::new();
            let changes = incremental_parser.parse_changes(change_event).await?;
            
            println!("\n📊 API Changes in {}..{}:", from, to);
            print_endpoint_changes(&changes);
        }
        Commands::Breaking { base, head, repo } => {
            let repo_path = repo.as_deref().unwrap_or_else(|| std::path::Path::new("."));
            println!("🔍 Checking for breaking changes {}..{} in {}", base, head, repo_path.display());
            
            let diff_processor = DiffProcessor::new(Some(repo_path))?;
            let change_event = diff_processor.process_change(ChangeSource::GitDiff {
                from: base.clone(),
                to: head.clone(),
            }).await?;
            
            let mut incremental_parser = IncrementalParser::new();
            let changes = incremental_parser.parse_changes(change_event).await?;
            
            let breaking_changes = detect_breaking_changes(&changes);
            
            if breaking_changes.is_empty() {
                println!("✅ No breaking changes detected!");
            } else {
                println!("⚠️  Breaking changes detected:");
                for breaking_change in breaking_changes {
                    println!("   🚨 {}", breaking_change);
                }
            }
        }
    }
    
    Ok(())
}

async fn handle_file_event(event: FileEvent) -> Result<()> {
    match event.event_type {
        FileEventType::Created => {
            println!("📄 Created: {}", event.path.display());
            if should_parse_file(&event.path) {
                analyze_file(&event.path).await?;
            }
        }
        FileEventType::Modified => {
            println!("✏️  Modified: {}", event.path.display());
            if should_parse_file(&event.path) {
                analyze_file(&event.path).await?;
            }
        }
        FileEventType::Deleted => {
            println!("🗑️  Deleted: {}", event.path.display());
        }
        FileEventType::Renamed { from, to } => {
            println!("🔄 Renamed: {} -> {}", from.display(), to.display());
            if should_parse_file(&to) {
                analyze_file(&to).await?;
            }
        }
    }
    Ok(())
}

fn should_parse_file(path: &PathBuf) -> bool {
    detect_language(path).is_some()
}

async fn analyze_file(path: &PathBuf) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let endpoints = parse_file_content(&content, path)?;
    
    if !endpoints.is_empty() {
        println!("   📍 Found {} endpoint(s) in {}", endpoints.len(), path.display());
        for endpoint in endpoints {
            println!("      {} {} (line {})", 
                format!("{:?}", endpoint.method),
                endpoint.path,
                endpoint.line
            );
        }
    }
    
    Ok(())
}

fn discover_endpoints(path: &PathBuf) -> Result<Vec<reqsmith_parser::Endpoint>> {
    let mut all_endpoints = Vec::new();
    
    if path.is_file() {
        let content = fs::read_to_string(path)?;
        let endpoints = parse_file_content(&content, path)?;
        all_endpoints.extend(endpoints);
    } else if path.is_dir() {
        for entry in walkdir::WalkDir::new(path) {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && should_parse_file(&path.to_path_buf()) {
                let content = fs::read_to_string(path)?;
                let endpoints = parse_file_content(&content, &path.to_path_buf())?;
                all_endpoints.extend(endpoints);
            }
        }
    }
    
    Ok(all_endpoints)
}

fn parse_file_content(content: &str, path: &PathBuf) -> Result<Vec<reqsmith_parser::Endpoint>> {
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let parser: Box<dyn LanguageParser> = match extension {
        "js" | "mjs" | "ts" | "tsx" => Box::new(JavaScriptParser),
        "py" => Box::new(PythonParser),
        "php" => Box::new(PhpParser::new()?),
        _ => return Ok(vec![]),
    };
    
    parser.parse(content)
}

/// Print endpoint changes in a formatted way
fn print_endpoint_changes(changes: &EndpointChanges) {
    if !changes.has_changes() && changes.unchanged.is_empty() {
        println!("   No changes detected");
        return;
    }

    if !changes.added.is_empty() {
        println!("   ✅ Added {} endpoint(s):", changes.added.len());
        for endpoint in &changes.added {
            println!("      + {} {} (line {})", 
                format!("{:?}", endpoint.method),
                endpoint.path,
                endpoint.line
            );
        }
    }

    if !changes.modified.is_empty() {
        println!("   📝 Modified {} endpoint(s):", changes.modified.len());
        for change in &changes.modified {
            println!("      ~ {} {} (line {} -> {})", 
                format!("{:?}", change.new.method),
                change.new.path,
                change.old.line,
                change.new.line
            );
        }
    }

    if !changes.removed.is_empty() {
        println!("   ❌ Removed {} endpoint(s):", changes.removed.len());
        for endpoint in &changes.removed {
            println!("      - {} {} (line {})", 
                format!("{:?}", endpoint.method),
                endpoint.path,
                endpoint.line
            );
        }
    }

    if !changes.unchanged.is_empty() {
        println!("   📊 {} endpoint(s) unchanged", changes.unchanged.len());
    }
}

/// Detect breaking changes from endpoint changes
fn detect_breaking_changes(changes: &EndpointChanges) -> Vec<String> {
    let mut breaking_changes = Vec::new();

    // Removed endpoints are always breaking
    for endpoint in &changes.removed {
        breaking_changes.push(format!(
            "Removed endpoint: {} {}",
            format!("{:?}", endpoint.method),
            endpoint.path
        ));
    }

    // Modified endpoints might be breaking
    for change in &changes.modified {
        match change.change_type {
            reqsmith_parser::ChangeType::PathChanged => {
                breaking_changes.push(format!(
                    "Changed path: {} {} -> {}",
                    format!("{:?}", change.old.method),
                    change.old.path,
                    change.new.path
                ));
            }
            reqsmith_parser::ChangeType::MethodChanged => {
                breaking_changes.push(format!(
                    "Changed method: {} {} -> {} {}",
                    format!("{:?}", change.old.method),
                    change.old.path,
                    format!("{:?}", change.new.method),
                    change.new.path
                ));
            }
            _ => {
                // Other changes are potentially breaking but less critical
            }
        }
    }

    breaking_changes
}

/// Load parser state from file
fn load_parser_state(state_path: &std::path::Path) -> Result<IncrementalParser> {
    if state_path.exists() {
        let content = fs::read_to_string(state_path)?;
        let state: reqsmith_parser::incremental::EndpointState = serde_json::from_str(&content)?;
        Ok(IncrementalParser::with_state(state))
    } else {
        Ok(IncrementalParser::new())
    }
}

/// Save parser state to file
fn save_parser_state(parser: &IncrementalParser, state_path: &std::path::Path) -> Result<()> {
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(parser.get_state())?;
    fs::write(state_path, content)?;
    Ok(())
}
