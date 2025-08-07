use super::fixtures;
use reqsmith_parser::config::ConfigDiscovery;

#[tokio::test]
async fn detects_nextjs_app_router() {
    let project_path = fixtures::app_router();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    assert_eq!(config.frameworks.len(), 1);
    assert_eq!(config.frameworks[0].framework, "nextjs");
    assert!(config.frameworks[0].confidence > 0.9);
}

#[tokio::test]
async fn finds_package_json_and_config_signals() {
    let project_path = fixtures::app_router();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let nextjs_detection = config.frameworks.iter()
        .find(|f| f.framework == "nextjs")
        .unwrap();
    
    let signal_types: std::collections::HashSet<_> = nextjs_detection.signals.iter()
        .map(|s| s.signal_type.as_str())
        .collect();
    
    assert!(signal_types.contains("package.json"));
    assert!(signal_types.contains("config_file"));
}

#[tokio::test]
async fn detects_app_router_pattern() {
    let project_path = fixtures::app_router();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let nextjs_detection = config.frameworks.iter()
        .find(|f| f.framework == "nextjs")
        .unwrap();
    
    let pattern_names: std::collections::HashSet<_> = nextjs_detection.patterns.iter()
        .map(|p| p.name.as_str())
        .collect();
    
    assert!(pattern_names.contains("nextjs.app-router"));
}

#[tokio::test]
async fn high_confidence_with_multiple_signals() {
    let project_path = fixtures::app_router();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let nextjs_detection = config.frameworks.iter()
        .find(|f| f.framework == "nextjs")
        .unwrap();
    
    // Should have very high confidence with package.json, config file, and app directory
    assert!(nextjs_detection.confidence > 0.95);
}
