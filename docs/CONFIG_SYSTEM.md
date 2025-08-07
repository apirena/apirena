# Auto-Generated Configuration System ✅ IMPLEMENTED

The Reqsmith config generation system automatically detects project frameworks and creates intelligent configuration files for optimal endpoint discovery.

## Current Implementation Status ✅

**✅ WORKING**: Full configuration generation system implemented and tested
- **Framework Detection**: 5 frameworks supported (Express, Flask, FastAPI, Next.js, Laravel)
- **Config File Generation**: `.reqsmith/discovered.config.js` created automatically
- **Confidence Scoring**: Intelligent framework detection with 0.0-1.0 confidence
- **Monorepo Support**: Multi-framework project detection
- **Performance Optimization**: Auto-tuned settings based on project size
- **Debug Mode**: Detailed detection signals for troubleshooting

**⏳ CLI COMMAND PLANNED**: Direct config generation commands (currently integrated into discovery)
- The system works automatically during `reqsmith discover` and `reqsmith watch`
- Standalone `reqsmith config` command planned for Phase 3

## Quick Start

```bash
# Current working method - config generated automatically
reqsmith discover /path/to/your/project
# ✅ Automatically generates .reqsmith/discovered.config.js

reqsmith watch /path/to/your/project  
# ✅ Generates config + monitors for changes

# Planned for Phase 3
reqsmith config /path/to/your/project --debug
reqsmith config /path/to/your/project --format json
```

## What Gets Generated

### `.reqsmith/discovered.config.js`
A human-readable JavaScript configuration file that includes:

- **Project Structure Analysis**: Type (monorepo/single/multi-app), file count, size
- **Framework Detection**: All detected frameworks with confidence scores
- **Route Patterns**: Optimized patterns for each framework
- **Performance Settings**: Auto-tuned based on project size
- **Override Section**: Where you can add custom patterns

### `.reqsmith/discovered.config.json` (Debug Mode)
A JSON version of the configuration for easy parsing and analysis.

## Framework Detection ✅ IMPLEMENTED

The system automatically detects:

### JavaScript/TypeScript ✅ WORKING
- **Express.js**: ✅ Via package.json + code patterns like `app.get()`, `router.post()` 
- **Next.js**: ✅ Via package.json + `next.config.js` + API directories (app router)
- **Fastify**: ⏳ Patterns ready, integration pending
- **Koa**: ⏳ Patterns ready, integration pending

### Python ✅ WORKING  
- **FastAPI**: ✅ Via requirements.txt/pyproject.toml + `@app.get()` decorators
- **Flask**: ✅ Via requirements.txt + `@app.route()` decorators
- **Django**: ⏳ Patterns ready, integration pending

### PHP ✅ WORKING
- **Laravel**: ✅ Via composer.json + artisan file + Route:: patterns

### Other Languages ⏳ PLANNED
- **Go**: Gin, Echo, Fiber patterns (framework detection ready, AST parser needed)
- **Rust**: Actix, Rocket, Axum patterns (framework detection ready, AST parser needed)
- **Java**: Spring Boot patterns (planned for Phase 3)

## Configuration Structure

```javascript
export default {
  // Metadata about when/how config was generated
  _meta: {
    version: "1.0.0",
    generated: "2025-07-21T03:46:56Z",
    debugMode: false,
  },

  // Project analysis
  structure: {
    type: "monorepo",           // or "single", "multi-app"
    fileCount: 1247,
    totalSize: 45678,
    roots: ["./apps", "./packages"],
  },

  // Detected frameworks with patterns
  frameworks: [
    {
      path: "./apps/api",
      framework: "express",
      confidence: 0.95,           // 0.0 - 1.0
      
      // Only in debug mode
      _signals: [
        { type: "package.json", value: "express dependency found", confidence: 0.8 },
        { type: "code_pattern", value: "app.get() calls found", confidence: 0.9 }
      ],
      
      patterns: [
        {
          name: "express.app-routes",
          files: "**/*.{js,ts}",
          routes: ["app.{method}('{path}', {handler})"],
          confidence: 0.95
        }
      ]
    }
  ],

  // Auto-calculated performance settings
  performance: {
    threads: "4",                  // Based on project size
    cacheStrategy: "aggressive",   // Based on file count
    maxFileSize: "1MB",
    estimatedScanTime: "< 10 seconds"
  },

  // Your customizations (preserved on regeneration)
  overrides: {
    customPatterns: [
      {
        name: 'my-custom-api',
        files: 'lib/handlers/**/*.js',
        routes: ['defineHandler("{method}", "{path}", {handler})']
      }
    ]
  }
};
```

## Confidence Scores

Detection confidence helps you understand how certain the system is:

- **0.9 - 1.0**: Very confident, multiple strong signals
- **0.7 - 0.9**: Confident, good evidence found
- **0.6 - 0.7**: Moderate confidence, some evidence
- **< 0.6**: Low confidence, patterns not included

## Performance Optimization

The system automatically optimizes based on project size:

| Project Size | Threads | Cache Strategy | Max File Size |
|-------------|---------|---------------|---------------|
| < 1K files  | auto    | minimal       | 1MB          |
| 1K-10K files| 4       | balanced      | 1MB          |
| > 10K files | 8       | aggressive    | 512KB        |

## Customization

### Adding Custom Patterns

```javascript
// In .reqsmith/discovered.config.js
overrides: {
  customPatterns: [
    {
      name: 'my-framework',
      files: 'src/routes/**/*.js',
      routes: ['registerRoute("{method}", "{path}", {handler})'],
      confidence: 0.8
    }
  ],

  // Exclude specific paths
  exclude: [
    'legacy/**',
    'deprecated/**'
  ],

  // Override parameter generation
  parameterHints: {
    userId: () => `user_${Math.random().toString(36).substr(2, 9)}`,
    email: () => 'test@example.com'
  }
}
```

### Preserving Overrides

When you regenerate the config, your `overrides` section is preserved. The system will:

1. Keep all your custom patterns
2. Update framework detection
3. Refresh performance settings
4. Maintain your customizations

## Best Practices

### 1. Review Generated Config
Always check the generated configuration to ensure it detected your frameworks correctly.

### 2. Use Debug Mode Initially
Run with `--debug` first to see detection signals and understand why frameworks were detected.

### 3. Add Custom Patterns Gradually
Start with auto-detection, then add custom patterns only for missed endpoints.

### 4. Regenerate Periodically
Regenerate config when adding new frameworks or significantly changing project structure.

### 5. Version Control ✅ RECOMMENDED
**✅ DO COMMIT** `.reqsmith/discovered.config.js` to version control so your team shares the same patterns.

**Why commit the generated config:**
- **Team Consistency**: Everyone gets identical endpoint detection patterns
- **CI/CD Compatibility**: Build systems use the same framework detection  
- **Faster Onboarding**: New developers get working config immediately
- **Deterministic Builds**: Same patterns across all environments
- **Change Tracking**: See when frameworks are added/removed

**Recommended .gitignore:**
```gitignore
# Keep generated configs (like package.json)
# .reqsmith/discovered.config.js   # ✅ COMMIT THIS
# .reqsmith/discovered.config.json # ✅ COMMIT THIS  

# Ignore user-specific files (future)
.reqsmith/cache/
.reqsmith/logs/
.reqsmith/user-settings.json
```

**Auto-Regeneration Triggers:**
- `reqsmith discover` or `reqsmith watch` commands
- Framework dependency changes (new package.json entries)
- Major project structure changes
- Manual config regeneration

## Troubleshooting

### Framework Not Detected
1. Check if dependencies are in package.json/requirements.txt
2. Verify code patterns match expected style
3. Use `--debug` to see what signals were found
4. Add custom patterns in overrides

### Low Confidence Scores
- Add more explicit patterns in your code
- Check for typos in dependency names
- Ensure framework files are in expected locations

### Missing Endpoints
1. Verify patterns match your code style
2. Add custom patterns for non-standard implementations
3. Check file exclusion patterns
4. Use `reqsmith discover` to test pattern matching

## Examples

### Monorepo with Multiple Frameworks
```bash
# Project structure:
# apps/
#   api/        (Express.js)
#   web/        (Next.js)
#   admin/      (React + API routes)
# services/
#   auth/       (FastAPI)
#   ml/         (FastAPI)

reqsmith config . --debug
# Detects: Express, Next.js, FastAPI (3 instances)
```

### Single App Project
```bash
# Simple Flask project
reqsmith config . 
# Detects: Flask with appropriate patterns
```

### Custom Framework
```javascript
// Add to overrides for custom framework
customPatterns: [
  {
    name: 'my-api-framework',
    files: 'api/**/*.js',
    routes: ['API.route("{method}", "{path}", {handler})']
  }
]
```

The auto-generated configuration system makes Reqsmith work out of the box for most projects while providing the flexibility to handle any custom patterns you might have.
