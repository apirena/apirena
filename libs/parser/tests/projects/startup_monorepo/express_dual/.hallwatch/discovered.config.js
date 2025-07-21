// Auto-generated Hallwatch configuration
// Generated: 2025-07-21T06:10:32.296980962+00:00
// Version: 1.0.0
// Debug Mode: true

export default {
  // Metadata
  _meta: {
    version: "1.0.0",
    generated: "2025-07-21T06:10:32.296980962+00:00",
    lastModified: "2025-07-21T06:10:32.296980962+00:00",
    debugMode: true,
  },

  // Detected project structure
  structure: {
    type: "monorepo",
    fileCount: 15,
    totalSize: 6955,
    roots: ["./apps", "./packages"],
  },

  // Detected frameworks
  frameworks: [
    {
      path: "/var/deployment/hallwatch/libs/parser/tests/projects/startup_monorepo/express_dual/apps/api",
      framework: "express",
      confidence: 1,

      // Detection signals (debug mode)
      _signals: [
        { type: "package.json", value: "express dependency found", confidence: 0.8, source: "/var/deployment/hallwatch/libs/parser/tests/projects/startup_monorepo/express_dual/apps/api/package.json" },
        { type: "code_pattern", value: "express() or app.method() found", confidence: 0.9, source: "/var/deployment/hallwatch/libs/parser/tests/projects/startup_monorepo/express_dual/apps/api/src/server.js" },
      ],

      // Patterns for route detection
      patterns: [
        {
          name: "express.app-routes",
          files: "**/*.{js,ts}",
          routes: ["app.{method}('{path}', {handler})", "app.{method}('{path}', {middlewares}, {handler})"],
          confidence: 0.95,
        },
        {
          name: "express.router",
          files: "**/routes/**/*.{js,ts}",
          routes: ["router.{method}('{path}', {handler})"],
          confidence: 0.9,
        },
      ],
    },
    {
      path: "/var/deployment/hallwatch/libs/parser/tests/projects/startup_monorepo/express_dual/apps/admin",
      framework: "express",
      confidence: 1,

      // Detection signals (debug mode)
      _signals: [
        { type: "package.json", value: "express dependency found", confidence: 0.8, source: "/var/deployment/hallwatch/libs/parser/tests/projects/startup_monorepo/express_dual/apps/admin/package.json" },
        { type: "code_pattern", value: "express() or app.method() found", confidence: 0.9, source: "/var/deployment/hallwatch/libs/parser/tests/projects/startup_monorepo/express_dual/apps/admin/app.js" },
      ],

      // Patterns for route detection
      patterns: [
        {
          name: "express.app-routes",
          files: "**/*.{js,ts}",
          routes: ["app.{method}('{path}', {handler})", "app.{method}('{path}', {middlewares}, {handler})"],
          confidence: 0.95,
        },
        {
          name: "express.router",
          files: "**/routes/**/*.{js,ts}",
          routes: ["router.{method}('{path}', {handler})"],
          confidence: 0.9,
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
