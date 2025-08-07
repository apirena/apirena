use super::fixtures;
use reqsmith_parser::config::ConfigDiscovery;

#[tokio::test]
async fn detects_express_basic() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    assert_eq!(config.frameworks.len(), 1);
    assert_eq!(config.frameworks[0].framework, "express");
    assert!(config.frameworks[0].confidence > 0.9);
}

#[tokio::test]
async fn finds_package_json_signal() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(true); // Debug mode to get signals
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detection = config.frameworks.iter()
        .find(|f| f.framework == "express")
        .unwrap();
    
    let signal_types: std::collections::HashSet<_> = express_detection.signals.iter()
        .map(|s| s.signal_type.as_str())
        .collect();
    
    assert!(signal_types.contains("package.json"));
}

#[tokio::test]
async fn finds_code_patterns() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detection = config.frameworks.iter()
        .find(|f| f.framework == "express")
        .unwrap();
    
    let signal_types: std::collections::HashSet<_> = express_detection.signals.iter()
        .map(|s| s.signal_type.as_str())
        .collect();
    
    assert!(signal_types.contains("code_pattern"));
}

#[tokio::test]
async fn calculates_high_confidence_for_clear_express() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detection = config.frameworks.iter()
        .find(|f| f.framework == "express")
        .unwrap();
    
    // Should have very high confidence with both package.json and code patterns
    assert!(express_detection.confidence > 0.95);
}
