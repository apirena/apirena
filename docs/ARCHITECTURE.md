# Hallwatch Architecture

## Overview

Hallwatch is built as a modular, high-performance system for real-time API discovery and testing. The architecture follows these principles:

1. **Modular Design** - Each library has a single responsibility
2. **Performance First** - Sub-10ms parsing, minimal memory usage
3. **Language Agnostic** - Extensible parser system
4. **Local-First** - No external dependencies required

## System Architecture

```
┌─────────────────────────────────────────────┐
│            Desktop Application              │
│         (Tauri + Svelte 5)                 │
├─────────────────────────────────────────────┤
│          CLI Application                    │
│            (Rust)                          │
├─────────────────────────────────────────────┤
│             Core Libraries                  │
│  ┌─────────────────────────────────────┐   │
│  │      File System Watcher             │   │
│  │        (hallwatch-core)               │   │
│  ├─────────────────────────────────────┤   │
│  │    Tree-sitter Parser Pool          │   │
│  │       (hallwatch-parser)              │   │
│  ├─────────────────────────────────────┤   │
│  │        Git Integration              │   │
│  │        (hallwatch-git)                │   │
│  ├─────────────────────────────────────┤   │
│  │     AI Analysis Layer               │   │
│  │        (hallwatch-ai)                 │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

## Core Libraries

### hallwatch-core

**Responsibility**: File system monitoring and event handling

**Key Components**:
- `FileWatcher` - Cross-platform file system events
- `EventBus` - Internal message passing
- `IgnoreFilter` - .gitignore and custom pattern support

**Performance Targets**:
- < 5ms event processing
- < 50MB memory usage
- Support for 10,000+ files

### hallwatch-parser

**Responsibility**: Source code parsing and endpoint extraction

**Key Components**:
- `LanguageParser` trait - Common interface for all languages
- `ParserPool` - Concurrent parsing with caching
- Language-specific parsers (JavaScript, Python, Go, etc.)

**Performance Targets**:
- < 10ms parse time per file
- Support for 20+ languages
- 95%+ accuracy on real-world code

### hallwatch-git (Future)

**Responsibility**: Git integration for API change tracking

**Key Components**:
- `GitWatcher` - Monitor branch changes
- `DiffGenerator` - Compare endpoints between commits
- `ChangeAnalyzer` - Semantic API change detection

### hallwatch-ai (Future)

**Responsibility**: AI-powered analysis and test generation

**Key Components**:
- `ModelClient` - Abstract interface for AI models
- `TestGenerator` - Create test cases from endpoints
- `DocumentationExtractor` - Parse natural language docs

## Data Flow

```
File Change → FileWatcher → ParserPool → LanguageParser → Endpoints
     ↓              ↓            ↓            ↓             ↓
  EventBus → CLI/Desktop → UI Update → User Action → Test Request
```

## Technology Stack

- **Language**: Rust (performance, safety, concurrency)
- **Parsing**: Tree-sitter (incremental, error-tolerant)
- **Desktop**: Tauri 2 (native performance, web technologies)
- **UI Framework**: Svelte 5 (reactive, minimal runtime)
- **Monorepo**: NX (efficient builds, dependency management)
- **AI**: Ollama + OpenAI (local-first with cloud fallback)

## Design Patterns

### Plugin Architecture

Each language parser is a plugin that implements the `LanguageParser` trait:

```rust
pub trait LanguageParser: Send + Sync {
    fn parse(&self, content: &str) -> Result<Vec<Endpoint>>;
    fn supports_extension(&self, extension: &str) -> bool;
    fn framework_hints(&self) -> Vec<&'static str>;
}
```

### Event-Driven Updates

All communication between components uses an event system:

```rust
pub enum ApiEvent {
    FileChanged { path: PathBuf },
    EndpointsDiscovered { path: PathBuf, endpoints: Vec<Endpoint> },
    GitBranchChanged { from: String, to: String },
}
```

### Incremental Processing

- Files are only re-parsed when changed
- Tree-sitter provides incremental parsing
- UI updates are batched for performance

## Error Handling

- All fallible operations return `Result<T, Error>`
- Errors are propagated up with context using `anyhow`
- Critical errors are logged, non-critical are handled gracefully
- Parsing errors don't crash the application

## Testing Strategy

- **Unit Tests**: Every public function
- **Integration Tests**: End-to-end workflows
- **Property Tests**: Parser accuracy with random inputs
- **Performance Tests**: Benchmark parsing speed and memory usage

## Security Considerations

- No network requests by default (local-first)
- File system access is limited to watched directories
- AI features require explicit user consent
- No sensitive data is logged or transmitted

## Future Extensions

- **Language Server Protocol**: IDE integration
- **Plugin System**: Third-party parser plugins
- **Distributed Mode**: Team collaboration features
- **API Mocking**: Generate mock servers from discovered APIs
