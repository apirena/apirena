// Auto-generated Hallwatch configuration
// Generated: 2025-07-21T06:10:44.162381518+00:00
// Version: 1.0.0
// Debug Mode: false

export default {
  // Metadata
  _meta: {
    version: "1.0.0",
    generated: "2025-07-21T06:10:44.162381518+00:00",
    lastModified: "2025-07-21T06:10:44.162381518+00:00",
    debugMode: false,
  },

  // Detected project structure
  structure: {
    type: "single",
    fileCount: 4,
    totalSize: 1927,
    roots: [],
  },

  // Detected frameworks
  frameworks: [
    {
      path: "/var/deployment/hallwatch/libs/parser/tests/unit/flask/fixtures/basic_app",
      framework: "flask",
      confidence: 1,

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
