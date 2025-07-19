use std::path::{Path, PathBuf};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use anyhow::Result;

pub struct FileWatcher {
    watcher: Option<RecommendedWatcher>,
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

impl FileWatcher {
    pub fn new() -> Self {
        Self { watcher: None }
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
            EventKind::Other => {
                // Handle rename events
                if event.paths.len() == 2 {
                    file_events.push(FileEvent {
                        path: event.paths[1].clone(),
                        event_type: FileEventType::Renamed {
                            from: event.paths[0].clone(),
                            to: event.paths[1].clone(),
                        },
                    });
                }
            }
            _ => {
                // Other event types we don't handle yet
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
