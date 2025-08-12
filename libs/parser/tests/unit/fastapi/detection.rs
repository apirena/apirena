use super::fixtures;
use pinpath_parser::config::ConfigDiscovery;

#[tokio::test]
async fn detects_fastapi_basic() {
    let project_path = fixtures::basic_app();
    let discovery = ConfigDiscovery::new(false);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // FastAPI detection tests will be implemented here
    assert!(!config.frameworks.is_empty());
}
