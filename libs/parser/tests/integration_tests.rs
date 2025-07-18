use apirena_parser::{
    languages::{javascript::JavaScriptParser, python::PythonParser, php::PhpParser},
    LanguageParser, HttpMethod,
};
use std::path::Path;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn test_javascript_express_basic_routes() {
    let parser = JavaScriptParser;
    let fixture_path = Path::new(FIXTURES_DIR).join("javascript/express/basic-routes.js");
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");
    
    let endpoints = parser.parse(&content).expect("Failed to parse JavaScript");
    
    assert_eq!(endpoints.len(), 8, "Expected 8 endpoints in basic-routes.js");
    
    // Check specific endpoints
    let get_users = endpoints.iter().find(|e| e.path == "/users" && matches!(e.method, HttpMethod::Get));
    assert!(get_users.is_some(), "Should find GET /users endpoint");
    
    let post_users = endpoints.iter().find(|e| e.path == "/users" && matches!(e.method, HttpMethod::Post));
    assert!(post_users.is_some(), "Should find POST /users endpoint");
}

#[test]
fn test_javascript_express_nested_routers() {
    let parser = JavaScriptParser;
    let fixture_path = Path::new(FIXTURES_DIR).join("javascript/express/nested-routers.js");
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");
    
    let endpoints = parser.parse(&content).expect("Failed to parse JavaScript");
    
    assert_eq!(endpoints.len(), 12, "Expected 12 endpoints in nested-routers.js");
    
    // Check for health endpoint
    let health = endpoints.iter().find(|e| e.path == "/health");
    assert!(health.is_some(), "Should find /health endpoint");
}

#[test]
fn test_python_flask_basic_routes() {
    let parser = PythonParser;
    let fixture_path = Path::new(FIXTURES_DIR).join("python/flask/basic-routes.py");
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");
    
    let endpoints = parser.parse(&content).expect("Failed to parse Python");
    
    assert!(endpoints.len() >= 10, "Expected at least 10 endpoints in Flask basic routes");
    
    // Check for user endpoints
    let get_users = endpoints.iter().find(|e| e.path == "/users" && matches!(e.method, HttpMethod::Get));
    assert!(get_users.is_some(), "Should find GET /users endpoint");
}

#[test]  
fn test_python_fastapi_basic_routes() {
    let parser = PythonParser;
    let fixture_path = Path::new(FIXTURES_DIR).join("python/fastapi/basic-routes.py");
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");
    
    let endpoints = parser.parse(&content).expect("Failed to parse Python");
    
    assert_eq!(endpoints.len(), 13, "Expected 13 endpoints in FastAPI basic routes");
    
    // Check for parameterized routes
    let user_by_id = endpoints.iter().find(|e| e.path == "/users/{user_id}");
    assert!(user_by_id.is_some(), "Should find parameterized user route");
}

#[test]
fn test_php_laravel_basic_routes() {
    let parser = PhpParser::new().expect("Failed to create PHP parser");
    let fixture_path = Path::new(FIXTURES_DIR).join("php/laravel/basic-routes.php");
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");
    
    let endpoints = parser.parse(&content).expect("Failed to parse PHP");
    
    assert_eq!(endpoints.len(), 24, "Expected 24 endpoints in Laravel basic routes");
    
    // Check for Laravel-style routes
    let get_users = endpoints.iter().find(|e| e.path == "/users" && matches!(e.method, HttpMethod::Get));
    assert!(get_users.is_some(), "Should find GET /users endpoint");
    
    // Check for parameterized routes
    let user_by_id = endpoints.iter().find(|e| e.path == "/users/{id}");
    assert!(user_by_id.is_some(), "Should find parameterized user route");
}

#[test]
fn test_php_laravel_controllers() {
    let parser = PhpParser::new().expect("Failed to create PHP parser");
    let fixture_path = Path::new(FIXTURES_DIR).join("php/laravel/controllers.php");
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");
    
    let endpoints = parser.parse(&content).expect("Failed to parse PHP");
    
    assert_eq!(endpoints.len(), 30, "Expected 30 endpoints in Laravel controllers");
    
    // Check for admin routes
    let dashboard = endpoints.iter().find(|e| e.path == "/dashboard");
    assert!(dashboard.is_some(), "Should find dashboard endpoint");
}

#[test]
fn test_php_laravel_api_routes() {
    let parser = PhpParser::new().expect("Failed to create PHP parser");
    let fixture_path = Path::new(FIXTURES_DIR).join("php/laravel/api-routes.php");
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");
    
    let endpoints = parser.parse(&content).expect("Failed to parse PHP");
    
    assert_eq!(endpoints.len(), 36, "Expected 36 endpoints in Laravel API routes");
    
    // Check for auth endpoints
    let login = endpoints.iter().find(|e| e.path == "/login" && matches!(e.method, HttpMethod::Post));
    assert!(login.is_some(), "Should find POST /login endpoint");
    
    // Check for resource routes
    let posts = endpoints.iter().filter(|e| e.path.starts_with("/posts")).count();
    assert!(posts >= 5, "Should find multiple post-related endpoints");
}

#[test] 
fn test_performance_large_file() {
    let parser = JavaScriptParser;
    let fixture_path = Path::new(FIXTURES_DIR).join("javascript/express/nested-routers.js");
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");
    
    let start = std::time::Instant::now();
    let endpoints = parser.parse(&content).expect("Failed to parse JavaScript");
    let duration = start.elapsed();
    
    assert!(!endpoints.is_empty(), "Should find endpoints");
    assert!(duration.as_millis() < 50, "Parse time should be under 50ms, got {}ms", duration.as_millis());
}

#[test]
fn test_empty_file_handling() {
    let parser = JavaScriptParser;
    let endpoints = parser.parse("").expect("Should handle empty content");
    assert!(endpoints.is_empty(), "Empty file should yield no endpoints");
}

#[test]
fn test_malformed_code_handling() {
    let parser = JavaScriptParser;
    let malformed_js = "app.get('/test' // missing closing parenthesis and semicolon";
    
    // Should not panic, may return empty results
    let result = parser.parse(malformed_js);
    match result {
        Ok(endpoints) => {
            // It's ok if we get no endpoints from malformed code
            assert!(endpoints.len() <= 1, "Malformed code shouldn't produce many endpoints");
        }
        Err(_) => {
            // It's also ok if parsing fails gracefully
        }
    }
}
