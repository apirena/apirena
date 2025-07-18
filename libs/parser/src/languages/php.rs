use crate::{Endpoint, HttpMethod, LanguageParser};
use anyhow::Result;

pub struct PhpParser;

impl PhpParser {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    fn parse_route_method(&self, method_name: &str) -> HttpMethod {
        match method_name.to_lowercase().as_str() {
            "get" => HttpMethod::Get,
            "post" => HttpMethod::Post,
            "put" => HttpMethod::Put,
            "delete" => HttpMethod::Delete,
            "patch" => HttpMethod::Patch,
            "options" => HttpMethod::Options,
            "head" => HttpMethod::Head,
            "any" | "match" | "resource" | "apiresource" => HttpMethod::Get, // Default for complex routes
            _ => HttpMethod::Get,
        }
    }

    fn extract_string_content(&self, text: &str) -> Option<String> {
        // Extract content from quoted strings
        if text.len() >= 2 {
            if (text.starts_with('"') && text.ends_with('"')) || 
               (text.starts_with('\'') && text.ends_with('\'')) {
                let inner = &text[1..text.len() - 1];
                return Some(inner.to_string());
            }
        }
        None
    }
}

impl LanguageParser for PhpParser {
    fn parse(&self, content: &str) -> Result<Vec<Endpoint>> {
        let mut endpoints = Vec::new();

        // Regex for Laravel Route facade calls
        // Pattern: Route::method('path', handler)
        let route_pattern = regex::Regex::new(
            r#"Route::(get|post|put|delete|patch|options|head|any|match|resource|apiResource)\s*\(\s*(['"][^'"]*['"])\s*,([^)]*)\)"#
        ).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            for caps in route_pattern.captures_iter(line) {
                if let (Some(method_match), Some(path_match)) = (caps.get(1), caps.get(2)) {
                    let method_name = method_match.as_str();
                    let path_quoted = path_match.as_str();
                    
                    if let Some(path) = self.extract_string_content(path_quoted) {
                        let normalized_path = if path.starts_with('/') { 
                            path 
                        } else { 
                            format!("/{}", path) 
                        };

                        let handler = caps.get(3)
                            .map(|m| m.as_str().trim().to_string())
                            .unwrap_or_else(|| "anonymous".to_string());

                        endpoints.push(Endpoint {
                            method: self.parse_route_method(method_name),
                            path: normalized_path,
                            handler,
                            line: line_num + 1,
                            column: method_match.start(),
                            documentation: None,
                        });
                    }
                }
            }
        }

        Ok(endpoints)
    }

    fn supports_extension(&self, extension: &str) -> bool {
        extension == "php"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_routes() {
        let parser = PhpParser::new().unwrap();
        
        let code = r#"
<?php

Route::get('/users', 'UserController@index');
Route::post('/users', 'UserController@store');
Route::put('/users/{id}', 'UserController@update');
Route::delete('/users/{id}', 'UserController@destroy');
        "#;

        let endpoints = parser.parse(code).unwrap();
        assert_eq!(endpoints.len(), 4);
        
        assert_eq!(endpoints[0].method, HttpMethod::Get);
        assert_eq!(endpoints[0].path, "/users");
        
        assert_eq!(endpoints[1].method, HttpMethod::Post);
        assert_eq!(endpoints[1].path, "/users");
        
        assert_eq!(endpoints[2].method, HttpMethod::Put);
        assert_eq!(endpoints[2].path, "/users/{id}");
        
        assert_eq!(endpoints[3].method, HttpMethod::Delete);
        assert_eq!(endpoints[3].path, "/users/{id}");
    }

    #[test]
    fn test_parse_closure_routes() {
        let parser = PhpParser::new().unwrap();
        
        let code = r#"
<?php

Route::get('/api/status', function () {
    return response()->json(['status' => 'ok']);
});
        "#;

        let endpoints = parser.parse(code).unwrap();
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].method, HttpMethod::Get);
        assert_eq!(endpoints[0].path, "/api/status");
    }
}
