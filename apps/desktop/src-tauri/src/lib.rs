use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use reqsmith_parser::{Endpoint, LanguageParser, detect_language};
use reqsmith_parser::languages::{javascript::JavaScriptParser, python::PythonParser, php::PhpParser};

mod storage;
use storage::{ReqSmithStorage, EndpointRecord};

fn find_benchmark_project() -> Option<String> {
    // Try to locate the repo root and pick a small express sample for demo
    if let Ok(mut dir) = std::env::current_dir() {
        for _ in 0..6 {
            let candidate = dir
                .join("libs/benchmarks/projects/small/express-api");
            if candidate.exists() {
                return Some(candidate.to_string_lossy().to_string());
            }
            if !dir.pop() {
                break;
            }
        }
    }
    None
}

#[tauri::command]
async fn select_project_folder() -> Result<Option<String>, String> {
    // Prefer a benchmark sample for reliable demo data
    if let Some(sample) = find_benchmark_project() {
        return Ok(Some(sample));
    }

    // Fallback: current working directory
    let test_path = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());
        
    Ok(Some(test_path))
}

fn should_visit(path: &Path) -> bool {
    if path.is_dir() {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            return !matches!(
                name,
                "node_modules" | ".git" | ".reqsmith" | "target" | "dist" | "build"
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

    // Ensure .reqsmith structure exists
    let storage = ReqSmithStorage::new(project_path.clone())
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
    let manifest_path = project_path.join(".reqsmith/endpoints/manifest.json");
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

    // Initial discovery and persist to disk, then emit results
    match discover_endpoints(path.clone()).await {
        Ok(updated_endpoints) => {
            if let Err(e) = app_handle.emit("endpoints-updated", &updated_endpoints) {
                eprintln!("Failed to emit endpoints update: {}", e);
            }
        }
        Err(err) => return Err(err),
    }

    Ok(path)
}

#[tauri::command]
async fn stop_watching(
    _watch_id: String,
) -> Result<(), String> {
    // No-op for now until a real watcher is wired up with cancellable tasks.
    Ok(())
}

#[tauri::command]
async fn send_request(
    endpoint: Endpoint, 
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: Option<String>
) -> Result<HttpResponse, String> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:3000{}", endpoint.path);
    
    // Build request based on method
    let mut request_builder = match endpoint.method {
        reqsmith_parser::HttpMethod::Get => client.get(&url),
        reqsmith_parser::HttpMethod::Post => client.post(&url),
        reqsmith_parser::HttpMethod::Put => client.put(&url),
        reqsmith_parser::HttpMethod::Delete => client.delete(&url),
        reqsmith_parser::HttpMethod::Patch => client.patch(&url),
        reqsmith_parser::HttpMethod::Options => client.request(reqwest::Method::OPTIONS, &url),
        reqsmith_parser::HttpMethod::Head => client.head(&url),
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
        .setup(|app| {
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Reqsmith")
                .inner_size(1100.0, 720.0);

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
            discover_endpoints,
            read_manifest,
            start_watching,
            stop_watching,
            send_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
