use crate::{Endpoint, HttpMethod, LanguageParser};
use anyhow::{Result, anyhow};
use tree_sitter::{Parser, Query, QueryCursor};

pub struct PythonParser;

impl LanguageParser for PythonParser {
    fn parse(&self, content: &str) -> Result<Vec<Endpoint>> {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_python::language())?;
        
        let tree = parser.parse(content, None)
            .ok_or_else(|| anyhow!("Failed to parse Python code"))?;
        
        let mut endpoints = Vec::new();
        
        // Query for Flask route decorators with methods parameter
        let flask_methods_query = Query::new(
            tree_sitter_python::language(),
            r#"
            (decorated_definition
              (decorator 
                (call
                  function: (attribute
                    object: (identifier) @app
                    attribute: (identifier) @decorator_name)
                  arguments: (argument_list
                    (string (string_content) @path)
                    (keyword_argument
                      name: (identifier) @param_name
                      value: (list (string (string_content) @http_method)*)))))
              definition: (function_definition
                name: (identifier) @handler))
            "#,
        )?;

        // Query for simple route decorators (app.get, app.post, etc.)
        let simple_route_query = Query::new(
            tree_sitter_python::language(),
            r#"
            (decorated_definition
              (decorator 
                (call
                  function: (attribute
                    object: (identifier) @app
                    attribute: (identifier) @method)
                  arguments: (argument_list
                    (string (string_content) @path))))
              definition: (function_definition
                name: (identifier) @handler))
            "#,
        )?;

        let mut cursor = QueryCursor::new();
        
        // Parse routes with methods parameter
        let matches = cursor.matches(&flask_methods_query, tree.root_node(), content.as_bytes());
        for m in matches {
            let mut path_value = None;
            let mut handler_name = None;
            let mut http_methods = Vec::new();
            let mut start_position = None;

            for capture in m.captures {
                let text = &content[capture.node.byte_range()];
                let capture_name = &flask_methods_query.capture_names()[capture.index as usize];
                match capture_name.as_str() {
                    "path" => {
                        path_value = Some(text.to_string());
                        start_position = Some(capture.node.start_position());
                    }
                    "handler" => {
                        handler_name = Some(text.to_string());
                    }
                    "http_method" => {
                        http_methods.push(text.to_string());
                    }
                    _ => {}
                }
            }

            if let (Some(path), Some(handler), Some(pos)) = (path_value, handler_name, start_position) {
                if http_methods.is_empty() {
                    // Default to GET if no methods specified
                    endpoints.push(Endpoint {
                        method: HttpMethod::Get,
                        path,
                        handler: handler.clone(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        documentation: None,
                    });
                } else {
                    for method_str in http_methods {
                        if let Ok(http_method) = parse_http_method(&method_str) {
                            endpoints.push(Endpoint {
                                method: http_method,
                                path: path.clone(),
                                handler: handler.clone(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                documentation: None,
                            });
                        }
                    }
                }
            }
        }

        // Parse simple route decorators
        let matches = cursor.matches(&simple_route_query, tree.root_node(), content.as_bytes());
        for m in matches {
            let mut method_name = None;
            let mut path_value = None;
            let mut handler_name = None;
            let mut start_position = None;

            for capture in m.captures {
                let text = &content[capture.node.byte_range()];
                let capture_name = &simple_route_query.capture_names()[capture.index as usize];
                match capture_name.as_str() {
                    "method" => {
                        method_name = Some(text.to_string());
                        start_position = Some(capture.node.start_position());
                    }
                    "path" => {
                        path_value = Some(text.to_string());
                    }
                    "handler" => {
                        handler_name = Some(text.to_string());
                    }
                    _ => {}
                }
            }

            if let (Some(method), Some(path), Some(handler), Some(pos)) = 
                (method_name, path_value, handler_name, start_position) {
                if let Ok(http_method) = parse_http_method(&method) {
                    endpoints.push(Endpoint {
                        method: http_method,
                        path,
                        handler,
                        line: pos.row + 1,
                        column: pos.column + 1,
                        documentation: None,
                    });
                }
            }
        }

        Ok(endpoints)
    }

    fn supports_extension(&self, extension: &str) -> bool {
        extension == "py"
    }
}

fn parse_http_method(method_str: &str) -> Result<HttpMethod> {
    match method_str.to_lowercase().as_str() {
        "get" | "route" => Ok(HttpMethod::Get), // Flask uses 'route', FastAPI uses 'get'
        "post" => Ok(HttpMethod::Post),
        "put" => Ok(HttpMethod::Put),
        "delete" => Ok(HttpMethod::Delete),
        "patch" => Ok(HttpMethod::Patch),
        "head" => Ok(HttpMethod::Head),
        "options" => Ok(HttpMethod::Options),
        _ => Err(anyhow!("Unknown HTTP method: {}", method_str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_python_extension() {
        let parser = PythonParser;
        assert!(parser.supports_extension("py"));
        assert!(!parser.supports_extension("js"));
    }

    #[test]
    fn test_parse_empty_content() {
        let parser = PythonParser;
        let result = parser.parse("").unwrap();
        assert_eq!(result.len(), 0);
    }
}
