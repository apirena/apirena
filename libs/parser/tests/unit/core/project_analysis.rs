use pinpath_parser::config::ConfigDiscovery;
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn analyzes_project_structure() {
    let temp = TempDir::new().unwrap();
    
    // Create some test files
    fs::write(temp.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
    fs::write(temp.path().join("app.js"), "console.log('test')").unwrap();
    
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(temp.path()).await.unwrap();
    
    // Basic project analysis tests
    assert!(config.project_structure.file_count > 0);
}
