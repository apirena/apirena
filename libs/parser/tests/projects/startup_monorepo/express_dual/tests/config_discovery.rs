use crate::projects::fixtures;
use hallwatch_parser::config::ConfigDiscovery;
use std::fs;

#[tokio::test]
async fn discovers_dual_express_apps() {
    let project_path = fixtures::express_dual_monorepo();
    
    // Clean up any existing config files to ensure test isolation
    let config_path = project_path.join(".hallwatch/discovered.config.js");
    if config_path.exists() {
        let _ = fs::remove_file(&config_path);
    }
    
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Verify project structure detection
    assert_eq!(config.project_structure.project_type, "monorepo");
    assert!(config.project_structure.file_count > 5);
    
    // Verify framework detection - should find 2 Express instances
    let express_detections: Vec<_> = config.frameworks.iter()
        .filter(|f| f.framework == "express")
        .collect();
    
    assert_eq!(express_detections.len(), 2);
    assert!(express_detections.iter().all(|d| d.confidence > 0.9));
    
    // Clean up after test
    let _ = fs::remove_file(&config_path);
}

#[tokio::test]
async fn generates_monorepo_config() {
    let project_path = fixtures::express_dual_monorepo();
    
    // Clean up any existing config files to ensure test isolation
    let config_path = project_path.join(".hallwatch/discovered.config.js");
    if config_path.exists() {
        let _ = fs::remove_file(&config_path);
    }
    
    let discovery = ConfigDiscovery::new(true);
    let _config = discovery.discover(&project_path).await.unwrap();
    
    // Verify config file was created
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path).unwrap();
    assert!(config_content.contains("framework: \"express\""));
    assert!(config_content.contains("debugMode: true"));
    assert!(config_content.contains("_signals:"));
    
    // Clean up after test
    let _ = fs::remove_file(&config_path);
}

#[tokio::test]
async fn detects_different_express_patterns() {
    let project_path = fixtures::express_dual_monorepo();
    
    // Clean up any existing config files to ensure test isolation
    let config_path = project_path.join(".hallwatch/discovered.config.js");
    if config_path.exists() {
        let _ = fs::remove_file(&config_path);
    }
    
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detections: Vec<_> = config.frameworks.iter()
        .filter(|f| f.framework == "express")
        .collect();
    
    // Should detect both router patterns and direct app patterns
    assert!(express_detections.iter().any(|d| 
        d.signals.iter().any(|s| s.source.contains("routes/users.js"))
    ));
    
    assert!(express_detections.iter().any(|d| 
        d.signals.iter().any(|s| s.source.contains("admin/app.js"))
    ));
    
    // Clean up after test
    let _ = fs::remove_file(&config_path);
}
