use super::fixtures;
use hallwatch_parser::config::ConfigDiscovery;

#[tokio::test]
async fn matches_nextjs_app_router_patterns() {
    let project_path = fixtures::app_router();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let nextjs_detection = config.frameworks.iter()
        .find(|f| f.framework == "nextjs")
        .unwrap();
    
    // Should find Next.js App Router patterns
    assert!(nextjs_detection.patterns.len() > 0);
}
