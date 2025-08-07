use crate::projects::fixtures;
use reqsmith_parser::config::ConfigDiscovery;
use std::fs;

#[tokio::test]
async fn discovers_all_four_frameworks() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Should detect all 4 frameworks
    assert_eq!(config.frameworks.len(), 4);
    
    let frameworks: std::collections::HashSet<_> = config.frameworks.iter()
        .map(|f| f.framework.as_str())
        .collect();
    
    assert!(frameworks.contains("express"));
    assert!(frameworks.contains("nextjs"));
    assert!(frameworks.contains("fastapi"));
    assert!(frameworks.contains("flask"));
}

#[tokio::test]
async fn all_frameworks_have_good_confidence() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Verify each has reasonable confidence
    assert!(config.frameworks.iter().all(|f| f.confidence > 0.7));
}

#[tokio::test]
async fn generates_clean_production_config() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(false); // Non-debug mode
    let _config = discovery.discover(&project_path).await.unwrap();
    
    // In non-debug mode, signals should not be included in JS output
    let config_path = project_path.join(".reqsmith/discovered.config.js");
    let config_content = fs::read_to_string(&config_path).unwrap();
    assert!(!config_content.contains("_signals:"));
    assert!(config_content.contains("debugMode: false"));
}

#[tokio::test]
async fn detects_javascript_and_python_frameworks() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Should detect JavaScript frameworks
    let js_frameworks: Vec<_> = config.frameworks.iter()
        .filter(|f| f.framework == "express" || f.framework == "nextjs")
        .collect();
    assert_eq!(js_frameworks.len(), 2);
    
    // Should detect Python frameworks
    let py_frameworks: Vec<_> = config.frameworks.iter()
        .filter(|f| f.framework == "fastapi" || f.framework == "flask")
        .collect();
    assert_eq!(py_frameworks.len(), 2);
}
