use crate::projects::fixtures;
use pinpath_parser::config::ConfigDiscovery;
use std::fs;

#[tokio::test]
async fn full_discovery_pipeline_works() {
    let project_path = fixtures::express_dual_monorepo();
    
    // Clean up any existing config files to ensure test isolation
    let config_path = project_path.join(".pinpath/discovered.config.js");
    if config_path.exists() {
        let _ = fs::remove_file(&config_path);
    }
    
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Complete workflow: detect frameworks + generate config
    assert!(config.frameworks.len() >= 2);
    
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path).unwrap();
    assert!(config_content.contains("express"));
    
    // Clean up after test
    let _ = fs::remove_file(&config_path);
}

#[tokio::test]
async fn handles_monorepo_structure() {
    let project_path = fixtures::express_dual_monorepo();
    
    // Clean up any existing config files to ensure test isolation
    let config_path = project_path.join(".pinpath/discovered.config.js");
    if config_path.exists() {
        let _ = fs::remove_file(&config_path);
    }
    
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Should correctly identify as monorepo
    assert_eq!(config.project_structure.project_type, "monorepo");
    
    // Should handle workspace structure
    assert!(config.project_structure.file_count > 5);
    
    // Clean up after test
    let _ = fs::remove_file(&config_path);
}
