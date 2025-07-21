use crate::projects::fixtures;
use hallwatch_parser::config::ConfigDiscovery;
use std::time::Instant;

#[tokio::test]
async fn handles_multi_language_project_efficiently() {
    let project_path = fixtures::mixed_tech_monorepo();
    
    let start = Instant::now();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    let duration = start.elapsed();
    
    // Should complete reasonably quickly despite multiple languages
    assert!(duration.as_millis() < 2000);
    
    // Should detect all frameworks
    assert_eq!(config.frameworks.len(), 4);
}

#[tokio::test]
async fn uses_appropriate_performance_settings() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Should use reasonable settings for medium-sized project
    assert!(config.performance.cache_strategy == "standard" || 
            config.performance.cache_strategy == "minimal");
}
