# Contributing to Apirena

Thank you for your interest in contributing to Apirena! This guide will help you get started with our AI-powered API endpoint discovery tool.

## Quick Start

### Prerequisites

- **Node.js** 18+ and **pnpm**
- **Rust** 1.88+ with Cargo
- **Git**

### Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/apirena.git
   cd apirena
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Build the project**
   ```bash
   # Build all projects using NX
   pnpm nx run-many --target=build --all
   
   # Or build specific projects
   pnpm nx build core
   pnpm nx build parser  
   pnpm nx build cli
   ```

4. **Run tests**
   ```bash
   # Run all unit tests
   pnpm nx run-many --target=test --all
   
   # Run integration tests (CLI against test fixtures)
   pnpm nx run cli:integration-test
   
   # Run specific Rust crate tests
   pnpm nx test core
   pnpm nx test parser
   ```

5. **Development workflow**
   ```bash
   # Watch mode for development
   pnpm nx watch --all -- pnpm nx test \\$NX_PROJECT_NAME
   
   # Lint code
   pnpm nx run-many --target=clippy --all
   ```

## Testing Strategy

### Unit Tests
- **Location**: `libs/*/src/` directories alongside source code
- **Run**: `pnpm nx test <library-name>` (e.g., `pnpm nx test parser`)
- **Scope**: Individual functions and modules within each library

### Integration Tests  
- **Location**: `libs/*/tests/` with library-specific test fixtures
- **Parser tests**: `pnpm nx integration-test parser`
- **Core tests**: `pnpm nx integration-test core`
- **CLI tests**: `pnpm nx run cli:test:php:laravel` (granular by framework)

### Granular Testing
Run specific framework tests using NX targets:
- **PHP Laravel**: `pnpm nx run cli:test:php:laravel`
- **JavaScript Express**: `pnpm nx run cli:test:javascript:express`
- **Python FastAPI**: `pnpm nx run cli:test:python:fastapi`
- **Python Flask**: `pnpm nx run cli:test:python:flask`

### Performance Tests
- **Parse time benchmarks**: Ensure parsing stays under 50ms
- **Memory usage**: Monitor resource consumption
- **Run**: `pnpm nx benchmark parser`

### Test Organization
```
libs/
├── parser/tests/
│   ├── integration_tests.rs    # Parser integration tests
│   └── fixtures/              # Language-specific test files
│       ├── javascript/express/
│       ├── python/flask/
│       ├── python/fastapi/
│       ├── php/laravel/
│       ├── go/gin/
│       └── rust/axum/
└── core/tests/
    └── integration_tests.rs      # File watcher integration tests
```

## Development Guidelines

### Code Style

- **Rust**: Follow standard Rust conventions (rustfmt, clippy)
- **TypeScript**: Use ESLint and Prettier
- **Commits**: Use conventional commits (feat:, fix:, docs:, etc.)

### Testing

We use a comprehensive testing approach:

- **Unit Tests**: Write tests for all new functionality using Rust's built-in test framework
- **Integration Tests**: CLI tests against real framework code samples  
- **Coverage Goal**: Aim for >80% test coverage
- **Performance**: Parse time must stay under 50ms for typical files

**Adding a new framework:**
1. Create test fixtures in `libs/parser/tests/fixtures/<language>/<framework>/`
2. Add test cases to `libs/parser/tests/integration_tests.rs`
3. Update CLI project.json with new granular test targets
4. Ensure all test files parse correctly with expected endpoint counts

**Testing best practices:**
- Test edge cases and error conditions
- Include both positive and negative test cases
- Add performance benchmarks for new parsers
- Use library-specific test organization following NX principles
- Document expected test outcomes in integration tests

### Documentation

- Document all public APIs
- Update README.md for user-facing changes
- Add examples for new features

## Areas for Contribution

### High Priority

1. **Language Parsers**
   - Add support for new programming languages
   - Improve existing parser accuracy
   - Add framework detection patterns

2. **Framework Support**
   - Express.js, Fastify (JavaScript)
   - FastAPI, Flask, Django (Python)
   - Gin, Echo, Fiber (Go)
   - Actix, Rocket, Axum (Rust)

3. **Core Features**
   - File watching optimization
   - Parser performance improvements
   - Error handling enhancements

### Medium Priority

1. **Desktop Application**
   - UI/UX improvements
   - New feature implementation
   - Cross-platform testing

2. **CLI Enhancements**
   - Additional command-line options
   - Output formatting improvements
   - Configuration file support

### Low Priority

1. **Documentation**
   - Tutorial improvements
   - API documentation
   - Example projects

2. **Testing**
   - Test coverage improvements
   - Performance benchmarks
   - Edge case testing

## Submitting Changes

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow the coding guidelines
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   # Run unit tests for affected projects
   pnpm nx affected:test
   
   # Run linting
   pnpm nx affected:clippy
   
   # Run integration tests to verify CLI behavior  
   pnpm nx run cli:integration-test
   
   # Run specific framework tests
   pnpm nx run cli:test-php-laravel
   ```

4. **Commit your changes**
   ```bash
   git commit -m "feat(parser): add Ruby on Rails support"
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**
   - Provide a clear description of your changes
   - Reference any related issues
   - Include screenshots for UI changes

## Pull Request Guidelines

- **Title**: Use conventional commit format
- **Description**: Explain what you changed and why
- **Testing**: Describe how you tested your changes
- **Documentation**: Update docs if needed

## Getting Help

- **Discord**: Join our community at [discord.gg/apirena](https://discord.gg/apirena)
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Use GitHub Discussions for questions

## License

By contributing to Apirena, you agree that your contributions will be licensed under the FSL 1.1 license.

Thank you for contributing! 🚀
