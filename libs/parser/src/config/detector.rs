use super::*;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FrameworkDetector {
    detectors: Vec<Box<dyn Detector + Send + Sync>>,
}

pub trait Detector: Send + Sync {
    fn name(&self) -> &str;
    fn detect(&self, path: &Path) -> Option<FrameworkDetection>;
}

impl FrameworkDetector {
    pub fn new() -> Self {
        Self {
            detectors: vec![
                Box::new(ExpressDetector),
                Box::new(NextJsDetector),
                Box::new(FastApiDetector),
                Box::new(FlaskDetector),
                Box::new(LaravelDetector),
                Box::new(SpringBootDetector),
                Box::new(ActixDetector),
                Box::new(GinDetector),
            ],
        }
    }

    pub async fn detect_all(&self, root: &Path) -> Result<Vec<FrameworkDetection>> {
        let mut detections = Vec::new();
        
        // Scan root and common subdirectories
        let scan_paths = self.get_scan_paths(root).await?;
        
        for path in scan_paths {
            for detector in &self.detectors {
                if let Some(detection) = detector.detect(&path) {
                    detections.push(detection);
                }
            }
        }
        
        // Remove duplicates and sort by confidence
        detections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        detections.dedup_by(|a, b| a.path == b.path && a.framework == b.framework);
        
        Ok(detections)
    }
    
    async fn get_scan_paths(&self, root: &Path) -> Result<Vec<PathBuf>> {
        let mut paths = vec![root.to_path_buf()];
        
        // Common monorepo patterns
        let common_dirs = [
            "apps", "packages", "services", "libs", "src", 
            "backend", "frontend", "api", "web", "server"
        ];
        
        for dir in common_dirs {
            let dir_path = root.join(dir);
            if dir_path.is_dir() {
                paths.push(dir_path.clone());
                
                // Scan one level deeper for monorepo apps
                if let Ok(entries) = fs::read_dir(&dir_path) {
                    for entry in entries.flatten() {
                        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                            paths.push(entry.path());
                        }
                    }
                }
            }
        }
        
        Ok(paths)
    }
}

// Express.js Detector
struct ExpressDetector;
impl Detector for ExpressDetector {
    fn name(&self) -> &str { "express" }
    
    fn detect(&self, path: &Path) -> Option<FrameworkDetection> {
        let package_json = path.join("package.json");
        if !package_json.exists() { return None; }
        
        let mut signals = Vec::new();
        let mut confidence: f32 = 0.0;
        
        // Check package.json for express dependency
        if let Ok(content) = fs::read_to_string(&package_json) {
            if content.contains("\"express\"") {
                signals.push(DetectionSignal {
                    signal_type: "package.json".to_string(),
                    value: "express dependency found".to_string(),
                    confidence_boost: 0.8,
                    source: package_json.to_string_lossy().to_string(),
                });
                confidence += 0.8;
            }
        }
        
        // Check for app.js or server.js with express patterns
        for filename in ["app.js", "server.js", "index.js", "src/app.js", "src/server.js"] {
            let file_path = path.join(filename);
            if let Ok(content) = fs::read_to_string(&file_path) {
                if content.contains("express()") || content.contains("app.get") || content.contains("app.post") {
                    signals.push(DetectionSignal {
                        signal_type: "code_pattern".to_string(),
                        value: "express() or app.method() found".to_string(),
                        confidence_boost: 0.9,
                        source: file_path.to_string_lossy().to_string(),
                    });
                    confidence += 0.9;
                    break;
                }
            }
        }
        
        if confidence < 0.6 { return None; }
        
        Some(FrameworkDetection {
            path: path.to_string_lossy().to_string(),
            framework: "express".to_string(),
            confidence: confidence.min(1.0),
            signals,
            patterns: vec![
                RoutePattern {
                    name: "express.app-routes".to_string(),
                    files: "**/*.{js,ts}".to_string(),
                    routes: vec![
                        "app.{method}('{path}', {handler})".to_string(),
                        "app.{method}('{path}', {middlewares}, {handler})".to_string(),
                    ],
                    convention: None,
                    confidence: 0.95,
                },
                RoutePattern {
                    name: "express.router".to_string(),
                    files: "**/routes/**/*.{js,ts}".to_string(),
                    routes: vec!["router.{method}('{path}', {handler})".to_string()],
                    convention: None,
                    confidence: 0.90,
                },
            ],
        })
    }
}

// Next.js Detector
struct NextJsDetector;
impl Detector for NextJsDetector {
    fn name(&self) -> &str { "nextjs" }
    
    fn detect(&self, path: &Path) -> Option<FrameworkDetection> {
        let package_json = path.join("package.json");
        let next_config = path.join("next.config.js");
        
        let mut signals = Vec::new();
        let mut confidence: f32 = 0.0;
        
        // Check for Next.js dependency
        if package_json.exists() {
            if let Ok(content) = fs::read_to_string(&package_json) {
                if content.contains("\"next\"") {
                    signals.push(DetectionSignal {
                        signal_type: "package.json".to_string(),
                        value: "next dependency found".to_string(),
                        confidence_boost: 0.9,
                        source: package_json.to_string_lossy().to_string(),
                    });
                    confidence += 0.9;
                }
            }
        }
        
        // Check for next.config.js
        if next_config.exists() {
            signals.push(DetectionSignal {
                signal_type: "config_file".to_string(),
                value: "next.config.js found".to_string(),
                confidence_boost: 0.8,
                source: next_config.to_string_lossy().to_string(),
            });
            confidence += 0.8;
        }
        
        // Check for pages/api or app/api directories
        let pages_api = path.join("pages/api");
        let app_api = path.join("app/api");
        
        if pages_api.is_dir() {
            signals.push(DetectionSignal {
                signal_type: "directory".to_string(),
                value: "pages/api directory found".to_string(),
                confidence_boost: 0.85,
                source: pages_api.to_string_lossy().to_string(),
            });
            confidence += 0.85;
        }
        
        if app_api.is_dir() {
            signals.push(DetectionSignal {
                signal_type: "directory".to_string(),
                value: "app/api directory found (App Router)".to_string(),
                confidence_boost: 0.85,
                source: app_api.to_string_lossy().to_string(),
            });
            confidence += 0.85;
        }
        
        if confidence < 0.7 { return None; }
        
        Some(FrameworkDetection {
            path: path.to_string_lossy().to_string(),
            framework: "nextjs".to_string(),
            confidence: confidence.min(1.0),
            signals,
            patterns: vec![
                RoutePattern {
                    name: "nextjs.pages-api".to_string(),
                    files: "pages/api/**/*.{js,ts}".to_string(),
                    routes: vec!["export default function handler(req, res)".to_string()],
                    convention: Some("file-based routing: /api/users -> pages/api/users.js".to_string()),
                    confidence: 0.95,
                },
                RoutePattern {
                    name: "nextjs.app-router".to_string(),
                    files: "app/api/**/route.{js,ts}".to_string(),
                    routes: vec!["export async function {method}()".to_string()],
                    convention: Some("file-based routing: /api/users -> app/api/users/route.js".to_string()),
                    confidence: 0.95,
                },
            ],
        })
    }
}

// FastAPI Detector
struct FastApiDetector;
impl Detector for FastApiDetector {
    fn name(&self) -> &str { "fastapi" }
    
    fn detect(&self, path: &Path) -> Option<FrameworkDetection> {
        let requirements_txt = path.join("requirements.txt");
        let pyproject_toml = path.join("pyproject.toml");
        
        let mut signals = Vec::new();
        let mut confidence: f32 = 0.0;
        
        // Check requirements.txt
        if let Ok(content) = fs::read_to_string(&requirements_txt) {
            if content.contains("fastapi") {
                signals.push(DetectionSignal {
                    signal_type: "requirements.txt".to_string(),
                    value: "fastapi dependency found".to_string(),
                    confidence_boost: 0.8,
                    source: requirements_txt.to_string_lossy().to_string(),
                });
                confidence += 0.8;
            }
        }
        
        // Check pyproject.toml
        if let Ok(content) = fs::read_to_string(&pyproject_toml) {
            if content.contains("fastapi") {
                signals.push(DetectionSignal {
                    signal_type: "pyproject.toml".to_string(),
                    value: "fastapi dependency found".to_string(),
                    confidence_boost: 0.8,
                    source: pyproject_toml.to_string_lossy().to_string(),
                });
                confidence += 0.8;
            }
        }
        
        // Check for FastAPI code patterns
        for filename in ["main.py", "app.py", "api.py", "src/main.py", "app/main.py"] {
            let file_path = path.join(filename);
            if let Ok(content) = fs::read_to_string(&file_path) {
                if content.contains("FastAPI()") || content.contains("@app.get") || content.contains("@app.post") {
                    signals.push(DetectionSignal {
                        signal_type: "code_pattern".to_string(),
                        value: "FastAPI() or @app.method decorator found".to_string(),
                        confidence_boost: 0.9,
                        source: file_path.to_string_lossy().to_string(),
                    });
                    confidence += 0.9;
                    break;
                }
            }
        }
        
        if confidence < 0.6 { return None; }
        
        Some(FrameworkDetection {
            path: path.to_string_lossy().to_string(),
            framework: "fastapi".to_string(),
            confidence: confidence.min(1.0),
            signals,
            patterns: vec![
                RoutePattern {
                    name: "fastapi.decorators".to_string(),
                    files: "**/*.py".to_string(),
                    routes: vec!["@app.{method}('{path}')".to_string(), "@router.{method}('{path}')".to_string()],
                    convention: None,
                    confidence: 0.95,
                },
            ],
        })
    }
}

// Flask Detector  
struct FlaskDetector;
impl Detector for FlaskDetector {
    fn name(&self) -> &str { "flask" }
    
    fn detect(&self, path: &Path) -> Option<FrameworkDetection> {
        let requirements_txt = path.join("requirements.txt");
        
        let mut signals = Vec::new();
        let mut confidence: f32 = 0.0;
        
        // Check requirements.txt
        if let Ok(content) = fs::read_to_string(&requirements_txt) {
            if content.contains("Flask") || content.contains("flask") {
                signals.push(DetectionSignal {
                    signal_type: "requirements.txt".to_string(),
                    value: "flask dependency found".to_string(),
                    confidence_boost: 0.8,
                    source: requirements_txt.to_string_lossy().to_string(),
                });
                confidence += 0.8;
            }
        }
        
        // Check for Flask code patterns
        for filename in ["app.py", "main.py", "run.py", "src/app.py"] {
            let file_path = path.join(filename);
            if let Ok(content) = fs::read_to_string(&file_path) {
                if content.contains("Flask(__name__)") || content.contains("@app.route") {
                    signals.push(DetectionSignal {
                        signal_type: "code_pattern".to_string(),
                        value: "Flask(__name__) or @app.route found".to_string(),
                        confidence_boost: 0.9,
                        source: file_path.to_string_lossy().to_string(),
                    });
                    confidence += 0.9;
                    break;
                }
            }
        }
        
        if confidence < 0.6 { return None; }
        
        Some(FrameworkDetection {
            path: path.to_string_lossy().to_string(),
            framework: "flask".to_string(),
            confidence: confidence.min(1.0),
            signals,
            patterns: vec![
                RoutePattern {
                    name: "flask.decorators".to_string(),
                    files: "**/*.py".to_string(),
                    routes: vec!["@app.route('{path}', methods=['{method}'])".to_string()],
                    convention: None,
                    confidence: 0.95,
                },
            ],
        })
    }
}

// Laravel Detector
struct LaravelDetector;
impl Detector for LaravelDetector {
    fn name(&self) -> &str { "laravel" }
    
    fn detect(&self, path: &Path) -> Option<FrameworkDetection> {
        let composer_json = path.join("composer.json");
        let artisan = path.join("artisan");
        
        let mut signals = Vec::new();
        let mut confidence: f32 = 0.0;
        
        // Check composer.json
        if let Ok(content) = fs::read_to_string(&composer_json) {
            if content.contains("laravel/framework") {
                signals.push(DetectionSignal {
                    signal_type: "composer.json".to_string(),
                    value: "laravel/framework dependency found".to_string(),
                    confidence_boost: 0.9,
                    source: composer_json.to_string_lossy().to_string(),
                });
                confidence += 0.9;
            }
        }
        
        // Check for artisan file
        if artisan.exists() {
            signals.push(DetectionSignal {
                signal_type: "file".to_string(),
                value: "artisan command file found".to_string(),
                confidence_boost: 0.8,
                source: artisan.to_string_lossy().to_string(),
            });
            confidence += 0.8;
        }
        
        if confidence < 0.6 { return None; }
        
        Some(FrameworkDetection {
            path: path.to_string_lossy().to_string(),
            framework: "laravel".to_string(),
            confidence: confidence.min(1.0),
            signals,
            patterns: vec![
                RoutePattern {
                    name: "laravel.routes".to_string(),
                    files: "routes/**/*.php".to_string(),
                    routes: vec!["Route::{method}('{path}', {handler})".to_string()],
                    convention: Some("/api/users -> routes/api.php + UserController".to_string()),
                    confidence: 0.95,
                },
            ],
        })
    }
}

// Placeholder detectors for other frameworks
struct SpringBootDetector;
impl Detector for SpringBootDetector {
    fn name(&self) -> &str { "spring-boot" }
    fn detect(&self, _path: &Path) -> Option<FrameworkDetection> { None }
}

struct ActixDetector;
impl Detector for ActixDetector {
    fn name(&self) -> &str { "actix" }
    fn detect(&self, _path: &Path) -> Option<FrameworkDetection> { None }
}

struct GinDetector;
impl Detector for GinDetector {
    fn name(&self) -> &str { "gin" }
    fn detect(&self, _path: &Path) -> Option<FrameworkDetection> { None }
}
