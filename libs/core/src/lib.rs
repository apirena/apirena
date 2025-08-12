use std::path::{Path, PathBuf};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use anyhow::Result;
use std::time::SystemTime;
use pinpath_diff::{ChangeEvent, ChangeSource, DiffProcessor};

pub struct FileWatcher {
    watcher: Option<RecommendedWatcher>,
    diff_processor: Option<DiffProcessor>,
}

#[derive(Debug, Clone)]
pub struct FileEvent {
    pub path: PathBuf,
    pub event_type: FileEventType,
}

#[derive(Debug, Clone)]
pub enum FileEventType {
    Created,
    Modified,
    Deleted,
    Renamed { from: PathBuf, to: PathBuf },
}

/// Enhanced watcher that can emit both file events and change events
pub struct EnhancedWatcher {
    file_watcher: FileWatcher,
    diff_processor: DiffProcessor,
}

impl EnhancedWatcher {
    pub fn new(repo_path: Option<&Path>) -> Result<Self> {
        Ok(Self {
            file_watcher: FileWatcher::new(),
            diff_processor: DiffProcessor::new(repo_path)?,
        })
    }

    /// Watch for changes and emit enhanced change events
    pub async fn watch_changes<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<mpsc::Receiver<ChangeEvent>> {
        let (tx, rx) = mpsc::channel(100);
        let mut file_rx = self.file_watcher.watch(&path).await?;
        
        // For now, convert file events to basic change events without git integration
        // TODO: Make this work with git in a thread-safe way
        tokio::spawn(async move {
            while let Some(file_event) = file_rx.recv().await {
                let change_source = ChangeSource::FileSystem {
                    path: file_event.path.clone(),
                    modified: SystemTime::now(),
                };
                
                // Create a simple change event without diff processing for now
                let change_event = ChangeEvent {
                    source: change_source,
                    diffs: vec![], // Empty diffs for now
                    timestamp: SystemTime::now(),
                };
                
                if tx.send(change_event).await.is_err() {
                    break;
                }
            }
        });
        
        Ok(rx)
    }

    /// Process git changes directly
    pub async fn process_git_diff(&self, from: &str, to: &str) -> Result<ChangeEvent> {
        let source = ChangeSource::GitDiff {
            from: from.to_string(),
            to: to.to_string(),
        };
        
        self.diff_processor.process_change(source).await
    }
}

impl FileWatcher {
    pub fn new() -> Self {
        Self { 
            watcher: None,
            diff_processor: None,
        }
    }

    pub fn is_watching(&self) -> bool {
        self.watcher.is_some()
    }

    pub async fn watch<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<mpsc::Receiver<FileEvent>> {
        let (tx, rx) = mpsc::channel(100);
        
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            match res {
                Ok(event) => {
                    let file_events = Self::convert_notify_event(event);
                    for file_event in file_events {
                        if let Err(e) = tx.blocking_send(file_event) {
                            eprintln!("Failed to send file event: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        self.watcher = Some(watcher);
        
        Ok(rx)
    }

    fn convert_notify_event(event: Event) -> Vec<FileEvent> {
        use notify::EventKind;
        
        let mut file_events = Vec::new();
        
        match event.kind {
            EventKind::Create(_) => {
                for path in event.paths {
                    file_events.push(FileEvent {
                        path: path.clone(),
                        event_type: FileEventType::Created,
                    });
                }
            }
            EventKind::Modify(_) => {
                for path in event.paths {
                    file_events.push(FileEvent {
                        path: path.clone(),
                        event_type: FileEventType::Modified,
                    });
                }
            }
            EventKind::Remove(_) => {
                for path in event.paths {
                    file_events.push(FileEvent {
                        path: path.clone(),
                        event_type: FileEventType::Deleted,
                    });
                }
            }
            EventKind::Access(_) => {
                // Access events can indicate file reads, but we typically don't want these
                // for basic file watching. Skip them.
            }
            EventKind::Any => {
                // Generic event - convert to Modified as a fallback
                for path in event.paths {
                    file_events.push(FileEvent {
                        path: path.clone(),
                        event_type: FileEventType::Modified,
                    });
                }
            }
            EventKind::Other => {
                // Handle rename events or treat as generic modification
                if event.paths.len() == 2 {
                    file_events.push(FileEvent {
                        path: event.paths[1].clone(),
                        event_type: FileEventType::Renamed {
                            from: event.paths[0].clone(),
                            to: event.paths[1].clone(),
                        },
                    });
                } else if !event.paths.is_empty() {
                    // Single path Other event - could be various things, treat as modified
                    for path in event.paths {
                        file_events.push(FileEvent {
                            path: path.clone(),
                            event_type: FileEventType::Modified,
                        });
                    }
                }
            }
        }
        
        file_events
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_watcher_creation() {
        let _watcher = FileWatcher::new();
    }
}
