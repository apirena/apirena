use super::fixtures;
use reqsmith_parser::config::ConfigDiscovery;

#[tokio::test]
async fn detects_laravel_basic() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Laravel detection tests will be implemented here
    assert!(!config.frameworks.is_empty());
}
