use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use std::fs;
use hallwatch_core::{FileWatcher, FileEvent, FileEventType};
use hallwatch_parser::{detect_language, languages::{javascript::JavaScriptParser, python::PythonParser, php::PhpParser}, LanguageParser};

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
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Watch { path, port } => {
            info!("Starting Apirena watcher on {} (port {})", path.display(), port);
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
