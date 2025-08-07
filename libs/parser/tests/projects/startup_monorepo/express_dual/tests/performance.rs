use crate::projects::fixtures;
use reqsmith_parser::config::ConfigDiscovery;
use std::time::Instant;

#[tokio::test]
async fn completes_quickly_for_small_monorepo() {
    let project_path = fixtures::express_dual_monorepo();
    
    let start = Instant::now();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    let duration = start.elapsed();
    
    // Should complete very quickly for small project
    assert!(duration.as_millis() < 1000);
    
    // Performance should be optimized for monorepo
    assert_eq!(config.project_structure.project_type, "monorepo");
}

#[tokio::test]
async fn uses_appropriate_caching_strategy() {
    let project_path = fixtures::express_dual_monorepo();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Should use reasonable caching for small monorepo
    assert!(config.performance.cache_strategy == "minimal" || 
            config.performance.cache_strategy == "standard");
}
