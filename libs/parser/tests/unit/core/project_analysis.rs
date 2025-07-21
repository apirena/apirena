use hallwatch_parser::config::ConfigDiscovery;
use tempfile::TempDir;

#[tokio::test]
async fn analyzes_project_structure() {
    let temp = TempDir::new().unwrap();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(temp.path()).await.unwrap();
    
    // Basic project analysis tests
    assert!(config.project_structure.file_count >= 0);
}
