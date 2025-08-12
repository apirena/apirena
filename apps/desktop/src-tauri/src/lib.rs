use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tauri::{AppHandle, Emitter, TitleBarStyle, WebviewUrl, WebviewWindowBuilder, Manager};
use pinpath_parser::{Endpoint, LanguageParser, detect_language};
use pinpath_parser::languages::{javascript::JavaScriptParser, python::PythonParser, php::PhpParser};

mod storage;
mod watcher;

use storage::{PinPathStorage, EndpointRecord};
use watcher::WatcherRegistry;

// Global watcher registry
static WATCHER_REGISTRY: OnceLock<WatcherRegistry> = OnceLock::new();

// Config management functions for persistent last directory
fn get_app_config_dir(app_handle: &AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get app config directory: {}", e))
}

fn get_last_selected_directory(app_handle: &AppHandle) -> Option<PathBuf> {
    let config_dir = get_app_config_dir(app_handle).ok()?;
    let config_file = config_dir.join("last_directory.txt");
    
    if config_file.exists() {
        if let Ok(path_str) = std::fs::read_to_string(&config_file) {
            let path = PathBuf::from(path_str.trim());
            if path.exists() && path.is_dir() {
                return Some(path);
            }
        }
    }
    None
}

fn save_last_selected_directory(app_handle: &AppHandle, dir: &Path) -> Result<(), String> {
    let config_dir = get_app_config_dir(app_handle)?;
    
    // Ensure config directory exists
    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        return Err(format!("Failed to create config directory: {}", e));
    }
    
    let config_file = config_dir.join("last_directory.txt");
    std::fs::write(&config_file, dir.to_string_lossy().as_bytes())
        .map_err(|e| format!("Failed to write last directory config: {}", e))
}


#[tauri::command]
async fn select_project_folder(app_handle: AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    // Get last selected directory from app config, or fall back to reasonable defaults
    let start_dir = get_last_selected_directory(&app_handle)
        .or_else(|| find_workspace_root())
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("/"))
        );
    
    println!("Starting folder picker from: {}", start_dir.display());
    
    // Native folder picker dialog with starting directory
    let folder = app_handle
        .dialog()
        .file()
        .set_title("Select Project Folder to Watch")
        .set_directory(&start_dir)
        .blocking_pick_folder();
    
    match folder {
        Some(path) => {
            let path_str = path.to_string();
            println!("Selected folder: {}", path_str);
            
            // Save the parent directory for next time
            if let Some(parent) = PathBuf::from(&path_str).parent().map(|p| p.to_path_buf()) {
                if let Err(e) = save_last_selected_directory(&app_handle, &parent) {
                    eprintln!("Failed to save last selected directory: {}", e);
                }
            }
            
            Ok(Some(path_str))
        },
        None => {
            println!("User cancelled folder selection");
            // No fallback - user must select a project
            Ok(None)
        }
    }
}

#[tauri::command]
async fn select_project_folder_from(app_handle: AppHandle, start_path: String) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let start_dir = std::path::PathBuf::from(&start_path);
    
    if !start_dir.exists() {
        return Err(format!("Start path does not exist: {}", start_path));
    }
    
    println!("Starting folder picker from custom path: {}", start_dir.display());
    
    // Native folder picker dialog with custom starting point
    let folder = app_handle
        .dialog()
        .file()
        .set_title("Select Project Folder to Watch")
        .set_directory(&start_dir)
        .blocking_pick_folder();
    
    match folder {
        Some(path) => {
            let path_str = path.to_string();
            println!("Selected folder: {}", path_str);
            
            // Save the parent directory for next time
            if let Some(parent) = PathBuf::from(&path_str).parent().map(|p| p.to_path_buf()) {
                if let Err(e) = save_last_selected_directory(&app_handle, &parent) {
                    eprintln!("Failed to save last selected directory: {}", e);
                }
            }
            
            Ok(Some(path_str))
        },
        None => {
            println!("User cancelled folder selection");
            Ok(None)
        }
    }
}



fn find_workspace_root() -> Option<std::path::PathBuf> {
    // Try to locate the workspace root
    if let Ok(mut dir) = std::env::current_dir() {
        for _ in 0..8 {  // Increased search depth
            if dir.join("nx.json").exists() && dir.join("pnpm-workspace.yaml").exists() {
                return Some(dir);
            }
            if !dir.pop() {
                break;
            }
        }
    }
    None
}

fn should_visit(path: &Path) -> bool {
    if path.is_dir() {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            return !matches!(
                name,
                "node_modules" | ".git" | ".pinpath" | "target" | "dist" | "build"
            );
        }
    }
    true
}

fn is_supported_source_file(path: &Path) -> bool {
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

#[tauri::command]
async fn discover_endpoints(path: String) -> Result<Vec<Endpoint>, String> {
    let project_path = PathBuf::from(&path);
    
    if !project_path.exists() {
        return Err("Project path does not exist".to_string());
    }

    // Ensure .pinpath structure exists
    let storage = PinPathStorage::new(project_path.clone())
        .map_err(|e| format!("Storage init failed: {}", e))?;

    let mut endpoints = Vec::new();
    let mut endpoint_records: Vec<EndpointRecord> = Vec::new();

    // Walk through the project directory recursively and parse files
    for entry in walkdir::WalkDir::new(&project_path)
        .into_iter()
        .filter_entry(|e| should_visit(e.path()))
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path().to_path_buf();
        if entry.file_type().is_file() && is_supported_source_file(&file_path) {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                match parse_file_content(&content, &file_path) {
                    Ok(file_endpoints) => {
                        // Use relative file path as handler for UI uniqueness
                        let rel_file = pathdiff::diff_paths(&file_path, &project_path)
                            .unwrap_or(file_path.clone())
                            .to_string_lossy()
                            .to_string();

                        for mut ep in file_endpoints {
                            ep.handler = rel_file.clone();
                            endpoints.push(ep.clone());
                            endpoint_records.push(EndpointRecord::from_endpoint(ep, &file_path));
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to parse {}: {}", file_path.display(), err);
                    }
                }
            }
        }
    }

    // Persist to filesystem manifest for transparency
    if let Err(e) = storage.save_endpoints(&endpoint_records) {
        eprintln!("Failed to save endpoints manifest: {}", e);
    }

    Ok(endpoints)
}

#[tauri::command]
async fn read_manifest(path: String) -> Result<Option<String>, String> {
    let project_path = PathBuf::from(&path);
    let manifest_path = project_path.join(".pinpath/endpoints/manifest.json");
    if !manifest_path.exists() {
        return Ok(None);
    }
    std::fs::read_to_string(&manifest_path)
        .map(Some)
        .map_err(|e| format!("Failed to read manifest: {}", e))
}

#[tauri::command] 
async fn start_watching(
    path: String,
    app_handle: AppHandle,
) -> Result<String, String> {
    let project_path = PathBuf::from(&path);
    
    if !project_path.exists() {
        return Err("Project path does not exist".to_string());
    }

    // Get or create the global watcher registry
    let registry = WATCHER_REGISTRY.get_or_init(|| WatcherRegistry::new());
    
    // Start watching the project directory
    let watch_id = registry.start_watching(project_path.clone(), app_handle.clone()).await?;
    
    // Also do initial discovery and emit results
    match discover_endpoints(path.clone()).await {
        Ok(updated_endpoints) => {
            if let Err(e) = app_handle.emit("endpoints-updated", &updated_endpoints) {
                eprintln!("Failed to emit initial endpoints: {}", e);
            }
        }
        Err(err) => {
            // Stop watching if initial discovery fails
            registry.stop_watching(&watch_id).await;
            return Err(err);
        }
    }

    Ok(watch_id)
}

#[tauri::command]
async fn stop_watching(
    watch_id: String,
) -> Result<(), String> {
    if let Some(registry) = WATCHER_REGISTRY.get() {
        registry.stop_watching(&watch_id).await;
    }
    Ok(())
}

#[tauri::command]
async fn copy_to_clipboard(text: String) -> Result<(), String> {
    // Simple fallback - just print to console for now
    // In a real implementation, you'd use the system clipboard API
    println!("Copying to clipboard: {}", text);
    Ok(())
}

#[tauri::command]
async fn send_request(
    endpoint: Endpoint, 
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: Option<String>,
    base_url: Option<String>
) -> Result<HttpResponse, String> {
    let client = reqwest::Client::new();
    let base = base_url.unwrap_or_else(|| "http://localhost:3000".to_string());
    let url = format!("{}{}", base, endpoint.path);
    
    // Build request based on method
    let mut request_builder = match endpoint.method {
        pinpath_parser::HttpMethod::Get => client.get(&url),
        pinpath_parser::HttpMethod::Post => client.post(&url),
        pinpath_parser::HttpMethod::Put => client.put(&url),
        pinpath_parser::HttpMethod::Delete => client.delete(&url),
        pinpath_parser::HttpMethod::Patch => client.patch(&url),
        pinpath_parser::HttpMethod::Options => client.request(reqwest::Method::OPTIONS, &url),
        pinpath_parser::HttpMethod::Head => client.head(&url),
    };

    // Add query parameters for GET requests
    if !params.is_empty() {
        let params_vec: Vec<(String, String)> = params.into_iter().collect();
        request_builder = request_builder.query(&params_vec);
    }

    // Add headers
    for (key, value) in headers {
        request_builder = request_builder.header(&key, &value);
    }

    // Add body for non-GET requests
    if let Some(body_content) = body {
        request_builder = request_builder.body(body_content);
    }

    // Send request
    let response = request_builder.send().await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status();
    let headers_map: HashMap<String, String> = response.headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    let body = response.text().await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    Ok(HttpResponse {
        status: status.as_u16(),
        headers: headers_map,
        body,
        duration_ms: 0, // TODO: Implement timing
    })
}

#[derive(serde::Serialize)]
struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
    duration_ms: u64,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("")
                .inner_size(1200.0, 800.0);

            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window = win_builder.build().expect("failed to build main window");

            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSColor, NSWindow};
                use cocoa::base::{id, nil};

                let ns_window = window.ns_window().unwrap() as id;
                unsafe {
                    let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                        nil,
                        25.0 / 255.0,
                        33.0 / 255.0,
                        45.0 / 255.0,
                        1.0,
                    );
                    ns_window.setBackgroundColor_(bg_color);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            select_project_folder,
            select_project_folder_from,
            discover_endpoints,
            read_manifest,
            start_watching,
            stop_watching,
            send_request,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
