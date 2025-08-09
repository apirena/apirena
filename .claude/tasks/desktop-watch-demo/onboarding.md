````markdown
# Desktop Watch Demo — Production-Ready Onboarding

## Project Vision
ReqSmith is a **filesystem-native** API endpoint discovery tool that watches codebases in real-time. All state lives transparently on disk - no hidden databases, no cloud dependencies. Like how `nx` stores its cache in `.nx/cache`, we store discovered endpoints and watch state in `.reqsmith/` directories that developers can inspect, version control (optionally), and understand.

## Core Philosophy: Filesystem-First Architecture
- **Transparent State**: All discovered endpoints cached as JSON files in `.reqsmith/endpoints/`
- **Portable Config**: Project settings in `.reqsmith/config.json` (can be committed)
- **Local Cache**: Parse results in `.reqsmith/cache/` (gitignored, like node_modules)
- **No Hidden State**: Everything inspectable with `ls` and `cat`

## Goal
Ship a production-quality desktop demo that:
- Discovers and watches API endpoints with <100ms incremental updates
- Stores all state in filesystem (`.reqsmith/` directories)
- Uses existing incremental parser and diff processor for efficiency
- Works self-contained with benchmark projects

## Success Criteria
- **Performance**: Incremental updates <50ms using cached ASTs
- **Transparency**: All state visible in `.reqsmith/` directories
- **Portability**: Copy a project folder → state travels with it
- **Git-Friendly**: Clear separation between committable config and local cache

## Architecture Overview

### Filesystem Layout
````
project-root/
├── .reqsmith/                    # ReqSmith directory (like .git or node_modules)
│   ├── config.json              # Project config (committable)
│   │   {
│   │     "baseUrl": "http://localhost:3000",
│   │     "watch": {
│   │       "include": ["src/**/*.{js,ts}"],
│   │       "exclude": ["**/node_modules/**"]
│   │     }
│   │   }
│   ├── endpoints/               # Discovered endpoints (optionally committable)
│   │   ├── manifest.json       # Index of all endpoints
│   │   └── by-file/           # Endpoints grouped by source file
│   │       ├── src-routes-users.json
│   │       └── src-routes-products.json
│   ├── cache/                  # Local cache (gitignored)
│   │   ├── ast/               # Cached ASTs with content hashes
│   │   │   ├── [hash].ast     # Tree-sitter AST cache
│   │   │   └── index.json     # Hash → file mapping
│   │   └── watch-state.json   # Active watcher state
│   └── .gitignore              # Auto-generated, ignores cache/
├── src/
│   └── routes/
│       ├── users.js
│       └── products.js
└── .gitignore                   # User's gitignore
````

### State Management Approach

#### 1. Config Storage (`.reqsmith/config.json`)
````json
{
  "version": "1.0.0",
  "baseUrl": "http://localhost:3000",
  "environments": {
    "dev": { "baseUrl": "http://localhost:3000" },
    "staging": { "baseUrl": "https://api-staging.example.com" }
  },
  "watch": {
    "include": ["src/**/*.{js,ts,py,php}"],
    "exclude": ["**/node_modules/**", "**/vendor/**"]
  },
  "parser": {
    "incrementalEnabled": true,
    "parallelism": 4
  }
}
````

#### 2. Endpoint Storage (`.reqsmith/endpoints/`)
````json
// .reqsmith/endpoints/manifest.json
{
  "version": "1.0.0",
  "lastUpdated": "2025-08-08T10:00:00Z",
  "endpoints": [
    {
      "id": "GET:/api/users",
      "method": "GET",
      "path": "/api/users",
      "file": "src/routes/users.js",
      "line": 10,
      "framework": "express",
      "lastSeen": "2025-08-08T10:00:00Z"
    }
  ],
  "statistics": {
    "total": 25,
    "byMethod": { "GET": 15, "POST": 8, "DELETE": 2 }
  }
}
````

#### 3. Cache Storage (`.reqsmith/cache/`)
````json
// .reqsmith/cache/ast/index.json
{
  "version": "1.0.0",
  "entries": {
    "src/routes/users.js": {
      "hash": "sha256:abc123...",
      "size": 2048,
      "lastModified": "2025-08-08T10:00:00Z",
      "astFile": "abc123.ast",
      "parseTime": 15.2
    }
  }
}
````

### Core Implementation

- Backend storage in `apps/desktop/src-tauri/src/storage.rs` implements `.reqsmith` directory creation and manifest writes with atomic rename.
- Tauri commands in `apps/desktop/src-tauri/src/lib.rs` now:
  - discover files recursively and persist manifest
  - expose `read_manifest(path)` for UI to load `.reqsmith/endpoints/manifest.json` without extra plugins
- Frontend store in `apps/desktop/src/lib/stores/endpoints.svelte.ts` can load from filesystem and react to updates.

## Implementation Plan

### Phase 1: Filesystem Infrastructure (4 hours)
1. Create `.reqsmith/` directory structure on project selection — Done
2. Implement config loading/saving — Pending
3. Setup endpoint manifest persistence — Done
4. Add AST caching layer — Pending

### Phase 2: Incremental Watching (4 hours)
1. Wire up IncrementalParser with filesystem cache — Pending
2. Implement diff computation against stored state — Pending
3. Update manifest atomically on changes — Done
4. Emit granular events to UI — Partial (endpoints-updated)

### Phase 3: UI Integration (2 hours)
1. Load initial state from `.reqsmith/endpoints/` — Done
2. Watch filesystem for external changes — Pending
3. Persist UI changes back to disk — N/A for now
4. Show cache statistics in UI — Pending

### Phase 4: Polish (2 hours)
1. Add cache invalidation commands — Pending
2. Implement cache size limits — Pending
3. Add performance metrics to cache — Pending
4. Create `.reqsmith/` inspector UI — Pending

## Testing Strategy
````bash
# Verify filesystem structure
 test -d .reqsmith/cache/ast || echo "Cache dir missing"
 test -f .reqsmith/endpoints/manifest.json || echo "Manifest missing"

# Check manifest
 cat .reqsmith/endpoints/manifest.json | jq '.statistics'
````

## Validation Checklist
- [x] `.reqsmith/` directory created on discovery
- [x] Endpoints persisted to `endpoints/manifest.json`
- [ ] AST cache reduces parse time by >90%
- [ ] Config changes reflected immediately
- [ ] Cache survives app restart
- [ ] Filesystem changes detected by watcher
- [ ] Git diff shows meaningful endpoint changes
- [ ] Cache size stays under limits

## Next Steps
- Implement config loading/saving in storage
- Integrate incremental parser and diff processor
- Add native file watcher to update manifest and emit UI events
- Expose cache stats for UI
````
