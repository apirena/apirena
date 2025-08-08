use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use reqsmith_parser::{Endpoint, languages::javascript::JavaScriptParser, LanguageParser};

#[tauri::command]
async fn select_project_folder() -> Result<Option<String>, String> {
    // For testing, use a hardcoded path to a test project
    // In production, this would open a native folder picker dialog
    let test_path = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());
        
    Ok(Some(test_path))
}

#[tauri::command]
async fn discover_endpoints(path: String) -> Result<Vec<Endpoint>, String> {
    let project_path = PathBuf::from(&path);
    
    if !project_path.exists() {
        return Err("Project path does not exist".to_string());
    }

    let parser = JavaScriptParser;
    let mut endpoints = Vec::new();

    // Walk through the project directory and parse files
    if let Ok(entries) = std::fs::read_dir(&project_path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let file_path = entry.path();
                    if let Some(ext) = file_path.extension() {
                        if ext == "js" || ext == "ts" {
                            if let Ok(content) = std::fs::read_to_string(&file_path) {
                                if let Ok(file_endpoints) = parser.parse(&content) {
                                    endpoints.extend(file_endpoints);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(endpoints)
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

    // For now, simply perform an initial discovery and emit results.
    if let Ok(updated_endpoints) = discover_endpoints(path.clone()).await {
        if let Err(e) = app_handle.emit("endpoints-updated", &updated_endpoints) {
            eprintln!("Failed to emit endpoints update: {}", e);
        }
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
        .invoke_handler(tauri::generate_handler![
            select_project_folder,
            discover_endpoints,
            start_watching,
            stop_watching,
            send_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
