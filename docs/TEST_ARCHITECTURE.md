# Test Architecture Proposal

## Current Problems
- Global test fixtures in workspace root
- Mixed unit/integration/e2e tests 
- Monolithic test runner
- No test isolation per library

## Proposed Structure

```
libs/
├── core/
│   ├── src/
│   ├── tests/           # Integration tests for core library
│   │   ├── file_watcher_tests.rs
│   │   └── fixtures/
│   │       ├── mock_files/
│   │       └── events/
│   └── project.json     # NX targets for core-specific tests
├── parser/
│   ├── src/
│   ├── tests/           # Integration tests for parser library  
│   │   ├── javascript_parser_tests.rs
│   │   ├── python_parser_tests.rs
│   │   ├── php_parser_tests.rs
│   │   └── fixtures/    # Parser-specific test files
│   │       ├── javascript/
│   │       │   ├── express/
│   │       │   ├── fastify/
│   │       │   └── nextjs/
│   │       ├── python/
│   │       │   ├── flask/
│   │       │   ├── fastapi/
│   │       │   └── django/
│   │       └── php/
│   │           └── laravel/
│   └── project.json     # NX targets for parser tests
└── git/
    ├── src/
    ├── tests/
    │   └── fixtures/
    └── project.json

apps/
├── cli/
│   ├── src/
│   ├── tests/           # CLI integration tests
│   │   ├── e2e/         # End-to-end CLI tests
│   │   ├── scenarios/   # Real-world usage scenarios
│   │   └── fixtures/    # CLI-specific test data
│   └── project.json     # CLI test targets
└── desktop/
    ├── src/
    ├── tests/
    └── project.json

tools/
├── testing/             # Shared test utilities
│   ├── test-runner.rs   # Rust-based test orchestration
│   ├── fixtures.rs      # Fixture management utilities
│   └── assertions.rs    # Custom assertion helpers
└── scripts/             # Build and automation scripts
```

## Test Types by Layer

### Unit Tests
- Location: `src/` alongside source code (`#[cfg(test)]`)
- Scope: Individual functions and modules
- Command: `nx test <library>`

### Integration Tests  
- Location: `<project>/tests/`
- Scope: Library APIs and cross-module functionality
- Command: `nx integration-test <library>`

### End-to-End Tests
- Location: `apps/<app>/tests/e2e/`
- Scope: Full application workflows
- Command: `nx e2e <app>`

### Performance Tests
- Location: `<project>/tests/benchmarks/`
- Scope: Parse times, memory usage
- Command: `nx benchmark <library>`

## Benefits
1. **Isolation**: Each library owns its test data
2. **Scalability**: Easy to add new languages/frameworks
3. **Maintainability**: Clear ownership and boundaries
4. **NX Compliance**: Proper dependency tracking and caching
5. **Parallel Execution**: Tests can run independently
