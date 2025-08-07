// Auto-generated Reqsmith configuration
// Generated: 2025-08-07T06:18:10.556576+00:00
// Version: 1.0.0
// Debug Mode: true

export default {
  // Metadata
  _meta: {
    version: "1.0.0",
    generated: "2025-08-07T06:18:10.556576+00:00",
    lastModified: "2025-08-07T06:18:10.556576+00:00",
    debugMode: true,
  },

  // Global configuration
  debugMode: true,

  // Detected project structure
  structure: {
    type: "monorepo",
    fileCount: 18,
    totalSize: 19852,
    roots: ["./frontend", "./backend", "./services"],
  },

  // Detected frameworks
  frameworks: [
    {
      path: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/services/auth",
      framework: "flask",
      confidence: 1,

      // Detection signals (debug mode)
      _signals: [
        { type: "requirements.txt", value: "flask dependency found", confidence: 0.8, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/services/auth/requirements.txt" },
        { type: "code_pattern", value: "Flask(__name__) or @app.route found", confidence: 0.9, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/services/auth/app.py" },
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
    },
    {
      path: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/services/ml",
      framework: "fastapi",
      confidence: 1,

      // Detection signals (debug mode)
      _signals: [
        { type: "requirements.txt", value: "fastapi dependency found", confidence: 0.8, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/services/ml/requirements.txt" },
        { type: "code_pattern", value: "FastAPI() or @app.method decorator found", confidence: 0.9, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/services/ml/main.py" },
      ],

      // Patterns for route detection
      patterns: [
        {
          name: "fastapi.decorators",
          files: "**/*.py",
          routes: ["@app.{method}('{path}')", "@router.{method}('{path}')"],
          confidence: 0.95,
        },
      ],
    },
    {
      path: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/backend",
      framework: "express",
      confidence: 1,

      // Detection signals (debug mode)
      _signals: [
        { type: "package.json", value: "express dependency found", confidence: 0.8, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/backend/package.json" },
        { type: "code_pattern", value: "express() or app.method() found", confidence: 0.9, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/backend/src/app.js" },
      ],

      // Patterns for route detection
      patterns: [
        {
          name: "express.basic-routes",
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
      path: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/frontend",
      framework: "nextjs",
      confidence: 1,

      // Detection signals (debug mode)
      _signals: [
        { type: "package.json", value: "next dependency found", confidence: 0.9, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/frontend/package.json" },
        { type: "config_file", value: "next.config.js found", confidence: 0.8, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/frontend/next.config.js" },
        { type: "directory", value: "pages/api directory found", confidence: 0.85, source: "/Users/isaiahdahl/deployment/reqsmith/libs/parser/tests/projects/startup_monorepo/mixed_tech/frontend/pages/api" },
      ],

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
