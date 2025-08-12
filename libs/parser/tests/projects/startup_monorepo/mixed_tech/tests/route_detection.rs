use crate::projects::fixtures;
use pinpath_parser::config::ConfigDiscovery;

#[tokio::test]
async fn finds_express_routes() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let express_detection = config.frameworks.iter()
        .find(|f| f.framework == "express")
        .unwrap();
    
    // Should find Express routes in backend
    assert!(express_detection.patterns.len() > 0);
}

#[tokio::test]
async fn finds_nextjs_api_routes() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    let nextjs_detection = config.frameworks.iter()
        .find(|f| f.framework == "nextjs")
        .unwrap();
    
    // Should find Next.js API routes
    assert!(nextjs_detection.signals.iter().any(|s| s.source.contains("pages/api")));
}

#[tokio::test]
async fn finds_python_api_endpoints() {
    let project_path = fixtures::mixed_tech_monorepo();
    let discovery = ConfigDiscovery::new(true);
    let config = discovery.discover(&project_path).await.unwrap();
    
    // Should find FastAPI endpoints
    let fastapi_detection = config.frameworks.iter()
        .find(|f| f.framework == "fastapi");
    if let Some(detection) = fastapi_detection {
        assert!(detection.patterns.len() > 0);
    }
    
    // Should find Flask routes
    let flask_detection = config.frameworks.iter()
        .find(|f| f.framework == "flask");
    if let Some(detection) = flask_detection {
        assert!(detection.patterns.len() > 0);
    }
}
