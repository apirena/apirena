# Claude Development Instructions for Reqsmith

## Project Overview

Reqsmith is a **code-first API playground** that eliminates spec drift by discovering endpoints directly from source code using AST parsing, enhanced with AI for intelligent parameter suggestions.

**Core Value**: Your code IS the spec - no annotations, no manual configuration, just real-time API discovery and testing.

## Architecture

```
- libs/core/        # File watching and event system (Rust)
- libs/parser/      # AST-based endpoint discovery (Rust)  
- libs/ai/          # Smart parameter analysis with caching (Rust)
- apps/cli/         # Command-line interface (Rust)
- apps/desktop/     # Tauri playground UI (Svelte 5 + Rust)
```

## Key Principles

1. **Code is Truth** - Implementation is the specification
2. **Zero Configuration** - Works with vanilla code, no decorators needed
3. **Smart Caching** - AI analyzes once per endpoint signature, caches forever
4. **Progressive Enhancement** - AI enhances but never blocks functionality
5. **User State Wins** - Playground preserves user values over AI suggestions

## Development Guidelines

### Working on Parser (`libs/parser/`)

Focus on pattern recognition without requiring annotations:

```rust
// Extract endpoints from vanilla framework code
impl LanguageParser for JavaScriptParser {
    fn parse(&self, content: &str) -> Result<Vec<Endpoint>> {
        // Support: Express, Fastify, Koa, Next.js, etc.
        // No decorators required
    }
}
```

**Performance Requirements:**
- <10ms parsing for typical files
- Support incremental parsing
- Graceful error recovery
- 95%+ accuracy on real-world code

### Working on AI Layer (`libs/ai/`)

AI enhances discovery with intelligent caching:

```rust
pub struct EndpointAnalyzer {
    cache: AICache,
    providers: Vec<Box<dyn AIProvider>>,
}

// Cache by signature: hash(method + path + params + middleware)
// Invalidate only when signature changes
// Preserve user overrides always
```

**Caching Strategy:**
- Signature-based caching (not per-file)
- TTL = Infinity until endpoint signature changes
- Local-first AI (Ollama preferred)
- Graceful degradation without AI

### Working on UI (`apps/desktop/`)

Playground state management with Svelte 5:

```typescript
// Three-layer state system
class PlaygroundStore {
  aiSuggestions = $state<Suggestions>({}); // From AI analysis
  userValues = $state<UserValues>({}); // User's saved inputs
  currentSession = $state<SessionData>({}); // Active playground state
  
  // Merge priority: AI < User < Current Session
  get finalValues() {
    return $derived({
      ...this.aiSuggestions,
      ...this.userValues,
      ...this.currentSession
    });
  }
}
```

## Common Patterns

### Endpoint Detection Patterns
```javascript
// Express
app.get('/users/:id', handler)
router.post('/api/users', middleware, handler)

// Koa
router.get('/users', async (ctx) => {})

// Fastify  
fastify.post('/users', { preHandler: auth }, handler)

// Next.js
export async function GET(request) {}
```

### Testing Strategy
```rust
#[test]
fn test_framework_patterns() {
    let code = "app.get('/users/:id', getUser)";
    let endpoints = parser.parse(code).unwrap();
    assert_eq!(endpoints[0].path, "/users/:id");
    assert_eq!(endpoints[0].method, HttpMethod::Get);
}
```

### Error Handling
```rust
// Never crash on invalid code
pub fn parse_file(content: &str) -> Result<Vec<Endpoint>> {
    match tree_sitter_parse(content) {
        Ok(tree) => extract_endpoints(tree),
        Err(_) => Ok(extract_with_fallback(content)) // Regex fallback
    }
}

// AI failures don't break experience
match ai.analyze(endpoint).await {
    Ok(analysis) => enhance_ui(analysis),
    Err(_) => use_basic_defaults() // Always functional
}
```

## Performance Targets

- **Parser**: <10ms per file, <50MB memory
- **AI Cache**: <100ms cached responses, 90%+ hit rate  
- **UI**: 60fps with 1000+ endpoints, <16ms updates
- **Overall**: <30 seconds from install to first test

## Technology Choices

- **Rust**: System-level performance for file watching/parsing
- **Tree-sitter**: Language-agnostic AST parsing with error recovery
- **Tauri**: Native desktop performance with web technologies
- **Svelte 5**: Reactive UI with minimal runtime overhead
- **NX**: Monorepo management with efficient builds
- **Ollama**: Local AI models for privacy and speed

## Current Status

✅ **Phase 1 Complete**: CLI with real-time endpoint discovery
- File watching with async event handling
- JavaScript/TypeScript + Python parsing working
- CLI commands: `discover`, `watch` with table/JSON output

🎯 **Phase 2 Active**: Multi-language support expansion
- Add Go (Gin, Echo) and Rust (Axum, Actix) parsers
- Framework auto-detection
- Performance benchmarking

📋 **Phase 3 Next**: Desktop playground with AI caching
- Tauri app with Svelte 5 UI
- Smart parameter suggestions
- Persistent playground state

## Development Commands

```bash
# Build everything
pnpm nx run-many --target=build --all

# Test specific library
pnpm nx test parser
pnpm nx test core

# Run CLI
pnpm nx run cli:serve -- discover ./test-project
pnpm nx run cli:serve -- watch ./test-project

# See affected projects
pnpm nx affected:graph
```

## Adding Language Support

1. Create parser module: `libs/parser/src/languages/golang.rs`
2. Implement `LanguageParser` trait with framework patterns
3. Add comprehensive tests with real-world examples
4. Update language registry in `libs/parser/src/lib.rs`

## When Working on This Codebase

1. **Read the parser first** - Understand how endpoint detection works
2. **Focus on real-world patterns** - Test with actual framework code
3. **Performance matters** - Profile parsing times, optimize hot paths
4. **Graceful degradation** - Everything works without AI
5. **Preserve user intent** - Don't override user's playground values

The goal is to make API testing feel magical - save a file, test immediately, no configuration required.
