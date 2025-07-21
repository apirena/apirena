// Flask unit tests - patterns module
use super::fixtures;
use hallwatch_parser::config::ConfigDiscovery;

#[tokio::test]
async fn detects_flask_route_patterns() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let flask_detection = config.frameworks.iter()
        .find(|f| f.framework == "flask")
        .unwrap();
    
    // Should find Flask route patterns
    assert!(flask_detection.patterns.len() > 0);
}
