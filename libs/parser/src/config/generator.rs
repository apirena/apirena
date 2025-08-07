use super::*;
use anyhow::Result;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct ConfigGenerator {
    debug_mode: bool,
    #[allow(dead_code)]
    template: String,
}

impl ConfigGenerator {
    pub fn new(debug_mode: bool) -> Self {
        Self {
            debug_mode,
            template: include_str!("template.js").to_string(),
        }
    }

    pub async fn generate(
        &self,
        project_root: &Path,
        frameworks: Vec<FrameworkDetection>,
        project_structure: ProjectStructure,
    ) -> Result<DiscoveredConfig> {
        let now: DateTime<Utc> = Utc::now();
        let performance = self.calculate_performance_config(&project_structure);
        
        let config = DiscoveredConfig {
            version: "1.0.0".to_string(),
            generated_at: now.to_rfc3339(),
            debug_mode: self.debug_mode,
            project_structure,
            frameworks,
            performance,
            overrides: HashMap::new(),
        };

        // Write the JavaScript config file
        self.write_config_file(project_root, &config).await?;
        
        Ok(config)
    }

    async fn write_config_file(&self, project_root: &Path, config: &DiscoveredConfig) -> Result<()> {
        let reqsmith_dir = project_root.join(".reqsmith");
        fs::create_dir_all(&reqsmith_dir)?;
        
        let config_path = reqsmith_dir.join("discovered.config.js");
        let js_content = self.render_template(config)?;
        
        fs::write(&config_path, js_content)?;
        
        // Also write JSON version for easy parsing
        if self.debug_mode {
            let json_path = reqsmith_dir.join("discovered.config.json");
            let json_content = serde_json::to_string_pretty(config)?;
            fs::write(json_path, json_content)?;
        }
        
        Ok(())
    }

    fn render_template(&self, config: &DiscoveredConfig) -> Result<String> {
        let mut content = String::new();
        
        content.push_str(&format!(
            r#"// Auto-generated Reqsmith configuration
// Generated: {}
// Version: {}
// Debug Mode: {}

export default {{
  // Metadata
  _meta: {{
    version: "{}",
    generated: "{}",
    lastModified: "{}",
    debugMode: {},
  }},

  // Global configuration
  debugMode: {},

  // Detected project structure
  structure: {{
    type: "{}",
    fileCount: {},
    totalSize: {},
    roots: [{}],
  }},

  // Detected frameworks
  frameworks: [
"#,
            config.generated_at,
            config.version,
            config.debug_mode,
            config.version,
            config.generated_at,
            config.generated_at,
            config.debug_mode,
            config.debug_mode,
            config.project_structure.project_type,
            config.project_structure.file_count,
            config.project_structure.total_size,
            config.project_structure.roots.iter()
                .map(|r| format!("\"{}\"", r))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        for (i, framework) in config.frameworks.iter().enumerate() {
            if i > 0 { content.push_str(",\n"); }
            
            content.push_str(&format!(
                r#"    {{
      path: "{}",
      framework: "{}",
      confidence: {},
"#,
                framework.path,
                framework.framework,
                framework.confidence
            ));

            if self.debug_mode {
                content.push_str("\n      // Detection signals (debug mode)\n      _signals: [\n");
                for signal in &framework.signals {
                    content.push_str(&format!(
                        r#"        {{ type: "{}", value: "{}", confidence: {}, source: "{}" }},
"#,
                        signal.signal_type,
                        signal.value.replace('"', "\\\""),
                        signal.confidence_boost,
                        signal.source.replace('"', "\\\"")
                    ));
                }
                content.push_str("      ],\n");
            }

            content.push_str("\n      // Patterns for route detection\n      patterns: [\n");
            for pattern in &framework.patterns {
                content.push_str(&format!(
                    r#"        {{
          name: "{}",
          files: "{}",
          routes: [{}],
"#,
                    pattern.name,
                    pattern.files,
                    pattern.routes.iter()
                        .map(|r| format!("\"{}\"", r.replace('"', "\\\"")))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));

                if let Some(convention) = &pattern.convention {
                    content.push_str(&format!(
                        r#"          convention: "{}",
"#,
                        convention.replace('"', "\\\"")
                    ));
                }

                content.push_str(&format!(
                    r#"          confidence: {},
        }},
"#,
                    pattern.confidence
                ));
            }
            content.push_str("      ],\n    }");
        }

        content.push_str(&format!(
            r#"
  ],

  // Performance settings (auto-calculated)
  performance: {{
    threads: "{}",
    cacheStrategy: "{}",
    maxFileSize: "{}",
    estimatedScanTime: "{}",
  }},

  // User overrides (preserved between regenerations)
  overrides: {{
    // Add your custom patterns here
    // These will be preserved when config is regenerated
    
    // Example:
    // customPatterns: [
    //   {{
    //     name: 'my-custom-api',
    //     files: 'lib/handlers/**/*.js',
    //     routes: ['defineHandler("{{method}}", "{{path}}", {{handler}})']
    //   }}
    // ]
  }},
}};
"#,
            config.performance.threads,
            config.performance.cache_strategy,
            config.performance.max_file_size,
            config.performance.estimated_scan_time
        ));

        Ok(content)
    }

    fn calculate_performance_config(&self, structure: &ProjectStructure) -> PerformanceConfig {
        let threads = if structure.file_count > 5000 {
            "8"
        } else if structure.file_count > 1000 {
            "4"
        } else {
            "auto"
        }.to_string();

        let cache_strategy = if structure.file_count > 10000 {
            "aggressive"
        } else if structure.file_count > 1000 {
            "balanced"
        } else {
            "minimal"
        }.to_string();

        let max_file_size = if structure.file_count > 5000 {
            "512KB"
        } else {
            "1MB"
        }.to_string();

        let estimated_scan_time = if structure.file_count > 10000 {
            "2-5 minutes"
        } else if structure.file_count > 1000 {
            "30-60 seconds"
        } else {
            "<10 seconds"
        }.to_string();

        PerformanceConfig {
            threads,
            cache_strategy,
            max_file_size,
            estimated_scan_time,
        }
    }
}
