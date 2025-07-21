use super::*;

/// Predefined patterns for common frameworks
pub struct PatternLibrary;

impl PatternLibrary {
    pub fn get_express_patterns() -> Vec<RoutePattern> {
        vec![
            RoutePattern {
                name: "express.app-routes".to_string(),
                files: "**/*.{js,ts}".to_string(),
                routes: vec![
                    "app.{method}('{path}', {handler})".to_string(),
                    "app.{method}('{path}', {middlewares}, {handler})".to_string(),
                ],
                convention: None,
                confidence: 0.95,
            },
            RoutePattern {
                name: "express.router".to_string(),
                files: "**/routes/**/*.{js,ts}".to_string(),
                routes: vec!["router.{method}('{path}', {handler})".to_string()],
                convention: None,
                confidence: 0.90,
            },
        ]
    }

    pub fn get_nextjs_patterns() -> Vec<RoutePattern> {
        vec![
            RoutePattern {
                name: "nextjs.pages-api".to_string(),
                files: "pages/api/**/*.{js,ts}".to_string(),
                routes: vec!["export default function handler(req, res)".to_string()],
                convention: Some("file-based routing: /api/users -> pages/api/users.js".to_string()),
                confidence: 0.95,
            },
            RoutePattern {
                name: "nextjs.app-router".to_string(),
                files: "app/api/**/route.{js,ts}".to_string(),
                routes: vec!["export async function {method}()".to_string()],
                convention: Some("file-based routing: /api/users -> app/api/users/route.js".to_string()),
                confidence: 0.95,
            },
        ]
    }

    pub fn get_fastapi_patterns() -> Vec<RoutePattern> {
        vec![
            RoutePattern {
                name: "fastapi.decorators".to_string(),
                files: "**/*.py".to_string(),
                routes: vec![
                    "@app.{method}('{path}')".to_string(),
                    "@router.{method}('{path}')".to_string(),
                ],
                convention: None,
                confidence: 0.95,
            },
        ]
    }

    pub fn get_flask_patterns() -> Vec<RoutePattern> {
        vec![
            RoutePattern {
                name: "flask.decorators".to_string(),
                files: "**/*.py".to_string(),
                routes: vec!["@app.route('{path}', methods=['{method}'])".to_string()],
                convention: None,
                confidence: 0.95,
            },
        ]
    }

    pub fn get_laravel_patterns() -> Vec<RoutePattern> {
        vec![
            RoutePattern {
                name: "laravel.routes".to_string(),
                files: "routes/**/*.php".to_string(),
                routes: vec!["Route::{method}('{path}', {handler})".to_string()],
                convention: Some("/api/users -> routes/api.php + UserController".to_string()),
                confidence: 0.95,
            },
        ]
    }
}
