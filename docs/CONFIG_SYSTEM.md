# Auto-Generated Configuration System

The Hallwatch config generation system automatically detects project frameworks and creates intelligent configuration files for optimal endpoint discovery.

## Quick Start

```bash
# Generate configuration for your project
hallwatch config /path/to/your/project

# Generate with debug information (shows detection signals)
hallwatch config /path/to/your/project --debug

# Output as JSON for programmatic use
hallwatch config /path/to/your/project --format json
```

## What Gets Generated

### `.hallwatch/discovered.config.js`
A human-readable JavaScript configuration file that includes:

- **Project Structure Analysis**: Type (monorepo/single/multi-app), file count, size
- **Framework Detection**: All detected frameworks with confidence scores
- **Route Patterns**: Optimized patterns for each framework
- **Performance Settings**: Auto-tuned based on project size
- **Override Section**: Where you can add custom patterns

### `.hallwatch/discovered.config.json` (Debug Mode)
A JSON version of the configuration for easy parsing and analysis.

## Framework Detection

The system automatically detects:

### JavaScript/TypeScript
- **Express.js**: Via package.json + code patterns like `app.get()`, `router.post()`
- **Next.js**: Via package.json + `next.config.js` + API directories
- **Fastify**: Via package.json + Fastify-specific patterns
- **Koa**: Via package.json + Koa patterns

### Python
- **FastAPI**: Via requirements.txt/pyproject.toml + `@app.get()` decorators
- **Flask**: Via requirements.txt + `@app.route()` decorators
- **Django**: Via settings.py and URL patterns

### PHP
- **Laravel**: Via composer.json + artisan file + Route:: patterns

### Other Languages
- **Go**: Gin, Echo, Fiber patterns (coming soon)
- **Rust**: Actix, Rocket, Axum patterns (coming soon)
- **Java**: Spring Boot patterns (coming soon)

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
// In .hallwatch/discovered.config.js
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

### 5. Version Control
Commit the generated config to version control so your team shares the same patterns.

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
4. Use `hallwatch discover` to test pattern matching

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

hallwatch config . --debug
# Detects: Express, Next.js, FastAPI (3 instances)
```

### Single App Project
```bash
# Simple Flask project
hallwatch config . 
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

The auto-generated configuration system makes Hallwatch work out of the box for most projects while providing the flexibility to handle any custom patterns you might have.
