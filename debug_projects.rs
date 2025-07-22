use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;

fn main() {
    println!("🚀 Debugging benchmark project discovery");
    
    let workspace_root = std::env::current_dir()
        .expect("Failed to get current directory");
    
    println!("Current working directory: {}", workspace_root.display());
    
    let projects_dir = workspace_root.join("libs/benchmarks/projects");
    
    println!("Looking for projects in: {}", projects_dir.display());
    println!("Projects directory exists: {}", projects_dir.exists());
    
    if projects_dir.exists() {
        println!("\nListing contents:");
        if let Ok(entries) = fs::read_dir(&projects_dir) {
            for entry in entries.flatten() {
                println!("  - {} ({})", 
                    entry.file_name().to_string_lossy(),
                    if entry.path().is_dir() { "dir" } else { "file" }
                );
            }
        }
        
        println!("\nChecking categories:");
        for category in &["small", "medium", "large"] {
            let category_path = projects_dir.join(category);
            println!("  {}: exists={}", category, category_path.exists());
            
            if category_path.exists() {
                if let Ok(entries) = fs::read_dir(&category_path) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            println!("    - {}", entry.file_name().to_string_lossy());
                        }
                    }
                }
            }
        }
    }
}
