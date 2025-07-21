use crate::projects::fixtures;
use hallwatch_parser::config::ConfigDiscovery;
use std::fs;

#[tokio::test]
async fn complete_discovery_workflow() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Complete workflow: detect all frameworks + generate config
    assert_eq!(config.frameworks.len(), 4);
    
    let config_path = project_path.join(".hallwatch/discovered.config.js");
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path).unwrap();
    assert!(config_content.contains("express"));
    assert!(config_content.contains("nextjs"));
    assert!(config_content.contains("fastapi"));
    assert!(config_content.contains("flask"));
}

#[tokio::test]
async fn handles_polyglot_structure() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Should handle multiple languages gracefully
    assert!(config.project_structure.file_count > 8);
    
    // Should correctly identify project type
    assert_eq!(config.project_structure.project_type, "monorepo");
}
