// Auto-generated PinPath configuration
// Generated: 2025-08-07T06:18:10.568750+00:00
// Version: 1.0.0
// Debug Mode: false

export default {
  // Metadata
  _meta: {
    version: "1.0.0",
    generated: "2025-08-07T06:18:10.568750+00:00",
    lastModified: "2025-08-07T06:18:10.568750+00:00",
    debugMode: false,
  },

  // Global configuration
  debugMode: false,

  // Detected project structure
  structure: {
    type: "single",
    fileCount: 3,
    totalSize: 2022,
    roots: [],
  },

  // Detected frameworks
  frameworks: [
    {
      path: "/Users/isaiahdahl/deployment/pinpath/libs/parser/tests/unit/laravel/fixtures/basic_app",
      framework: "laravel",
      confidence: 0.9,

      // Patterns for route detection
      patterns: [
        {
          name: "laravel.routes",
          files: "routes/**/*.php",
          routes: ["Route::{method}('{path}', {handler})"],
          convention: "/api/users -> routes/api.php + UserController",
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
