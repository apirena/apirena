# Hallwatch Project Overview

## What is Hallwatch?

A **code-first API playground** that eliminates the gap between writing APIs and testing them. No configuration, no annotations, no spec drift - just save your code and test immediately.

## The Problem We Solve

Traditional API tools require manual work:
- Postman: Manually build collections, copy-paste URLs
- Swagger: Write OpenAPI specs that drift from implementation  
- Insomnia: Import/export, maintain separate configurations

**Hallwatch**: Your code IS the specification.

## How It Works

```
Write Code → AST Parser → AI Enhancement → Instant Playground
     ↓           ↓             ↓              ↓
app.get()   Endpoint      Parameter      Test Ready
           Detected      Suggestions    
```

## Technical Approach

### 1. AST-Based Discovery
Using Tree-sitter parsers to extract endpoints from source code:
- No decorators required
- Works with any coding style
- Framework-agnostic patterns
- Error-tolerant parsing

### 2. Smart AI Caching
AI analyzes endpoints once, caches results by signature:
- Endpoint signature = hash(method + path + params + middleware)
- Cache until signature changes
- Local-first models (Ollama)
- User overrides preserved

### 3. Persistent Playground
State management that preserves your work:
- Test values persist between sessions
- Environment configurations saved
- Request history maintained
- Never lose your testing setup

## Differentiation

| Feature | Postman | Insomnia | Swagger | Hallwatch |
|---------|---------|----------|---------|-----------|
| Configuration | Manual | Import/Export | Annotations | None |
| Time to Test | 5+ min | 3+ min | 2+ min | <10 sec |
| Spec Accuracy | Drifts Day 1 | Requires Sync | Framework Dependent | Always Current |
| Intelligence | Basic Templates | Environment Vars | Type Validation | Context-Aware AI |

## Current Implementation

### Completed (Phase 1)
- ✅ File watching with async event handling
- ✅ JavaScript/TypeScript parser (Express, Fastify, etc.)
- ✅ Python parser (Flask, FastAPI)
- ✅ CLI with `discover` and `watch` commands
- ✅ Real-time endpoint detection
- ✅ JSON and table output formats

### In Progress (Phase 2)  
- 🎯 Go language support (Gin, Echo)
- 🎯 Rust language support (Axum, Actix)
- 🎯 Framework auto-detection
- 🎯 Performance benchmarking

### Next (Phase 3)
- 📋 Tauri desktop application
- 📋 AI parameter suggestions with caching
- 📋 Persistent playground state
- 📋 Environment management

## Repository Structure

```
hallwatch/
├── apps/
│   ├── cli/           # Command-line interface (Rust)
│   └── desktop/       # Tauri playground app (future)
├── libs/
│   ├── core/          # File watching system (Rust)
│   ├── parser/        # AST-based endpoint discovery (Rust)
│   └── ai/            # Smart caching and analysis (future)
├── docs/              # Architecture and specifications
└── .claude/           # AI agent instructions
```

## Example Usage

```bash
# Discover endpoints in a project
hallwatch discover ./my-api-project

# Watch for changes in real-time
hallwatch watch ./my-api-project

# Output
📋 Found 12 endpoint(s):
METHOD   PATH                HANDLER           LINE
GET      /users              getUsers          15
POST     /users              createUser        23
GET      /users/:id          getUserById       31
PUT      /users/:id          updateUser        45
DELETE   /users/:id          deleteUser        58
```

## Vision

Make API testing feel **magical**:
1. Write an endpoint in your editor
2. Save the file  
3. Test immediately in the playground
4. No steps in between

The ultimate goal is eliminating all friction between API implementation and testing, making developers more productive and APIs more reliable.
