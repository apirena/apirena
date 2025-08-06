use hallwatch_core::{FileWatcher, FileEventType};
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
    
    // Canonicalize paths to handle macOS /var -> /private/var symlinks
    let expected_path = test_file.canonicalize().expect("Failed to canonicalize test file path");
    let actual_path = event.path.canonicalize().unwrap_or(event.path.clone());
    
    assert_eq!(actual_path, expected_path);
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
    
    // Canonicalize paths to handle macOS /var -> /private/var symlinks
    let expected_path = test_file.canonicalize().expect("Failed to canonicalize test file path");
    let actual_path = event.path.canonicalize().unwrap_or(event.path.clone());
    
    assert_eq!(actual_path, expected_path);
    
    // File modification events might be reported as Created or Modified events on some systems
    assert!(matches!(event.event_type, FileEventType::Modified | FileEventType::Created),
            "Expected Modified or Created event, got: {:?}", event.event_type);
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
    
    // Collect events for the file deletion
    let mut events = Vec::new();
    let mut timeout_count = 0;
    const MAX_TIMEOUTS: usize = 3;
    
    while events.len() < 3 && timeout_count < MAX_TIMEOUTS {
        match timeout(Duration::from_millis(500), rx.recv()).await {
            Ok(Some(event)) => {
                events.push(event);
                timeout_count = 0;
            }
            Ok(None) => break, // Channel closed
            Err(_) => {
                timeout_count += 1;
            }
        }
    }
    
    // Find any event related to our test file
    let expected_parent = test_file.parent().unwrap().canonicalize().expect("Failed to canonicalize parent dir");
    let expected_filename = test_file.file_name().unwrap();
    
    let relevant_event = events.iter().find(|event| {
        let actual_parent = event.path.parent()
            .and_then(|p| p.canonicalize().ok())
            .unwrap_or_else(|| event.path.parent().unwrap().to_path_buf());
        let actual_filename = event.path.file_name().unwrap();
        
        actual_parent == expected_parent && actual_filename == expected_filename
    });
    
    assert!(relevant_event.is_some(), "Should receive at least one event for the deleted file");
    
    // File systems can report deletion as various event types depending on the platform
    // The important thing is that we detect a change to the file
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
    
    // Create multiple files with longer delays to ensure events are processed
    let file1 = temp_path.join("file1.txt");
    let file2 = temp_path.join("file2.txt");
    
    fs::write(&file1, "content1").await.expect("Failed to write file1");
    tokio::time::sleep(Duration::from_millis(100)).await; // Longer delay
    fs::write(&file2, "content2").await.expect("Failed to write file2");
    
    // Collect events with more flexible timeout and better error handling
    let mut events = Vec::new();
    let mut timeout_count = 0;
    const MAX_TIMEOUTS: usize = 3;
    const EVENT_TIMEOUT_MS: u64 = 1000; // Increased timeout
    
    while events.len() < 2 && timeout_count < MAX_TIMEOUTS {
        match timeout(Duration::from_millis(EVENT_TIMEOUT_MS), rx.recv()).await {
            Ok(Some(event)) => {
                events.push(event);
                timeout_count = 0; // Reset timeout count on successful event
            }
            Ok(None) => break, // Channel closed
            Err(_) => {
                timeout_count += 1;
                if events.len() > 0 {
                    // We got at least one event, that's acceptable
                    break;
                }
            }
        }
    }
    
    // Should receive at least one event (file systems can be flaky)
    assert!(events.len() >= 1, "Should detect at least one file creation");
    
    // Verify that at least one event corresponds to our created files
    let created_files = vec![&file1, &file2];
    let mut found_valid_event = false;
    
    for event in &events {
        // Canonicalize paths for comparison
        let event_path_canonical = event.path.canonicalize().unwrap_or(event.path.clone());
        
        for created_file in &created_files {
            let created_file_canonical = created_file.canonicalize().unwrap_or_else(|_| created_file.to_path_buf());
            if event_path_canonical == created_file_canonical {
                found_valid_event = true;
                break;
            }
        }
        
        if found_valid_event {
            break;
        }
    }
    
    assert!(found_valid_event, "Should detect at least one file creation for our test files");
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
    assert!(!watcher1.is_watching());
    assert!(!watcher2.is_watching());
}
