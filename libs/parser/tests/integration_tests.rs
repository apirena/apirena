// Reqsmith Parser Integration Tests
// This file loads the new modular test architecture
mod unit;
mod projects;
mod issues; // Issue reproduction tests

// Test modules are organized as follows:
// - unit/: Framework-specific unit tests with isolated fixtures
//   - unit::nextjs: Next.js framework detection and config generation
//   - unit::express: Express.js framework detection and patterns
//   - unit::flask: Flask framework detection and patterns
//   - unit::fastapi: FastAPI framework detection and patterns  
//   - unit::laravel: Laravel framework detection and patterns
//   - unit::core: Core parsing functionality tests
//
// - projects/: Full project integration tests with realistic scenarios
//   - projects::startup_monorepo: Multi-app monorepo scenarios
//   - projects::single_app: Single application projects
//   - projects::performance: Large-scale performance testing
//   - projects::edge_cases: Ambiguous and edge case scenarios
//
// - issues/: Specific issue reproduction tests
//   - Organized by issue number for easy tracking

// These tests can be run with granular control via Nx:
// nx test parser                              # All tests
// nx run parser:test:unit                     # All unit tests
// nx run parser:test:unit:nextjs              # Next.js unit tests only
// nx run parser:test:projects                 # All project integration tests
// nx run parser:test:projects:startup-monorepo # Specific project type
// nx run parser:test:fast                     # Quick unit tests only
// nx run parser:test:ci                       # CI-optimized test run














