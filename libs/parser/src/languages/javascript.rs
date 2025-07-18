use crate::{Endpoint, HttpMethod, LanguageParser};
use anyhow::{Result, anyhow};
use tree_sitter::{Parser, Query, QueryCursor};

pub struct JavaScriptParser;

impl LanguageParser for JavaScriptParser {
    fn parse(&self, content: &str) -> Result<Vec<Endpoint>> {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_javascript::language())?;
        
        let tree = parser.parse(content, None)
            .ok_or_else(|| anyhow!("Failed to parse JavaScript code"))?;
        
        let mut endpoints = Vec::new();
        
        // Query for Express.js style route definitions
        let express_query = Query::new(
            tree_sitter_javascript::language(),
            r#"
            (call_expression
              function: (member_expression
                object: (identifier) @app
                property: (property_identifier) @method)
              arguments: (arguments 
                (string (string_fragment) @path)
                . 
                (arrow_function)?)
            )
            "#,
        )?;

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&express_query, tree.root_node(), content.as_bytes());

        for m in matches {
            let mut method_name = None;
            let mut path_value = None;
            let mut start_position = None;

            for capture in m.captures {
                let text = &content[capture.node.byte_range()];
                let capture_name = &express_query.capture_names()[capture.index as usize];
                match capture_name.as_str() {
                    "method" => {
                        method_name = Some(text.to_string());
                        start_position = Some(capture.node.start_position());
                    }
                    "path" => {
                        path_value = Some(text.to_string());
                    }
                    _ => {}
                }
            }

            if let (Some(method), Some(path), Some(pos)) = (method_name, path_value, start_position) {
                if let Ok(http_method) = parse_http_method(&method) {
                    endpoints.push(Endpoint {
                        method: http_method,
                        path,
                        handler: format!("{}:{}", pos.row + 1, pos.column + 1),
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
        matches!(extension, "js" | "mjs" | "ts" | "tsx")
    }
}

fn parse_http_method(method_str: &str) -> Result<HttpMethod> {
    match method_str.to_lowercase().as_str() {
        "get" => Ok(HttpMethod::Get),
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
    fn test_supports_javascript_extensions() {
        let parser = JavaScriptParser;
        assert!(parser.supports_extension("js"));
        assert!(parser.supports_extension("ts"));
        assert!(parser.supports_extension("tsx"));
        assert!(!parser.supports_extension("py"));
    }

    #[test]
    fn test_parse_empty_content() {
        let parser = JavaScriptParser;
        let result = parser.parse("").unwrap();
        assert_eq!(result.len(), 0);
    }
}
