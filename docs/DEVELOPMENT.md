# Development Guide

## Setup

1. **Prerequisites**
   - Node.js 18+
   - PNPM 8+
   - Rust 1.75+
   - Git

2. **Installation**
   ```bash
   git clone https://github.com/hallwatch/hallwatch.git
   cd hallwatch
   pnpm install
   ```

3. **Building**
   ```bash
   # Build all projects
   pnpm nx run-many --target=build --all
   
   # Build specific library
   pnpm nx build parser
   
   # Build CLI
   cargo build -p hallwatch-cli
   ```

4. **Running**
   ```bash
   # Run CLI
   ./target/debug/hallwatch --help
   
   # Via NX (when configured)
   pnpm nx run cli:serve -- --help
   ```

## Testing

```bash
# Run all tests
pnpm nx run-many --target=test --all

# Run affected tests
pnpm nx affected:test

# Run specific Rust tests
cargo test -p hallwatch-parser

# Run with output
cargo test -p hallwatch-core -- --nocapture
```

## Development Workflow

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/add-golang-parser
   ```

2. **Make Changes**
   - Add code to appropriate library
   - Write tests
   - Update documentation

3. **Test Changes**
   ```bash
   pnpm nx affected:test
   pnpm nx affected:lint
   ```

4. **Commit and Push**
   ```bash
   git commit -m "feat(parser): add Go language support"
   git push origin feature/add-golang-parser
   ```

## Common Tasks

### Adding a New Language Parser

1. Create parser file: `libs/parser/src/languages/golang.rs`
2. Implement `LanguageParser` trait
3. Add tests
4. Update `libs/parser/src/lib.rs` to include new module
5. Add integration tests

### Adding Tree-sitter Grammar

1. Add dependency to `libs/parser/Cargo.toml`:
   ```toml
   tree-sitter-go = "0.20"
   ```

2. Create query file: `libs/parser/src/queries/go.scm`
3. Implement parsing logic in language module

### Debugging

- Use `RUST_LOG=debug` for verbose logging
- Add `dbg!()` macros for quick debugging
- Use `cargo test -- --nocapture` to see println output

## Project Structure

- `apps/cli/` - Command-line interface
- `libs/core/` - File watching and events
- `libs/parser/` - Language parsing logic
- `docs/` - Documentation
- `tools/` - Build tools and utilities
