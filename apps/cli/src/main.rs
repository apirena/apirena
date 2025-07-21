use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use std::fs;
use hallwatch_core::{FileWatcher, FileEvent, FileEventType};
use hallwatch_parser::{detect_language, languages::{javascript::JavaScriptParser, python::PythonParser, php::PhpParser}, LanguageParser};
use hallwatch_parser::config::ConfigDiscovery;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Watch { path, port } => {
            info!("Starting Hallwatch watcher on {} (port {})", path.display(), port);
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
                    let config_path = path.join(".hallwatch/discovered.config.js");
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
                            let json_path = path.join(".hallwatch/discovered.config.json");
                            if json_path.exists() {
                                println!("   - {}", json_path.display());
                            }
                        }
                        
                        println!("\n💡 You can now:");
                        println!("   1. Review the generated configuration");
                        println!("   2. Add custom patterns in the 'overrides' section");
                        println!("   3. Run 'hallwatch discover {}' to test endpoint detection", path.display());
                    } else {
                        println!("❌ Failed to create configuration file");
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!("Invalid format: {}. Use 'json' or 'js'", format));
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

fn discover_endpoints(path: &PathBuf) -> Result<Vec<hallwatch_parser::Endpoint>> {
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

fn parse_file_content(content: &str, path: &PathBuf) -> Result<Vec<hallwatch_parser::Endpoint>> {
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
