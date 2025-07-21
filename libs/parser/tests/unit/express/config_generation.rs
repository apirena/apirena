use super::fixtures;
use hallwatch_parser::config::ConfigDiscovery;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn generates_valid_config() {
    let temp_dir = TempDir::new().unwrap();
    
    // Copy fixture files to temp directory
    let package_json_content = fs::read_to_string(fixtures::basic_app().join("package.json")).unwrap();
    let app_js_content = fs::read_to_string(fixtures::basic_app().join("app.js")).unwrap();
    
    fs::write(temp_dir.path().join("package.json"), package_json_content).unwrap();
    fs::write(temp_dir.path().join("app.js"), app_js_content).unwrap();
    
    let discovery = ConfigDiscovery::new(false);
    let _config = discovery.discover(temp_dir.path()).await.unwrap();
    
    // Check that config file was created
    let config_path = temp_dir.path().join(".hallwatch/discovered.config.js");
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path).unwrap();
    assert!(config_content.contains("framework: \"express\""));
}

#[tokio::test]
async fn includes_correct_patterns() {
    let temp_dir = TempDir::new().unwrap();
    
    // Copy fixture files to temp directory
    let package_json_content = fs::read_to_string(fixtures::basic_app().join("package.json")).unwrap();
    let app_js_content = fs::read_to_string(fixtures::basic_app().join("app.js")).unwrap();
    
    fs::write(temp_dir.path().join("package.json"), package_json_content).unwrap();
    fs::write(temp_dir.path().join("app.js"), app_js_content).unwrap();
    
    let discovery = ConfigDiscovery::new(true);
    let _config = discovery.discover(temp_dir.path()).await.unwrap();
    
    let config_path = temp_dir.path().join(".hallwatch/discovered.config.js");
    let config_content = fs::read_to_string(&config_path).unwrap();
    
    // Should include Express-specific patterns
    assert!(config_content.contains("express"));
    assert!(config_content.contains("debugMode: true"));
}

#[tokio::test]
async fn preserves_debug_info() {
    let temp_dir = TempDir::new().unwrap();
    
    // Copy fixture files to temp directory
    let package_json_content = fs::read_to_string(fixtures::basic_app().join("package.json")).unwrap();
    let app_js_content = fs::read_to_string(fixtures::basic_app().join("app.js")).unwrap();
    
    fs::write(temp_dir.path().join("package.json"), package_json_content).unwrap();
    fs::write(temp_dir.path().join("app.js"), app_js_content).unwrap();
    
    let discovery = ConfigDiscovery::new(true);
    let _config = discovery.discover(temp_dir.path()).await.unwrap();
    
    let config_path = temp_dir.path().join(".hallwatch/discovered.config.js");
    let config_content = fs::read_to_string(&config_path).unwrap();
    
    // Debug mode should include signal information
    assert!(config_content.contains("_signals:"));
    assert!(config_content.contains("_meta:"));
}

#[tokio::test]
async fn generates_clean_production_config() {
    let temp_dir = TempDir::new().unwrap();
    
    // Copy fixture files to temp directory
    let package_json_content = fs::read_to_string(fixtures::basic_app().join("package.json")).unwrap();
    let app_js_content = fs::read_to_string(fixtures::basic_app().join("app.js")).unwrap();
    
    fs::write(temp_dir.path().join("package.json"), package_json_content).unwrap();
    fs::write(temp_dir.path().join("app.js"), app_js_content).unwrap();
    
    let discovery = ConfigDiscovery::new(false);
    let _config = discovery.discover(temp_dir.path()).await.unwrap();
    
    let config_path = temp_dir.path().join(".hallwatch/discovered.config.js");
    let config_content = fs::read_to_string(&config_path).unwrap();
    
    // Production mode should not include debug info
    assert!(!config_content.contains("_signals:"));
    assert!(config_content.contains("debugMode: false"));
}
