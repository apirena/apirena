use apirena_core::{FileWatcher, FileEventType};
use std::path::Path;
use tempfile::TempDir;
use tokio::fs;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_file_creation_detection() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    let mut watcher = FileWatcher::new();
    let mut rx = watcher.watch(temp_path).await.expect("Failed to start watching");
    
    // Create a test file
    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "test content").await.expect("Failed to write file");
    
    // Wait for file creation event
    let event = timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("Timeout waiting for event")
        .expect("Failed to receive event");
    
    assert_eq!(event.path, test_file);
    assert!(matches!(event.event_type, FileEventType::Created));
}

#[tokio::test]
async fn test_file_modification_detection() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create file first
    let test_file = temp_path.join("modify_test.txt");
    fs::write(&test_file, "initial content").await.expect("Failed to write file");
    
    // Give file system time to settle
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut watcher = FileWatcher::new();
    let mut rx = watcher.watch(temp_path).await.expect("Failed to start watching");
    
    // Modify the file
    fs::write(&test_file, "modified content").await.expect("Failed to modify file");
    
    // Wait for modification event
    let event = timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("Timeout waiting for event")
        .expect("Failed to receive event");
    
    assert_eq!(event.path, test_file);
    assert!(matches!(event.event_type, FileEventType::Modified));
}

#[tokio::test]
async fn test_file_deletion_detection() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create file first
    let test_file = temp_path.join("delete_test.txt");
    fs::write(&test_file, "content to delete").await.expect("Failed to write file");
    
    // Give file system time to settle
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut watcher = FileWatcher::new();
    let mut rx = watcher.watch(temp_path).await.expect("Failed to start watching");
    
    // Delete the file
    fs::remove_file(&test_file).await.expect("Failed to delete file");
    
    // Wait for deletion event
    let event = timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("Timeout waiting for event")
        .expect("Failed to receive event");
    
    assert_eq!(event.path, test_file);
    assert!(matches!(event.event_type, FileEventType::Deleted));
}

#[tokio::test]
async fn test_recursive_directory_watching() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create subdirectory
    let sub_dir = temp_path.join("subdir");
    fs::create_dir(&sub_dir).await.expect("Failed to create subdirectory");
    
    let mut watcher = FileWatcher::new();
    let mut rx = watcher.watch(temp_path).await.expect("Failed to start watching");
    
    // Create file in subdirectory
    let test_file = sub_dir.join("nested_file.txt");
    fs::write(&test_file, "nested content").await.expect("Failed to write nested file");
    
    // Wait for file creation event in subdirectory
    let event = timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("Timeout waiting for event")
        .expect("Failed to receive event");
    
    // Should detect file creation in subdirectory
    assert!(matches!(event.event_type, FileEventType::Created));
}

#[tokio::test]
async fn test_multiple_file_events() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    let mut watcher = FileWatcher::new();
    let mut rx = watcher.watch(temp_path).await.expect("Failed to start watching");
    
    // Create multiple files
    let file1 = temp_path.join("file1.txt");
    let file2 = temp_path.join("file2.txt");
    let file3 = temp_path.join("file3.txt");
    
    fs::write(&file1, "content1").await.expect("Failed to write file1");
    fs::write(&file2, "content2").await.expect("Failed to write file2");
    fs::write(&file3, "content3").await.expect("Failed to write file3");
    
    // Collect events
    let mut events = Vec::new();
    for _ in 0..3 {
        let event = timeout(Duration::from_secs(2), rx.recv())
            .await
            .expect("Timeout waiting for event")
            .expect("Failed to receive event");
        events.push(event);
    }
    
    // Should receive events for all files
    assert_eq!(events.len(), 3);
    
    let paths: Vec<_> = events.iter().map(|e| &e.path).collect();
    assert!(paths.contains(&&file1));
    assert!(paths.contains(&&file2));
    assert!(paths.contains(&&file3));
}

#[tokio::test]
async fn test_watcher_lifecycle() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Test that watcher can be created and destroyed properly
    {
        let mut watcher = FileWatcher::new();
        let _rx = watcher.watch(temp_path).await.expect("Failed to start watching");
        
        // Create a file to ensure watcher is working
        let test_file = temp_path.join("lifecycle_test.txt");
        fs::write(&test_file, "test").await.expect("Failed to write file");
        
        // Watcher should be dropped when going out of scope
    }
    
    // Should be able to create a new watcher after the old one is dropped
    {
        let mut watcher2 = FileWatcher::new();
        let _rx2 = watcher2.watch(temp_path).await.expect("Failed to start second watcher");
    }
}

#[test]
fn test_file_watcher_default() {
    let watcher1 = FileWatcher::default();
    let watcher2 = FileWatcher::new();
    
    // Both should be equivalent
    assert!(watcher1.watcher.is_none());
    assert!(watcher2.watcher.is_none());
}
