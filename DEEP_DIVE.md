# Apirena Technical Architecture

This document describes the technical implementation of Apirena, a code-aware API development environment.

## Table of Contents
- [Core Philosophy](#core-philosophy)
- [System Architecture](#system-architecture)  
- [File Watching Pipeline](#file-watching-pipeline)
- [Code Parsing Engine](#code-parsing-engine)
- [Git Integration](#git-integration)
- [Storage Strategy](#storage-strategy)
- [Real-Time UI Updates](#real-time-ui-updates)
- [AI Integration](#ai-integration)
- [Performance Considerations](#performance-considerations)

## Core Philosophy

Apirena operates on three principles:

1. **Code is Truth** - API structure is derived from parsing source files, not from separate specifications
2. **Zero Configuration** - Automatic detection of frameworks and patterns
3. **Local-First** - Everything runs on the developer's machine with no external dependencies

## System Architecture

```
┌─────────────────────────────────────────────┐
│            Tauri Application                │
├─────────────────────────────────────────────┤
│         Svelte 5 UI (Runes)                │
│  ┌─────────────┬────────────┬────────────┐ │
│  │ Endpoint    │   Test     │  History   │ │
│  │   List      │  Builder   │   Panel    │ │
│  └─────────────┴────────────┴────────────┘ │
├─────────────────────────────────────────────┤
│          WebSocket Event Stream             │
├─────────────────────────────────────────────┤
│             Rust Core Engine                │
│  ┌─────────────────────────────────────┐   │
│  │      File System Watcher             │   │
│  ├─────────────────────────────────────┤   │
│  │    Tree-sitter Parser Pool          │   │
│  ├─────────────────────────────────────┤   │
│  │        AI Analysis Layer            │   │
│  ├─────────────────────────────────────┤   │
│  │     Git State Tracker               │   │
│  ├─────────────────────────────────────┤   │
│  │   Minimal History Buffer            │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

## File Watching Pipeline

The file watcher is the heart of Apirena's real-time capabilities:

```rust
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    debouncer: Debouncer<PathBuf>,
    ignore_patterns: IgnoreFilter,
}

impl FileWatcher {
    pub fn start(&mut self, project_root: PathBuf) -> Result<()> {
        // Use native OS file watching (FSEvents/inotify/etc)
        let (tx, rx) = channel();
        
        let mut watcher = notify::recommended_watcher(move |event| {
            if let Ok(Event { kind: EventKind::Modify(_), paths, .. }) = event {
                for path in paths {
                    // Smart filtering at the source
                    if should_process(&path) {
                        tx.send(FileChange { path, timestamp: Instant::now() }).ok();
                    }
                }
            }
        })?;
        
        // Watch common API directories
        for dir in ["src", "app", "api", "routes", "controllers"] {
            let watch_path = project_root.join(dir);
            if watch_path.exists() {
                watcher.watch(&watch_path, RecursiveMode::Recursive)?;
            }
        }
        
        // Process with debouncing to handle rapid saves
        thread::spawn(move || {
            for change in rx {
                self.debouncer.submit(change.path, Duration::from_millis(50), |path| {
                    if let Err(e) = self.process_file(path) {
                        error!("Failed to process {}: {}", path.display(), e);
                    }
                });
            }
        });
        
        Ok(())
    }
}

fn should_process(path: &Path) -> bool {
    // Skip non-source files
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("js") | Some("ts") | Some("py") | Some("go") | 
        Some("rs") | Some("rb") | Some("java") | Some("php")
    ) && !path.components().any(|c| {
        matches!(c.as_os_str().to_str(), Some("node_modules") | 
                 Some("dist") | Some("build") | Some(".git"))
    })
}
```

## Code Parsing Engine

Tree-sitter provides incremental parsing across all supported languages:

```rust
pub struct ParserPool {
    parsers: HashMap<Language, Parser>,
    queries: HashMap<Language, EndpointQuery>,
    cache: Arc<RwLock<ParseCache>>,
}

impl ParserPool {
    pub async fn parse_file(&self, path: &Path, content: &str) -> Result<Vec<Endpoint>> {
        let lang = detect_language(path)?;
        let parser = self.parsers.get(&lang).ok_or("Unsupported language")?;
        
        // Check cache first
        let content_hash = hash_content(content);
        if let Some(cached) = self.cache.read().await.get(&(path.to_path_buf(), content_hash)) {
            return Ok(cached.clone());
        }
        
        // Parse with tree-sitter
        let tree = parser.parse(content, None)?;
        let query = &self.queries[&lang];
        
        let mut endpoints = Vec::new();
        let mut cursor = QueryCursor::new();
        
        for match_ in cursor.matches(&query.query, tree.root_node(), content.as_bytes()) {
            if let Some(endpoint) = self.extract_endpoint(&match_, content, lang) {
                // Find associated documentation
                if let Some(docs) = self.find_documentation(&tree, &endpoint.node, content) {
                    endpoint.raw_documentation = Some(docs);
                }
                endpoints.push(endpoint);
            }
        }
        
        // Cache results
        self.cache.write().await.insert((path.to_path_buf(), content_hash), endpoints.clone());
        
        Ok(endpoints)
    }
}

// Language-specific queries
const JAVASCRIPT_QUERY: &str = r#"
; Express/Fastify style
(call_expression
  function: (member_expression
    object: (identifier) @router
    property: (identifier) @method)
  arguments: (arguments
    (string) @path))

; Next.js API routes
(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @method
      value: (arrow_function))))
"#;

const PYTHON_QUERY: &str = r#"
; FastAPI/Flask decorators
(decorator
  (call
    function: (attribute
      object: (identifier) @app
      attribute: (identifier) @method)
    arguments: (argument_list
      (string) @path)))
"#;
```

## Git Integration

Track API changes across commits without polluting git history:

```rust
pub struct GitTracker {
    repo: Repository,
    current_head: Option<Oid>,
    endpoint_cache: LruCache<Oid, Vec<Endpoint>>,
}

impl GitTracker {
    pub fn watch_head(&mut self) -> Result<()> {
        let head_path = self.repo.path().join("HEAD");
        
        // Watch for branch switches
        let watcher = notify::recommended_watcher(move |_| {
            if let Ok(reference) = self.repo.head() {
                let new_head = reference.target().unwrap();
                
                if self.current_head != Some(new_head) {
                    self.handle_head_change(new_head);
                    self.current_head = Some(new_head);
                }
            }
        })?;
        
        watcher.watch(&head_path, RecursiveMode::NonRecursive)?;
        Ok(())
    }
    
    pub fn diff_branches(&self, base: &str, head: &str) -> Result<ApiDiff> {
        let base_commit = self.repo.revparse_single(base)?.id();
        let head_commit = self.repo.revparse_single(head)?.id();
        
        let base_endpoints = self.get_endpoints_at_commit(base_commit)?;
        let head_endpoints = self.get_endpoints_at_commit(head_commit)?;
        
        Ok(ApiDiff {
            added: head_endpoints.difference(&base_endpoints),
            removed: base_endpoints.difference(&head_endpoints),
            modified: self.find_modified_endpoints(&base_endpoints, &head_endpoints),
        })
    }
}
```

## Storage Strategy

Minimal storage focused on developer workflow:

```rust
pub struct Storage {
    // Recent test sessions for replay
    history: HistoryBuffer,
    
    // Git commit -> endpoint mappings
    snapshot_cache: SnapshotCache,
    
    // No analytics, no dashboards, no metrics
}

pub struct HistoryBuffer {
    sessions: VecDeque<TestSession>,
    max_size: usize, // Default: 1000 sessions
}

impl HistoryBuffer {
    pub fn add_session(&mut self, session: TestSession) {
        if self.sessions.len() >= self.max_size {
            self.sessions.pop_front();
        }
        self.sessions.push_back(session);
        
        // Optionally persist to disk for crash recovery
        if let Ok(mut file) = File::create(".apirena/history.json") {
            serde_json::to_writer(&mut file, &self.sessions).ok();
        }
    }
    
    pub fn find_recent(&self, endpoint: &str) -> Vec<&TestSession> {
        self.sessions
            .iter()
            .rev()
            .filter(|s| s.endpoint == endpoint)
            .take(10)
            .collect()
    }
}

// What we explicitly DON'T store:
// - Performance metrics
// - Error rates  
// - Usage analytics
// - Long-term history
```

## Real-Time UI Updates

WebSocket bridge for instant updates:

```rust
pub struct EventBridge {
    clients: Arc<Mutex<Vec<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
}

impl EventBridge {
    pub async fn broadcast(&self, event: ApiEvent) {
        let message = Message::text(serde_json::to_string(&event).unwrap());
        let clients = self.clients.lock().await;
        
        for client in clients.iter() {
            client.send(message.clone()).await.ok();
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ApiEvent {
    EndpointDiscovered { endpoint: Endpoint },
    EndpointUpdated { path: String, changes: Vec<Change> },
    EndpointRemoved { path: String },
    GitBranchChanged { branch: String },
    TestCompleted { session: TestSession },
}
```

Frontend state management with Svelte 5 runes:

```javascript
// stores/api.svelte.ts
class ApiStore {
    endpoints = $state([]);
    activeEndpoint = $state(null);
    recentTests = $state([]);
    
    constructor() {
        this.ws = new WebSocket('ws://localhost:40451');
        
        this.ws.onmessage = (e) => {
            const event = JSON.parse(e.data);
            
            switch (event.type) {
                case 'EndpointDiscovered':
                    this.endpoints = [...this.endpoints, event.endpoint];
                    break;
                    
                case 'EndpointUpdated':
                    const idx = this.endpoints.findIndex(e => e.path === event.path);
                    if (idx >= 0) {
                        this.endpoints[idx] = { ...this.endpoints[idx], ...event.changes };
                    }
                    break;
                    
                case 'TestCompleted':
                    this.recentTests = [event.session, ...this.recentTests.slice(0, 99)];
                    break;
            }
        };
    }
}

export const api = new ApiStore();
```

## AI Integration

Local-first AI with fallback options:

```rust
pub struct AiAnalyzer {
    local_model: Option<OllamaClient>,
    remote_model: Option<OpenAiClient>,
    cache: Arc<RwLock<HashMap<u64, Analysis>>>,
}

impl AiAnalyzer {
    pub async fn analyze_endpoint(&self, endpoint: &Endpoint, context: &Context) -> Result<Analysis> {
        // Cache key based on content
        let cache_key = calculate_hash(&(endpoint, context));
        
        // Check cache first
        if let Some(cached) = self.cache.read().await.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // Build focused prompt
        let prompt = format!(
            "Given this API endpoint:\n{}\n{}\n\nGenerate:\n\
            1. Expected request format\n\
            2. Common test cases\n\
            3. Edge cases to consider\n\
            Return as JSON.",
            endpoint.signature,
            endpoint.raw_documentation.as_deref().unwrap_or("")
        );
        
        // Try local first, fall back to remote
        let result = match &self.local_model {
            Some(local) => local.complete(&prompt).await
                .or_else(|_| self.remote_model.as_ref()
                    .ok_or("No AI model available")?
                    .complete(&prompt)),
            None => self.remote_model.as_ref()
                .ok_or("No AI model available")?
                .complete(&prompt)
        }?;
        
        let analysis = serde_json::from_str(&result)?;
        self.cache.write().await.insert(cache_key, analysis.clone());
        
        Ok(analysis)
    }
}
```

## Performance Considerations

### 1. Incremental Parsing

```rust
pub struct IncrementalParser {
    trees: HashMap<PathBuf, Tree>,
}

impl IncrementalParser {
    pub fn parse_edit(&mut self, path: &Path, edit: Edit, new_content: &str) -> Result<Tree> {
        if let Some(old_tree) = self.trees.get_mut(path) {
            // Apply edit to existing tree
            old_tree.edit(&InputEdit {
                start_byte: edit.start_byte,
                old_end_byte: edit.old_end_byte,
                new_end_byte: edit.new_end_byte,
                start_position: edit.start_position,
                old_end_position: edit.old_end_position,
                new_end_position: edit.new_end_position,
            });
            
            // Reparse with old tree as reference
            let new_tree = self.parser.parse(new_content, Some(old_tree))?;
            self.trees.insert(path.to_owned(), new_tree.clone());
            Ok(new_tree)
        } else {
            // First parse of this file
            self.parse_full(path, new_content)
        }
    }
}
```

### 2. Smart Caching

```rust
pub struct CacheStrategy {
    // In-memory LRU for hot paths
    memory: Arc<RwLock<LruCache<CacheKey, CachedResult>>>,
    
    // Disk cache for larger results (optional)
    disk: Option<DiskCache>,
}

impl CacheStrategy {
    pub async fn get_or_compute<F, T>(&self, key: CacheKey, compute: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
        T: Clone + Serialize + DeserializeOwned,
    {
        // Memory cache is always first
        if let Some(result) = self.memory.read().await.get(&key) {
            return Ok(result.clone());
        }
        
        // Disk cache for overflow
        if let Some(disk) = &self.disk {
            if let Some(result) = disk.get(&key).await? {
                // Promote to memory
                self.memory.write().await.put(key.clone(), result.clone());
                return Ok(result);
            }
        }
        
        // Compute and cache
        let result = compute()?;
        self.memory.write().await.put(key.clone(), result.clone());
        
        Ok(result)
    }
}
```

### 3. Parallel Processing

```rust
pub struct ParallelParser {
    thread_pool: ThreadPool,
}

impl ParallelParser {
    pub async fn parse_directory(&self, dir: &Path) -> Result<Vec<Endpoint>> {
        let files: Vec<_> = WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| should_process(e.path()))
            .collect();
        
        // Parse files in parallel
        let (tx, rx) = mpsc::channel();
        let chunk_size = (files.len() / self.thread_pool.max_count()).max(1);
        
        for chunk in files.chunks(chunk_size) {
            let tx = tx.clone();
            let chunk = chunk.to_vec();
            
            self.thread_pool.execute(move || {
                for entry in chunk {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(endpoints) = parse_file(entry.path(), &content) {
                            tx.send((entry.path().to_owned(), endpoints)).ok();
                        }
                    }
                }
            });
        }
        
        drop(tx);
        let mut all_endpoints = Vec::new();
        while let Ok((_, endpoints)) = rx.recv() {
            all_endpoints.extend(endpoints);
        }
        
        Ok(all_endpoints)
    }
}
```

## Summary

Apirena achieves real-time API understanding through:

1. **Native file watching** with intelligent debouncing
2. **Tree-sitter parsing** for instant code understanding
3. **Minimal storage** focused on developer workflow
4. **Local-first AI** for enhanced development experience
5. **Git integration** without repository pollution

The architecture prioritizes developer experience over analytics, making it the perfect companion during API development rather than another monitoring tool.
