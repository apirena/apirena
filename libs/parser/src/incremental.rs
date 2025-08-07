use crate::{Endpoint, LanguageParser};
use anyhow::Result;
use reqsmith_diff::{ChangeEvent, CodeRegion, FileDiff};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Manages incremental parsing with state persistence
pub struct IncrementalParser {
    state: EndpointState,
    language_parsers: HashMap<String, Box<dyn LanguageParser>>,
}

/// Persistent state of discovered endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointState {
    pub endpoints: HashMap<String, Endpoint>,  // Key: unique endpoint ID
    pub file_hashes: HashMap<PathBuf, String>, // Track file content versions
    pub last_updated: SystemTime,
}

/// Result of incremental parsing showing what changed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointChanges {
    pub added: Vec<Endpoint>,
    pub modified: Vec<EndpointChange>,
    pub removed: Vec<Endpoint>,
    pub unchanged: Vec<Endpoint>,
}

/// Details about a specific endpoint change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointChange {
    pub old: Endpoint,
    pub new: Endpoint,
    pub change_type: ChangeType,
}

/// Type of change detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    PathChanged,
    MethodChanged,
    ParametersChanged,
    HandlerChanged,
    MiddlewareChanged,
    LineChanged,
}

impl IncrementalParser {
    pub fn new() -> Self {
        let mut language_parsers: HashMap<String, Box<dyn LanguageParser>> = HashMap::new();
        
        // Initialize language parsers
        language_parsers.insert("javascript".to_string(), Box::new(crate::languages::javascript::JavaScriptParser));
        language_parsers.insert("typescript".to_string(), Box::new(crate::languages::javascript::JavaScriptParser));
        language_parsers.insert("python".to_string(), Box::new(crate::languages::python::PythonParser));
        
        // Add PHP parser - need to handle the Result
        if let Ok(php_parser) = crate::languages::php::PhpParser::new() {
            language_parsers.insert("php".to_string(), Box::new(php_parser));
        }

        Self {
            state: EndpointState {
                endpoints: HashMap::new(),
                file_hashes: HashMap::new(),
                last_updated: SystemTime::now(),
            },
            language_parsers,
        }
    }

    /// Load state from a previous session
    pub fn with_state(state: EndpointState) -> Self {
        let mut parser = Self::new();
        parser.state = state;
        parser
    }

    /// Process change events and return what endpoints changed
    pub async fn parse_changes(&mut self, change_event: ChangeEvent) -> Result<EndpointChanges> {
        let mut all_changes = EndpointChanges::new();

        for file_diff in change_event.diffs {
            let changes = self.parse_file_diff(&file_diff).await?;
            all_changes.merge(changes);
        }

        self.state.last_updated = SystemTime::now();
        Ok(all_changes)
    }

    /// Parse a specific file diff and detect endpoint changes
    async fn parse_file_diff(&mut self, file_diff: &FileDiff) -> Result<EndpointChanges> {
        let language = crate::detect_language(&file_diff.path);
        
        let parser = match language {
            Some(lang) => self.language_parsers.get(lang),
            None => return Ok(EndpointChanges::new()), // Skip unsupported files
        };

        let parser = match parser {
            Some(p) => p,
            None => return Ok(EndpointChanges::new()),
        };

        // Get old endpoints for this file
        let old_endpoints = self.get_endpoints_for_file(&file_diff.path);
        
        // Parse new content
        let new_endpoints = parser.parse(&file_diff.new_content)?;
        
        // Update file hash
        let new_hash = self.calculate_content_hash(&file_diff.new_content);
        self.state.file_hashes.insert(file_diff.path.clone(), new_hash);

        // Compare and generate changes
        let changes = self.compare_endpoints(old_endpoints, new_endpoints, &file_diff.path);
        
        // Update state with new endpoints
        self.update_file_endpoints(&file_diff.path, &changes);
        
        Ok(changes)
    }

    /// Parse only specific code regions (for performance)
    pub fn parse_regions(&self, regions: Vec<CodeRegion>, path: &Path) -> Result<Vec<Endpoint>> {
        let language = crate::detect_language(path);
        
        let parser = match language {
            Some(lang) => self.language_parsers.get(lang),
            None => return Ok(vec![]),
        };

        let parser = match parser {
            Some(p) => p,
            None => return Ok(vec![]),
        };

        let mut all_endpoints = Vec::new();
        
        for region in regions {
            if region.has_changes {
                let endpoints = parser.parse(&region.content)?;
                
                // Adjust line numbers based on region start
                let adjusted_endpoints: Vec<Endpoint> = endpoints
                    .into_iter()
                    .map(|mut ep| {
                        ep.line += region.start_line.saturating_sub(1);
                        ep
                    })
                    .collect();
                
                all_endpoints.extend(adjusted_endpoints);
            }
        }

        Ok(all_endpoints)
    }

    /// Get current state for persistence
    pub fn get_state(&self) -> &EndpointState {
        &self.state
    }

    /// Get all current endpoints
    pub fn get_all_endpoints(&self) -> Vec<&Endpoint> {
        self.state.endpoints.values().collect()
    }

    /// Get endpoints for a specific file
    fn get_endpoints_for_file(&self, path: &Path) -> Vec<Endpoint> {
        self.state
            .endpoints
            .values()
            .filter(|ep| self.endpoint_file_matches(ep, path))
            .cloned()
            .collect()
    }

    /// Check if an endpoint belongs to a specific file (simplified for now)
    fn endpoint_file_matches(&self, _endpoint: &Endpoint, _path: &Path) -> bool {
        // TODO: Store file path in endpoint or use a better mapping
        true
    }

    /// Compare old and new endpoints to detect changes
    fn compare_endpoints(
        &self,
        old_endpoints: Vec<Endpoint>,
        new_endpoints: Vec<Endpoint>,
        _path: &Path,
    ) -> EndpointChanges {
        let mut changes = EndpointChanges::new();
        
        // Create lookup maps
        let old_map: HashMap<String, &Endpoint> = old_endpoints
            .iter()
            .map(|ep| (self.generate_endpoint_id(ep), ep))
            .collect();
        
        let new_map: HashMap<String, &Endpoint> = new_endpoints
            .iter()
            .map(|ep| (self.generate_endpoint_id(ep), ep))
            .collect();

        // Find added endpoints
        for (id, endpoint) in &new_map {
            if !old_map.contains_key(id) {
                changes.added.push((*endpoint).clone());
            }
        }

        // Find removed endpoints
        for (id, endpoint) in &old_map {
            if !new_map.contains_key(id) {
                changes.removed.push((*endpoint).clone());
            }
        }

        // Find modified endpoints (same ID but different content)
        for (id, new_endpoint) in &new_map {
            if let Some(old_endpoint) = old_map.get(id) {
                if !self.endpoints_equal(old_endpoint, new_endpoint) {
                    let change_type = self.detect_change_type(old_endpoint, new_endpoint);
                    changes.modified.push(EndpointChange {
                        old: (*old_endpoint).clone(),
                        new: (*new_endpoint).clone(),
                        change_type,
                    });
                } else {
                    changes.unchanged.push((*new_endpoint).clone());
                }
            }
        }

        changes
    }

    /// Generate a unique ID for an endpoint
    fn generate_endpoint_id(&self, endpoint: &Endpoint) -> String {
        format!("{}:{}:{}", 
            format!("{:?}", endpoint.method).to_lowercase(),
            endpoint.path,
            endpoint.line
        )
    }

    /// Check if two endpoints are equal
    fn endpoints_equal(&self, old: &Endpoint, new: &Endpoint) -> bool {
        old.method == new.method
            && old.path == new.path
            && old.handler == new.handler
            && old.line == new.line
            && old.column == new.column
    }

    /// Detect what type of change occurred
    fn detect_change_type(&self, old: &Endpoint, new: &Endpoint) -> ChangeType {
        if old.path != new.path {
            ChangeType::PathChanged
        } else if old.method != new.method {
            ChangeType::MethodChanged
        } else if old.handler != new.handler {
            ChangeType::HandlerChanged
        } else if old.line != new.line || old.column != new.column {
            ChangeType::LineChanged
        } else {
            ChangeType::ParametersChanged // Default assumption
        }
    }

    /// Update state with new endpoints for a file
    fn update_file_endpoints(&mut self, path: &Path, changes: &EndpointChanges) {
        // Remove old endpoints for this file
        let path_str = path.to_string_lossy().to_string();
        self.state.endpoints.retain(|_id, ep| {
            !ep.handler.contains(&path_str) // Simplified file matching
        });

        // Add new endpoints
        for endpoint in &changes.added {
            let id = self.generate_endpoint_id(endpoint);
            self.state.endpoints.insert(id, endpoint.clone());
        }

        for change in &changes.modified {
            let id = self.generate_endpoint_id(&change.new);
            self.state.endpoints.insert(id, change.new.clone());
        }

        for endpoint in &changes.unchanged {
            let id = self.generate_endpoint_id(endpoint);
            self.state.endpoints.insert(id, endpoint.clone());
        }
    }

    /// Calculate SHA256 hash of content
    fn calculate_content_hash(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl EndpointChanges {
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            modified: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }

    /// Merge another set of changes into this one
    pub fn merge(&mut self, other: EndpointChanges) {
        self.added.extend(other.added);
        self.modified.extend(other.modified);
        self.removed.extend(other.removed);
        self.unchanged.extend(other.unchanged);
    }

    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.modified.is_empty() || !self.removed.is_empty()
    }

    /// Get total count of all endpoints
    pub fn total_count(&self) -> usize {
        self.added.len() + self.modified.len() + self.removed.len() + self.unchanged.len()
    }
}

impl Default for IncrementalParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HttpMethod};

    #[test]
    fn test_incremental_parser_creation() {
        let parser = IncrementalParser::new();
        assert!(parser.language_parsers.len() > 0);
    }

    #[test]
    fn test_endpoint_changes_merge() {
        let mut changes1 = EndpointChanges::new();
        changes1.added.push(Endpoint {
            method: HttpMethod::Get,
            path: "/test".to_string(),
            handler: "test".to_string(),
            line: 1,
            column: 1,
            documentation: None,
        });

        let mut changes2 = EndpointChanges::new();
        changes2.added.push(Endpoint {
            method: HttpMethod::Post,
            path: "/test2".to_string(),
            handler: "test2".to_string(),
            line: 2,
            column: 1,
            documentation: None,
        });

        changes1.merge(changes2);
        assert_eq!(changes1.added.len(), 2);
    }

    #[test]
    fn test_generate_endpoint_id() {
        let parser = IncrementalParser::new();
        let endpoint = Endpoint {
            method: HttpMethod::Get,
            path: "/api/users".to_string(),
            handler: "getUsers".to_string(),
            line: 10,
            column: 5,
            documentation: None,
        };

        let id = parser.generate_endpoint_id(&endpoint);
        assert_eq!(id, "get:/api/users:10");
    }
}
