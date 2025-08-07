use super::fixtures;
use reqsmith_parser::config::ConfigDiscovery;

#[tokio::test]
async fn detects_flask_basic() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    assert_eq!(config.frameworks.len(), 1);
    assert_eq!(config.frameworks[0].framework, "flask");
    assert!(config.frameworks[0].confidence > 0.9);
}

#[tokio::test]
async fn finds_requirements_txt_signal() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let flask_detection = config.frameworks.iter()
        .find(|f| f.framework == "flask")
        .unwrap();
    
    let signal_types: std::collections::HashSet<_> = flask_detection.signals.iter()
        .map(|s| s.signal_type.as_str())
        .collect();
    
    assert!(signal_types.contains("requirements.txt"));
}

#[tokio::test]
async fn finds_flask_import_patterns() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let flask_detection = config.frameworks.iter()
        .find(|f| f.framework == "flask")
        .unwrap();
    
    let signal_types: std::collections::HashSet<_> = flask_detection.signals.iter()
        .map(|s| s.signal_type.as_str())
        .collect();
    
    assert!(signal_types.contains("code_pattern"));
}
