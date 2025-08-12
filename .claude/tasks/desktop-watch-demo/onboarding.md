````markdown
# Desktop Watch Demo — MVP Production Implementation

## Project Vision
ReqSmith is a **filesystem-native** API endpoint discovery tool that watches codebases in real-time. All state lives transparently on disk - no hidden databases, no cloud dependencies. The true MVP is **folder selection → endpoint discovery → live watching** with state persistence.

## Core MVP Flow
1. **Select Project Folder**: User picks any directory containing code
2. **Parse & Discover**: Recursively find and parse API endpoints from source files
3. **Display Endpoints**: Show discovered endpoints in organized, filterable UI
4. **Auto-Watch**: Monitor file changes and update endpoint list in real-time
5. **Persist State**: All discoveries cached in `.reqsmith/` for instant reload

## Success Criteria (MVP)
- **Folder Selection**: Native file picker that works with any project
- **Multi-Language Parsing**: Support JavaScript/TypeScript, Python, PHP endpoints
- **Real-Time Updates**: File changes reflect in UI within 100ms
- **Filesystem State**: All data stored transparently in `.reqsmith/` directories
- **Performance**: Initial scan <2s, incremental updates <50ms

## Architecture Overview - MVP Focus

### 1. Folder Selection & Project Discovery

#### Current Implementation (`select_project_folder`)
````rust
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
            // Save the parent directory for next time
            if let Some(parent) = PathBuf::from(&path_str).parent().map(|p| p.to_path_buf()) {
                save_last_selected_directory(&app_handle, &parent)?;
            }
            Ok(Some(path_str))
        },
        None => Ok(None), // No fallback - user must select a project
    }
}
````

#### MVP Enhancement - Now Implemented ✅
````rust
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
````

#### File System Structure (Unchanged)
````
selected-project/
├── .reqsmith/                  # Created automatically on first scan
│   ├── config.json            # Project settings (committable)
│   ├── endpoints/             # Discovered endpoints (optionally committable)
│   │   ├── manifest.json      # Master index with statistics
│   │   └── by-file/           # Per-file endpoint caches
│   │       ├── src-routes-api.json
│   │       └── handlers-users.json
│   ├── cache/                 # Performance cache (gitignored)
│   │   ├── ast/               # Parsed AST cache with content hashes
│   │   ├── watch-state.json   # Active file watcher state
│   │   └── file-hashes.json   # Content change detection
│   └── .gitignore             # Auto-generated: cache/, *.log
└── src/                       # User's actual code
    ├── routes/
    ├── handlers/
    └── api/
````

### 2. Endpoint Discovery & Parsing Engine

#### Current Implementation (`discover_endpoints`)
- ✅ Recursive directory walking with exclusions
- ✅ Multi-language parser dispatch (JS/TS, Python, PHP)
- ✅ Endpoint extraction and filesystem persistence
- ❌ Missing file change detection optimization
- ❌ No incremental parsing with AST cache

#### MVP Enhancement: Integrate IncrementalParser
````rust
#[tauri::command]
async fn discover_endpoints(path: String) -> Result<Vec<Endpoint>, String> {
    let project_path = PathBuf::from(&path);
    let storage = ReqSmithStorage::new(project_path.clone())?;
    
    // Load previous state for incremental parsing
    let previous_state = storage.load_parser_state()
        .unwrap_or_else(|_| EndpointState::default());
    
    let mut parser = IncrementalParser::with_state(previous_state);
    let mut all_endpoints = Vec::new();
    
    // Scan files and detect changes
    for entry in walkdir::WalkDir::new(&project_path)
        .into_iter()
        .filter_entry(|e| should_visit(e.path()))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() && is_supported_source_file(entry.path()) {
            let file_path = entry.path();
            let content = std::fs::read_to_string(file_path)?;
            
            // Check if file changed using content hash
            if parser.file_needs_reparsing(file_path, &content) {
                let endpoints = parser.parse_file(file_path, &content)?;
                all_endpoints.extend(endpoints);
            } else {
                // Use cached results
                let cached = parser.get_cached_endpoints(file_path);
                all_endpoints.extend(cached);
            }
        }
    }
    
    // Persist updated state and manifest
    storage.save_parser_state(parser.get_state())?;
    storage.save_endpoints_manifest(&all_endpoints)?;
    
    Ok(all_endpoints)
}
````

### 3. Real-Time File Watching

#### Current Implementation (`start_watching`)
- ✅ Basic project path validation
- ✅ Initial discovery and UI event emission
- ❌ No actual file system watching
- ❌ No incremental change processing

#### MVP Enhancement: Native File Watcher
````rust
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

struct ProjectWatcher {
    watcher: RecommendedWatcher,
    incremental_parser: Arc<Mutex<IncrementalParser>>,
    storage: ReqSmithStorage,
}

#[tauri::command]
async fn start_watching(
    path: String,
    app_handle: AppHandle,
) -> Result<String, String> {
    let project_path = PathBuf::from(&path);
    let (tx, rx) = mpsc::channel();
    
    // Create file system watcher
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(|e| format!("Failed to create watcher: {}", e))?;
    
    watcher.watch(&project_path, RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {}", e))?;
    
    // Load parser state
    let storage = ReqSmithStorage::new(project_path.clone())?;
    let previous_state = storage.load_parser_state().unwrap_or_default();
    let parser = Arc::new(Mutex::new(IncrementalParser::with_state(previous_state)));
    
    // Spawn background task to handle file events
    let parser_clone = parser.clone();
    let storage_clone = storage.clone();
    let app_handle_clone = app_handle.clone();
    
    tokio::spawn(async move {
        while let Ok(event) = rx.recv() {
            if let Err(e) = handle_file_event(
                event, 
                &parser_clone, 
                &storage_clone, 
                &app_handle_clone
            ).await {
                eprintln!("File event handling failed: {}", e);
            }
        }
    });
    
    // Initial discovery
    let endpoints = discover_endpoints(path.clone()).await?;
    app_handle.emit("endpoints-updated", &endpoints)?;
    
    Ok(format!("watching:{}", path))
}

async fn handle_file_event(
    event: Event,
    parser: &Arc<Mutex<IncrementalParser>>,
    storage: &ReqSmithStorage,
    app_handle: &AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    match event.kind {
        EventKind::Modify(_) | EventKind::Create(_) => {
            for path in event.paths {
                if is_supported_source_file(&path) {
                    let content = std::fs::read_to_string(&path)?;
                    
                    // Process change incrementally
                    let mut parser = parser.lock().unwrap();
                    let change_event = reqsmith_diff::ChangeEvent::from_file_change(&path, &content);
                    let changes = parser.parse_changes(change_event).await?;
                    
                    if changes.has_changes() {
                        // Persist updated state
                        storage.save_parser_state(parser.get_state())?;
                        
                        // Emit real-time updates to UI
                        if !changes.added.is_empty() {
                            app_handle.emit("endpoints-added", &changes.added)?;
                        }
                        if !changes.removed.is_empty() {
                            app_handle.emit("endpoints-removed", &changes.removed)?;
                        }
                        if !changes.modified.is_empty() {
                            app_handle.emit("endpoints-modified", &changes.modified)?;
                        }
                        
                        // Emit complete updated list
                        let all_endpoints: Vec<Endpoint> = parser.get_all_endpoints()
                            .into_iter().cloned().collect();
                        app_handle.emit("endpoints-updated", &all_endpoints)?;
                    }
                }
            }
        }
        _ => {} // Ignore other events
    }
    Ok(())
}
````

### 4. UI Integration & State Management

#### Frontend Store Enhancement
````typescript
// apps/desktop/src/lib/stores/endpoints.svelte.ts
class EndpointStore {
  // ... existing code ...
  
  private async setupEventListeners() {
    if (!isTauri()) return;

    // Listen for granular change events
    await listen<Endpoint[]>('endpoints-added', (event) => {
      this.projectState.endpoints.push(...event.payload);
      this.notifyEndpointChange('added', event.payload);
    });

    await listen<Endpoint[]>('endpoints-removed', (event) => {
      const removedPaths = new Set(event.payload.map(e => `${e.method}:${e.path}`));
      this.projectState.endpoints = this.projectState.endpoints.filter(
        e => !removedPaths.has(`${e.method}:${e.path}`)
      );
      this.notifyEndpointChange('removed', event.payload);
    });

    await listen<EndpointChange[]>('endpoints-modified', (event) => {
      for (const change of event.payload) {
        const index = this.projectState.endpoints.findIndex(
          e => e.path === change.old.path && e.method === change.old.method
        );
        if (index >= 0) {
          this.projectState.endpoints[index] = change.new;
        }
      }
      this.notifyEndpointChange('modified', event.payload.map(c => c.new));
    });

    // Full refresh fallback
    await listen<Endpoint[]>('endpoints-updated', (event) => {
      this.projectState.endpoints = event.payload.map(sanitizeEndpoint);
    });
  }
  
  private notifyEndpointChange(type: 'added' | 'removed' | 'modified', endpoints: Endpoint[]) {
    // Optional: Toast notifications for live changes
    console.log(`${type}: ${endpoints.length} endpoints`);
  }
}
````

## Implementation Plan - MVP Focus

### Phase 1: Native Folder Selection (2 hours) ✅ COMPLETED
1. **Add Tauri Dialog Plugin** ✅
   ````bash
   cd apps/desktop/src-tauri
   cargo add tauri-plugin-dialog
   ````
2. **Add Tauri OS Plugin for Config Storage** ✅
   ````bash
   cd apps/desktop/src-tauri
   cargo add tauri-plugin-os
   ````
3. **Implement Native Folder Picker** ✅ - Replaced hardcoded benchmark fallback
4. **Add Persistent Directory Memory** ✅ - Remembers last selected directory for next selection
5. **Remove Default Project Fallback** ✅ - User must explicitly select a project to continue
6. **Simplify UI** ✅ - Removed benchmark shortcuts, simplified to just "Select" and "Change"
7. **Test Cross-Platform** ⚠️ - Folder picker works, needs testing on Windows/Linux
8. **Handle Edge Cases** ⚠️ - Basic error handling in place, could use more robust validation

### Phase 2: Incremental Parsing Integration (3 hours)
1. **Wire IncrementalParser** - Replace basic parsing with cached/incremental approach
2. **AST Caching Layer** - Implement content-hash-based parse result caching
3. **File Change Detection** - Only reparse files with actual content changes
4. **Performance Benchmarks** - Measure initial scan time vs incremental updates

### Phase 3: Real-Time File Watching (4 hours)
1. **Add File System Watcher**
   ````bash
   cd apps/desktop/src-tauri  
   cargo add notify
   ````
2. **Implement Background Event Processing** - Handle file modify/create/delete events
3. **Incremental Change Processing** - Use reqsmith_diff for granular updates
4. **UI Event Emission** - Real-time updates to frontend via Tauri events

### Phase 4: Enhanced UI Integration (2 hours)
1. **Granular Event Handling** - Listen for `endpoints-added`, `endpoints-removed`, `endpoints-modified`
2. **Live Update Indicators** - Visual feedback when endpoints change in real-time
3. **Error Handling** - Graceful handling of parse errors, file permission issues
4. **Performance Monitoring** - Show parsing times, cache hit rates in UI

### Phase 5: State Persistence & Recovery (1 hour)
1. **Session Restoration** - Load previous parsing state on app restart
2. **Cache Management** - Implement size limits and cleanup for AST cache
3. **Config Validation** - Ensure `.reqsmith/config.json` has sensible defaults
4. **Git Integration** - Proper `.gitignore` handling for cache vs committable state

## Testing Strategy - MVP Validation
````bash
# Test folder selection
1. Open app → Select various project types (Node.js, Python, PHP mixed)
2. Verify .reqsmith/ directory creation
3. Check endpoints discovered and persisted to manifest.json

# Test incremental parsing
1. Edit a source file with endpoints
2. Verify UI updates within 100ms
3. Check that unchanged files aren't reparsed (cache hit)

# Test file watching
1. Add new endpoint file while app is running  
2. Delete endpoint file
3. Modify existing endpoint (path, method, parameters)
4. Verify all changes reflected in real-time

# Performance validation
time cat .reqsmith/cache/ast/index.json   # Check cache usage
du -h .reqsmith/                          # Monitor storage size  
tail -f .reqsmith/watch-state.json        # Monitor watcher activity
````

## Success Metrics - MVP Definition of Done
- [x] `.reqsmith/` directory created automatically ✅
- [x] Endpoints persisted to filesystem manifest ✅  
- [x] UI loads endpoints from manifest.json ✅
- [x] **Native folder picker replaces hardcoded paths** ✅
- [x] **Project reselection functionality** ✅
- [x] **Persistent directory memory** ✅ - Folder picker starts from last selected directory
- [x] **No default project fallback** ✅ - User must explicitly select a project
- [x] **Simplified selection UI** ✅ - Clean select/change project interface
- [ ] **File changes update UI within 100ms** ⏳
- [ ] **AST cache reduces reparse time by >90%** ⏳
- [ ] **Multi-language parsing works** (JS/TS ✅, Python ✅, PHP ✅)
- [ ] **Session restoration loads previous state** ⏳
- [ ] **Handles 100+ endpoint projects smoothly** ⏳

## Next Immediate Steps
1. **Add tauri-plugin-dialog** for native folder selection
2. **Replace `select_project_folder` with real file picker**
3. **Integrate `IncrementalParser` in `discover_endpoints`**  
4. **Add `notify` crate and implement real-time file watching**
5. **Test with realistic project directories (not just benchmark samples)**

The true MVP is getting users from "I have a codebase" to "I can see my endpoints updating live" with minimal friction and maximum transparency through filesystem-native state management.
````
