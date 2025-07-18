# AI Agent Development Guidelines

## Overview

This guide helps AI agents work effectively on the Apirena codebase. The project uses NX for monorepo management, Rust for backend logic, and Svelte 5 for the UI.

## Project Structure

```
- libs/     # Shared libraries (Rust)
- apps/     # Applications (CLI and Desktop)
- docs/     # Documentation
- tools/    # Build tools and scripts
```

## Development Workflow

### 1. Before Starting a Task

```bash
# Ensure you're on the latest main branch
git checkout main
git pull

# Install/update dependencies
pnpm install

# Check if everything builds
pnpm nx run-many --target=build --all
```

### 2. Working on a Feature

Each feature should be developed in a library first, then integrated into apps.

#### Creating a New Library

```bash
# For Rust libraries
pnpm nx g @monodon/rust:library <name> --directory=libs

# For TypeScript libraries  
pnpm nx g @nx/js:library <name> --directory=libs
```

#### Library Structure

```
libs/my-feature/
├── src/
│   ├── lib.rs          # Main library file
│   ├── mod.rs          # Module definitions
│   └── tests.rs        # Unit tests
├── Cargo.toml          # Rust dependencies
├── project.json        # NX configuration
└── README.md          # Library documentation
```

### 3. Code Standards

#### Rust Code

```rust
// Always use Result for fallible operations
pub fn parse_file(path: &Path) -> Result<Vec<Endpoint>> {
    // Implementation
}

// Document public APIs
/// Discovers API endpoints in the given file
/// 
/// # Arguments
/// * `path` - Path to the source file
/// 
/// # Returns
/// * `Ok(Vec<Endpoint>)` - List of discovered endpoints
/// * `Err(Error)` - If parsing fails
pub fn discover_endpoints(path: &Path) -> Result<Vec<Endpoint>> {
    // Implementation
}

// Use strong types
pub struct Endpoint {
    pub method: HttpMethod,
    pub path: String,
    pub handler: String,
}

// Implement standard traits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}
```

#### TypeScript/Svelte Code

```typescript
// Use TypeScript strict mode
// Define interfaces for all data structures
export interface Endpoint {
  method: HttpMethod;
  path: string;
  handler: string;
  documentation?: string;
}

// Use Svelte 5 runes
class EndpointStore {
  endpoints = $state<Endpoint[]>([]);
  loading = $state(false);
  
  // Derived values use $derived
  get totalEndpoints() {
    return $derived(this.endpoints.length);
  }
}

// Component props with TypeScript
interface Props {
  endpoint: Endpoint;
  onTest?: (endpoint: Endpoint) => void;
}
```

### 4. Testing Requirements

#### Unit Tests

Every function should have tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_express_route() {
        let content = r#"app.get('/users', handler)"#;
        let endpoints = parse_javascript(content).unwrap();
        
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].method, HttpMethod::Get);
        assert_eq!(endpoints[0].path, "/users");
    }
}
```

#### Integration Tests

```rust
// tests/integration_test.rs
#[test]
fn test_full_parsing_pipeline() {
    // Test complete workflow
}
```

### 5. Adding Language Support

To add support for a new language:

1. Create parser module:
```rust
// libs/parser/src/languages/python.rs
pub struct PythonParser;

impl LanguageParser for PythonParser {
    fn parse(&self, content: &str) -> Result<Vec<Endpoint>> {
        // Implementation
    }
}
```

2. Add Tree-sitter query:
```
// libs/parser/src/queries/python.scm
(decorator
  (call
    function: (attribute) @route_decorator
    arguments: (argument_list
      (string) @path)))
```

3. Add tests:
```rust
#[test]
fn test_parse_flask_route() {
    let content = r#"
@app.route('/users')
def get_users():
    return []
"#;
    
    let endpoints = PythonParser.parse(content).unwrap();
    assert_eq!(endpoints[0].path, "/users");
}
```

### 6. NX Task Guidelines

#### Common Commands

```bash
# Build a specific library
pnpm nx build parser

# Test a specific library
pnpm nx test core

# Run affected tests after changes
pnpm nx affected:test

# See what will be affected by your changes
pnpm nx affected:graph

# Format code
pnpm nx format:write

# Lint code
pnpm nx affected:lint
```

#### Task Dependencies

NX handles task dependencies automatically. Define them in `project.json`:

```json
{
  "targets": {
    "build": {
      "dependsOn": ["^build"]
    }
  }
}
```

### 7. Debugging Tips

#### Rust Debugging

```bash
# Run with debug logging
RUST_LOG=debug pnpm nx run cli:serve

# Run specific test with output
pnpm nx test parser -- --nocapture test_name
```

#### UI Debugging

```javascript
// Use console.log in development
$inspect(endpoints); // Svelte 5 helper

// Chrome DevTools for Tauri
// Press F12 in the desktop app
```

### 8. Performance Guidelines

- Parser must complete in < 10ms
- UI updates must be < 16ms (60fps)
- Memory usage should be < 100MB
- Use streaming for large operations

### 9. Error Handling

```rust
// Use anyhow for error propagation
use anyhow::{Result, Context};

pub fn parse_file(path: &Path) -> Result<Vec<Endpoint>> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read file")?;
    
    parse_content(&content)
        .context("Failed to parse content")
}

// Custom errors for specific cases
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Invalid syntax at line {line}")]
    SyntaxError { line: usize },
}
```

### 10. Git Workflow

```bash
# Create feature branch
git checkout -b feature/add-ruby-support

# Make changes and test
pnpm nx affected:test

# Commit with conventional commits
git commit -m "feat(parser): add Ruby on Rails support"

# Push and create PR
git push origin feature/add-ruby-support
```

## Common Patterns

### Adding a New Endpoint Parser

1. Define the parser interface
2. Implement for specific framework
3. Add tests with real-world examples
4. Update documentation

### Extending the UI

1. Create Svelte component in `libs/ui`
2. Add to desktop app
3. Connect to Rust backend via Tauri commands
4. Test with real data

### Performance Optimization

1. Profile first with `cargo flamegraph`
2. Add benchmarks
3. Optimize hot paths
4. Verify improvements

## Resources

- [NX Documentation](https://nx.dev)
- [Tauri Guides](https://tauri.app)
- [Tree-sitter Docs](https://tree-sitter.github.io)
- [Svelte 5 Docs](https://svelte.dev)
