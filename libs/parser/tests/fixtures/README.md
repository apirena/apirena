# Test Fixtures Architecture

This directory contains a comprehensive suite of test fixtures for validating endpoint discovery across multiple languages and frameworks.

## Directory Structure

```
test-fixtures/
├── README.md                    # This file
├── test-scenarios.yml           # Test scenarios configuration
├── expected-results/            # Expected parsing results
├── javascript/
│   ├── express/
│   │   ├── basic-routes.js      # Simple CRUD operations
│   │   ├── middleware.js        # Routes with middleware
│   │   ├── nested-routers.js    # Router.use() patterns
│   │   ├── param-routes.js      # Dynamic parameters
│   │   └── edge-cases.js        # Complex/unusual patterns
│   ├── fastify/
│   │   ├── basic-routes.js
│   │   ├── plugins.js
│   │   └── schemas.js
│   └── koa/
│       ├── basic-routes.js
│       └── middleware.js
├── typescript/
│   ├── express/
│   │   ├── basic-routes.ts
│   │   ├── typed-routes.ts
│   │   └── decorators.ts
│   └── nestjs/
│       ├── controllers.ts
│       ├── modules.ts
│       └── guards.ts
├── python/
│   ├── flask/
│   │   ├── basic-routes.py      # @app.route patterns
│   │   ├── blueprints.py        # Flask blueprints
│   │   ├── methods.py           # Different HTTP methods
│   │   └── decorators.py        # Custom decorators
│   ├── fastapi/
│   │   ├── basic-routes.py      # @app.get, @app.post patterns
│   │   ├── dependencies.py      # Dependency injection
│   │   ├── models.py            # Pydantic models
│   │   └── routers.py           # APIRouter patterns
│   └── django/
│       ├── urls.py              # URL patterns
│       ├── views.py             # Class-based views
│       └── rest-framework.py    # DRF viewsets
├── php/
│   └── laravel/
│       ├── basic-routes.php     # Route facade methods
│       ├── controllers.php      # Controller routing
│       ├── api-routes.php       # API versioning
│       └── resource-routes.php  # Resource controllers
├── go/
│   ├── gin/
│   │   ├── basic-routes.go
│   │   ├── groups.go
│   │   └── middleware.go
│   └── echo/
│       ├── basic-routes.go
│       └── groups.go
└── rust/
    ├── axum/
    │   ├── basic-routes.rs
    │   ├── extractors.rs
    │   └── middleware.rs
    └── actix/
        ├── basic-routes.rs
        └── services.rs
```

## Test Categories

### 1. Basic Routes
Simple HTTP method definitions:
- GET, POST, PUT, DELETE, PATCH
- Static paths: `/users`, `/api/health`
- Basic response handling

### 2. Dynamic Routes
Routes with parameters:
- Path parameters: `/users/:id`, `/users/{id}`
- Query parameters
- Optional parameters

### 3. Middleware & Guards
Routes with authentication/authorization:
- Authentication middleware
- Rate limiting
- CORS handling

### 4. Nested Structures
Complex routing patterns:
- Router groups/blueprints
- Nested routers
- Modular route definitions

### 5. Edge Cases
Unusual but valid patterns:
- Regex routes
- Wildcard routes
- Conditional routing
- Dynamic route generation

## Test Scenarios Configuration

Each test file should have a corresponding entry in `test-scenarios.yml` that defines:
- Expected endpoints to be discovered
- Framework/language identification
- Performance benchmarks
- Edge case handling

## Usage

### For Development
```bash
# Test specific framework
apirena discover test-fixtures/javascript/express/

# Test all JavaScript frameworks
apirena discover test-fixtures/javascript/

# Benchmark all languages
apirena discover test-fixtures/ --format json > benchmark-results.json
```

### For CI/CD
```bash
# Run comprehensive test suite
cargo test --all
nx test parser
```

### Adding New Test Cases

1. Create the test file in the appropriate `language/framework/` directory
2. Add expected results to `test-scenarios.yml`
3. Run tests to verify parsing accuracy
4. Document any new patterns discovered

## Framework Priorities

### Phase 2 (Current)
- ✅ JavaScript/Express 
- ✅ Python/Flask
- 🎯 Python/FastAPI
- 🎯 PHP/Laravel
- 🎯 Go/Gin
- 🎯 Rust/Axum

### Phase 3 (Future)
- TypeScript/NestJS
- JavaScript/Fastify
- Python/Django
- Go/Echo
- Rust/Actix

### Phase 4 (Advanced)
- PHP/Laravel (Blade, Eloquent)
- Java/Spring Boot
- C#/.NET
- Ruby/Rails

## Performance Targets

- **Parse Time**: < 10ms per file
- **Memory Usage**: < 50MB for entire test suite
- **Accuracy**: > 95% endpoint detection
- **False Positives**: < 2%

## Contributing

When adding new test fixtures:
1. Follow the established directory structure
2. Include comprehensive comments explaining the patterns
3. Add both positive and negative test cases
4. Update the expected results configuration
5. Ensure tests pass on all supported platforms
