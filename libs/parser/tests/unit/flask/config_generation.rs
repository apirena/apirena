// Flask unit tests - config generation module
use super::fixtures;
use pinpath_parser::config::ConfigDiscovery;
use std::fs;

#[tokio::test]
async fn generates_flask_config() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(false);
    let _config = discovery.discover(&project_path).await.unwrap();
    
    // Check that config file was created
    let config_path = project_path.join(".pinpath/discovered.config.js");
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path).unwrap();
    assert!(config_content.contains("framework: \"flask\""));
}
