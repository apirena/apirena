// Auto-generated Hallwatch configuration
// Generated: 2025-07-22T15:21:13.837670+00:00
// Version: 1.0.0
// Debug Mode: false

export default {
  // Metadata
  _meta: {
    version: "1.0.0",
    generated: "2025-07-22T15:21:13.837670+00:00",
    lastModified: "2025-07-22T15:21:13.837670+00:00",
    debugMode: false,
  },

  // Global configuration
  debugMode: false,

  // Detected project structure
  structure: {
    type: "single",
    fileCount: 6,
    totalSize: 5142,
    roots: [],
  },

  // Detected frameworks
  frameworks: [
    {
      path: "/Users/isaiahdahl/deployment/hallwatch/libs/parser/tests/unit/nextjs/fixtures/app_router",
      framework: "nextjs",
      confidence: 1,

      // Patterns for route detection
      patterns: [
        {
          name: "nextjs.pages-api",
          files: "pages/api/**/*.{js,ts}",
          routes: ["export default function handler(req, res)"],
          convention: "file-based routing: /api/users -> pages/api/users.js",
          confidence: 0.95,
        },
        {
          name: "nextjs.app-router",
          files: "app/api/**/route.{js,ts}",
          routes: ["export async function {method}()"],
          convention: "file-based routing: /api/users -> app/api/users/route.js",
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
