use crate::projects::fixtures;
use hallwatch_parser::config::ConfigDiscovery;

#[tokio::test]
async fn finds_api_routes() {
    let project_path = fixtures::express_dual_monorepo();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detections: Vec<_> = config.frameworks.iter()
        .filter(|f| f.framework == "express")
        .collect();
    
    // Should find routes in both services
    assert!(express_detections.len() >= 2);
    
    // Should detect various route patterns
    let all_patterns: Vec<_> = express_detections.iter()
        .flat_map(|d| &d.patterns)
        .collect();
    
    assert!(all_patterns.len() > 0);
}

#[tokio::test]
async fn detects_admin_routes() {
    let project_path = fixtures::express_dual_monorepo();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detections: Vec<_> = config.frameworks.iter()
        .filter(|f| f.framework == "express")
        .collect();
    
    // Should find admin-specific routes
    assert!(express_detections.iter().any(|d| 
        d.signals.iter().any(|s| s.source.contains("admin"))
    ));
}
