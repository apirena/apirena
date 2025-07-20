# Hallwatch Architecture

## Overview

Hallwatch is built as a modular, high-performance system for code-first API testing. The architecture follows these principles:

1. **Code-First Design** - Implementation is the source of truth
2. **Performance First** - Sub-10ms parsing, minimal memory usage
3. **Smart Caching** - AI runs once per endpoint, not per file change
4. **Local-First** - Privacy-preserving, no external dependencies required

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
│  │    AST Parser + Smart Caching       │   │
│  │       (hallwatch-parser)              │   │
│  ├─────────────────────────────────────┤   │
│  │  AI Enhancement Layer + Cache       │   │
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

**Responsibility**: AST-based endpoint discovery without annotations

**Key Components**:
- `LanguageParser` trait - Common interface for all languages
- `ParserPool` - Concurrent parsing with caching
- Language-specific parsers (JavaScript, Python, Go, etc.)
- Framework pattern detection

**Performance Targets**:
- < 10ms parse time per file
- Support for 20+ languages
- 95%+ accuracy on real-world code
- Zero configuration required

### hallwatch-ai

**Responsibility**: Intelligent parameter analysis with smart caching

**Key Components**:
- `EndpointAnalyzer` - Context-aware parameter suggestions
- `AICache` - Signature-based result caching
- `ModelProvider` - Local/cloud AI abstraction
- `UserStateManager` - Preserve playground values

**Performance Targets**:
- < 100ms cached responses
- < 2s initial analysis
- 90%+ cache hit rate
- Graceful degradation without AI

## Data Flow

```
File Change → FileWatcher → ParserPool → LanguageParser → Endpoints
     ↓              ↓            ↓            ↓             ↓
  EventBus → CLI/Desktop → AI Analysis → Cached Results → UI Update
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
    AIAnalysisComplete { endpoint: Endpoint, analysis: Analysis },
}
```

### Smart Caching Strategy

```rust
// Endpoint signature for caching
pub fn endpoint_signature(endpoint: &Endpoint) -> String {
    format!("{}-{}-{}", 
        endpoint.method, 
        endpoint.path, 
        endpoint.middleware_hash()
    )
}

// Cache structure
pub struct AICache {
    entries: HashMap<String, CachedAnalysis>,
    ttl: Duration,
}
```

## Error Handling

- All fallible operations return `Result<T, Error>`
- Errors are propagated up with context using `anyhow`
- Critical errors are logged, non-critical are handled gracefully
- Parsing errors don't crash the application
- AI failures gracefully degrade to basic functionality

## Security Considerations

- No network requests by default (local-first)
- File system access is limited to watched directories
- AI features require explicit user consent
- No sensitive data is logged or transmitted
- Local AI models preferred for privacy

## Future Extensions

- **Language Server Protocol**: IDE integration
- **Plugin System**: Third-party parser plugins  
- **Team Collaboration**: Shared playground sessions
- **Advanced Testing**: Load testing, contract testing
