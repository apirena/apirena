// Auto-generated Hallwatch configuration
// Generated: 2025-07-22T15:21:13.836376+00:00
// Version: 1.0.0
// Debug Mode: true

export default {
  // Metadata
  _meta: {
    version: "1.0.0",
    generated: "2025-07-22T15:21:13.836376+00:00",
    lastModified: "2025-07-22T15:21:13.836376+00:00",
    debugMode: true,
  },

  // Global configuration
  debugMode: true,

  // Detected project structure
  structure: {
    type: "single",
    fileCount: 4,
    totalSize: 3491,
    roots: [],
  },

  // Detected frameworks
  frameworks: [
    {
      path: "/Users/isaiahdahl/deployment/hallwatch/libs/parser/tests/unit/flask/fixtures/basic_app",
      framework: "flask",
      confidence: 1,

      // Detection signals (debug mode)
      _signals: [
        { type: "requirements.txt", value: "flask dependency found", confidence: 0.8, source: "/Users/isaiahdahl/deployment/hallwatch/libs/parser/tests/unit/flask/fixtures/basic_app/requirements.txt" },
        { type: "code_pattern", value: "Flask(__name__) or @app.route found", confidence: 0.9, source: "/Users/isaiahdahl/deployment/hallwatch/libs/parser/tests/unit/flask/fixtures/basic_app/app.py" },
      ],

      // Patterns for route detection
      patterns: [
        {
          name: "flask.decorators",
          files: "**/*.py",
          routes: ["@app.route('{path}', methods=['{method}'])"],
          confidence: 0.95,
        },
      ],
    }
  ],

  // Performance settings (auto-calculated)
  performance: {
    threads: "auto",
    cacheStrategy: "minimal",
    maxFileSize: "1MB",
    estimatedScanTime: "<10 seconds",
  },

  // User overrides (preserved between regenerations)
  overrides: {
    // Add your custom patterns here
    // These will be preserved when config is regenerated
    
    // Example:
    // customPatterns: [
    //   {
    //     name: 'my-custom-api',
    //     files: 'lib/handlers/**/*.js',
    //     routes: ['defineHandler("{method}", "{path}", {handler})']
    //   }
    // ]
  },
};
