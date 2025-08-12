use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use tauri::{AppHandle, Emitter};
use tokio::task;

use reqsmith_parser::{IncrementalParser, incremental::EndpointState, Endpoint, detect_language, LanguageParser};
use reqsmith_parser::languages::{javascript::JavaScriptParser, python::PythonParser, php::PhpParser};

use crate::storage::{ReqSmithStorage, EndpointRecord};

pub struct ProjectWatcher {
    _watcher: RecommendedWatcher,
    project_path: PathBuf,
}

/// Global registry of active watchers
pub struct WatcherRegistry {
    watchers: Arc<Mutex<HashMap<String, ProjectWatcher>>>,
}

impl WatcherRegistry {
    pub fn new() -> Self {
        Self {
            watchers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start_watching(
        &self,
        project_path: PathBuf,
        app_handle: AppHandle,
    ) -> Result<String, String> {
        let watch_id = format!("watch:{}", project_path.display());
        
        // Remove existing watcher for this project if any
        self.stop_watching(&watch_id).await;

        let (tx, rx) = mpsc::channel();
        
        // Create file system watcher
        let mut watcher = notify::recommended_watcher(tx)
            .map_err(|e| format!("Failed to create watcher: {}", e))?;
        
        watcher.watch(&project_path, RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch directory: {}", e))?;
        
        // Initialize storage and parser
        let storage = ReqSmithStorage::new(project_path.clone())
            .map_err(|e| format!("Failed to initialize storage: {}", e))?;
        
        // Load previous parser state
        let previous_state = storage.load_parser_state().unwrap_or_else(|_| EndpointState {
            endpoints: HashMap::new(),
            file_hashes: HashMap::new(),
            last_updated: std::time::SystemTime::now(),
        });
        let parser = Arc::new(Mutex::new(IncrementalParser::with_state(previous_state)));
        
        // Store the watcher
        {
            let mut watchers = self.watchers.lock().unwrap();
            watchers.insert(watch_id.clone(), ProjectWatcher {
                _watcher: watcher,
                project_path: project_path.clone(),
            });
        }
        
        // Spawn background task to handle file events
        let parser_clone = parser.clone();
        let storage_clone = storage;
        let app_handle_clone = app_handle.clone();
        let project_path_clone = project_path.clone();
        
        task::spawn(async move {
            while let Ok(event_result) = rx.recv() {
                match event_result {
                    Ok(event) => {
                        if let Err(e) = handle_file_event(
                            event, 
                            &parser_clone, 
                            &storage_clone, 
                            &app_handle_clone,
                            &project_path_clone,
                        ).await {
                            eprintln!("File event handling failed: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("File watcher error: {}", e);
                    }
                }
            }
        });
        
        Ok(watch_id)
    }

    pub async fn stop_watching(&self, watch_id: &str) {
        let mut watchers = self.watchers.lock().unwrap();
        if let Some(_watcher) = watchers.remove(watch_id) {
            println!("Stopped watching: {}", watch_id);
        }
    }
}

async fn handle_file_event(
    event: Event,
    _parser: &Arc<Mutex<IncrementalParser>>,
    storage: &ReqSmithStorage,
    app_handle: &AppHandle,
    project_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match event.kind {
        EventKind::Modify(_) | EventKind::Create(_) => {
            for path in event.paths {
                if is_supported_source_file(&path) && path.starts_with(project_path) {
                    println!("Processing file change: {}", path.display());
                    
                    let content = match std::fs::read_to_string(&path) {
                        Ok(content) => content,
                        Err(_) => continue, // File might be deleted or inaccessible
                    };
                    
                    // Parse the file with appropriate parser
                    let new_endpoints = match parse_file_content(&content, &path) {
                        Ok(endpoints) => endpoints,
                        Err(e) => {
                            eprintln!("Failed to parse {}: {}", path.display(), e);
                            continue;
                        }
                    };
                    
                    // Get relative path for handler field
                    let rel_path = pathdiff::diff_paths(&path, project_path)
                        .unwrap_or(path.clone())
                        .to_string_lossy()
                        .to_string();
                    
                    // Set handler field to relative path for all endpoints
                    let endpoints_with_handler: Vec<Endpoint> = new_endpoints
                        .into_iter()
                        .map(|mut ep| {
                            ep.handler = rel_path.clone();
                            ep
                        })
                        .collect();
                    
                    // For now, just emit the updated endpoints from this file
                    // TODO: Implement proper incremental change detection
                    if !endpoints_with_handler.is_empty() {
                        // Convert to EndpointRecord for storage
                        let endpoint_records: Vec<EndpointRecord> = endpoints_with_handler
                            .iter()
                            .map(|ep| EndpointRecord::from_endpoint(ep.clone(), &path))
                            .collect();
                        
                        // Update storage (this will merge with existing endpoints from other files)
                        if let Err(e) = storage.save_endpoints(&endpoint_records) {
                            eprintln!("Failed to update storage: {}", e);
                        }
                        
                        // Emit real-time update to UI
                        if let Err(e) = app_handle.emit("endpoints-updated", &endpoints_with_handler) {
                            eprintln!("Failed to emit endpoint update: {}", e);
                        } else {
                            println!("Emitted {} endpoints from {}", endpoints_with_handler.len(), rel_path);
                        }
                    }
                }
            }
        }
        EventKind::Remove(_) => {
            // TODO: Handle file deletion by removing endpoints from that file
            println!("File removed: {:?}", event.paths);
        }
        _ => {
            // Ignore other events (access, metadata changes, etc.)
        }
    }
    Ok(())
}

fn is_supported_source_file(path: &Path) -> bool {
    // Skip hidden directories and common build/dependency directories
    if let Some(parent) = path.parent() {
        for component in parent.components() {
            if let Some(name) = component.as_os_str().to_str() {
                if name.starts_with('.') || matches!(name, "node_modules" | "target" | "dist" | "build" | "vendor" | "__pycache__") {
                    return false;
                }
            }
        }
    }
    
    detect_language(path).is_some()
}

fn parse_file_content(content: &str, path: &Path) -> anyhow::Result<Vec<Endpoint>> {
    let extension = path
        .extension()
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
