# Parser Testing Strategy

## Overview

The `hallwatch-parser` library follows a comprehensive testing strategy organized around Nx monorepo conventions, providing granular test execution and optimal CI/CD integration.

## Test Organization

### Directory Structure
```
libs/parser/tests/
├── integration_tests.rs     # Main test entry point
├── unit/                    # Framework-specific unit tests
│   ├── mod.rs
│   ├── nextjs/             # Next.js specific tests
│   ├── express/            # Express.js specific tests
│   ├── flask/              # Flask specific tests
│   ├── fastapi/            # FastAPI specific tests
│   ├── laravel/            # Laravel specific tests
│   └── core/               # Core parser functionality
├── projects/               # Project integration tests
│   ├── mod.rs
│   ├── startup_monorepo/   # Multi-app monorepo scenarios
│   ├── single_app/         # Single application projects
│   ├── edge_cases/         # Ambiguous patterns and edge cases
│   └── performance/        # Large-scale performance testing
└── issues/                 # Bug reproduction tests
    └── mod.rs
```

### Test Categories

#### 1. Unit Tests (`unit/`)
- **Purpose**: Test individual framework detection and parsing logic
- **Scope**: Isolated, framework-specific functionality
- **Examples**: Next.js route detection, Express middleware parsing
- **Speed**: Fast (< 1s per test)

#### 2. Project Tests (`projects/`)
- **Purpose**: Test parser behavior on realistic project structures
- **Scope**: End-to-end parsing of complete project scenarios
- **Examples**: Monorepo with multiple frameworks, single-app deployments
- **Speed**: Medium (1-5s per test)

#### 3. Performance Tests (`projects/performance/`)
- **Purpose**: Validate parser performance on large codebases
- **Scope**: Stress testing with massive file structures
- **Examples**: 1000+ file monorepos, deeply nested directories
- **Speed**: Slow (5-30s per test)

#### 4. Issue Tests (`issues/`)
- **Purpose**: Reproduce and prevent regression of reported bugs
- **Scope**: Specific bug scenarios
- **Examples**: Edge cases found in production
- **Speed**: Variable

## Nx Integration

### Granular Test Execution

The project is configured with granular Nx targets for optimal testing:

```bash
# All tests
nx test parser

# Category-based testing
nx run parser:test:unit                    # All unit tests
nx run parser:test:projects               # All project integration tests
nx run parser:test:fast                   # Quick unit tests only

# Framework-specific testing
nx run parser:test:unit:nextjs            # Next.js tests only
nx run parser:test:unit:express           # Express.js tests only
nx run parser:test:unit:flask             # Flask tests only
nx run parser:test:unit:fastapi           # FastAPI tests only
nx run parser:test:unit:laravel           # Laravel tests only

# Project scenario testing
nx run parser:test:projects:startup-monorepo  # Monorepo scenarios
nx run parser:test:projects:single-app        # Single app scenarios
nx run parser:test:projects:performance       # Performance tests

# Language-specific testing (legacy)
nx run parser:test:javascript             # JavaScript/TypeScript projects
nx run parser:test:python                 # Python projects
nx run parser:test:php                    # PHP projects

# CI/CD optimized
nx run parser:test:ci                     # CI-friendly test execution
```

### Test Runner Script

Use the provided test runner for convenient local development:

```bash
# Fast development feedback
./libs/parser/test.sh fast

# Framework-specific development
./libs/parser/test.sh nextjs
./libs/parser/test.sh express

# Full test suite
./libs/parser/test.sh all
```

## CI/CD Integration

### Recommended CI Pipeline

```yaml
# Example GitHub Actions workflow
test-parser:
  strategy:
    matrix:
      test-type: [fast, unit, projects]
  steps:
    - name: Run ${{ matrix.test-type }} tests
      run: nx run parser:test:${{ matrix.test-type }}
```

### Performance Considerations

- **Fast tests** (`test:fast`): Run on every commit
- **Unit tests** (`test:unit`): Run on PR creation/update
- **Project tests** (`test:projects`): Run on main branch merges
- **Performance tests** (`test:projects:performance`): Run nightly or on release

## Best Practices

### Writing Tests

1. **Use appropriate test category**: Place tests in the correct directory based on scope
2. **Leverage fixtures**: Use the provided fixture helpers for consistent test data
3. **Follow naming conventions**: Use descriptive test names that indicate framework and scenario
4. **Test isolation**: Ensure tests don't depend on each other or external state

### Test Data Management

- **Unit tests**: Use minimal, focused fixtures
- **Project tests**: Use realistic project structures
- **Performance tests**: Generate large datasets programmatically
- **Issue tests**: Include exact reproduction scenarios

### Local Development

1. Start with fast tests: `./test.sh fast`
2. Focus on relevant framework: `./test.sh nextjs`
3. Run full suite before PR: `./test.sh all`
4. Use watch mode during development: `cargo watch -x test`

## Extending the Test Suite

### Adding New Framework Support

1. Create new unit test module: `libs/parser/tests/unit/new_framework/`
2. Add project test scenarios: `libs/parser/tests/projects/*/new_framework/`
3. Update Nx targets in `project.json`
4. Add to test runner script

### Adding Project Scenarios

1. Create scenario directory in appropriate `projects/` subdirectory
2. Include realistic file structure and fixtures
3. Add comprehensive test cases for the scenario
4. Document in this file

## Troubleshooting

### Common Issues

- **Test timeouts**: Increase timeout for performance tests
- **Fixture conflicts**: Ensure unique temporary directories
- **Flaky tests**: Check for race conditions in file system operations
- **Memory usage**: Monitor large fixture generation in performance tests

### Debug Tools

```bash
# Verbose test output
nx run parser:test:ci

# Single test debugging
cargo test specific_test_name -- --nocapture

# Performance profiling
cargo bench --package hallwatch-parser
```
