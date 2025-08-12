use super::fixtures;
use pinpath_parser::config::ConfigDiscovery;

#[tokio::test]
async fn detects_basic_routes() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detection = config.frameworks.iter()
        .find(|f| f.framework == "express")
        .unwrap();
    
    // Should find Express route patterns
    let pattern_names: std::collections::HashSet<_> = express_detection.patterns.iter()
        .map(|p| p.name.as_str())
        .collect();
    
    assert!(pattern_names.contains("express.basic-routes"));
}

#[tokio::test]
async fn detects_get_routes() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detection = config.frameworks.iter()
        .find(|f| f.framework == "express")
        .unwrap();
    
    // Should detect routes in the app.js file
    assert!(express_detection.signals.iter().any(|s| s.source.contains("app.js")));
}
