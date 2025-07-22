use crate::{ChangeEvent, ChangeSource, CodeRegion, FileDiff, LineChangeType};
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::sync::Arc;

/// Main processor for handling different types of changes
#[derive(Clone)]
pub struct DiffProcessor {
    git: Option<Arc<crate::git::GitIntegration>>,
    text_differ: crate::text::TextDiffer,
}

impl DiffProcessor {
    /// Create a new diff processor, optionally with git integration
    pub fn new(repo_path: Option<&Path>) -> Result<Self> {
        let git = if let Some(path) = repo_path {
            Some(Arc::new(crate::git::GitIntegration::new(path)?))
        } else {
            None
        };
        
        Ok(Self {
            git,
            text_differ: crate::text::TextDiffer::new(),
        })
    }

    /// Process any type of change into a standardized ChangeEvent
    pub async fn process_change(&self, source: ChangeSource) -> Result<ChangeEvent> {
        let diffs = match &source {
            ChangeSource::FileSystem { path, .. } => {
                self.process_file_change(path).await?
            }
            ChangeSource::GitCommit { hash, files } => {
                self.process_git_commit(hash, files).await?
            }
            ChangeSource::GitDiff { from, to } => {
                self.process_git_diff(from, to).await?
            }
            ChangeSource::Manual { .. } => {
                vec![] // Manual changes don't have automatic diffs
            }
        };

        Ok(ChangeEvent {
            source,
            diffs,
            timestamp: SystemTime::now(),
        })
    }

    /// Extract code regions that contain changes, with context lines
    pub fn extract_changed_regions(
        &self,
        diff: &FileDiff,
        context_lines: usize,
    ) -> Result<Vec<CodeRegion>> {
        let mut regions = Vec::new();
        let lines: Vec<&str> = diff.new_content.lines().collect();
        
        // Group consecutive changes into regions
        let mut current_region_start: Option<usize> = None;
        let mut current_changes = Vec::new();
        
        for change in &diff.changes {
            match change.change_type {
                LineChangeType::Context => {
                    if let Some(start) = current_region_start {
                        // End current region if we have enough context
                        if current_changes.len() > 0 {
                            let end_line = (change.line_number + context_lines).min(lines.len());
                            regions.push(self.create_region(
                                start,
                                end_line,
                                &lines,
                                current_changes.clone(),
                            )?);
                        }
                        current_region_start = None;
                        current_changes.clear();
                    }
                }
                _ => {
                    // Start new region if needed
                    if current_region_start.is_none() {
                        current_region_start = Some(
                            change.line_number.saturating_sub(context_lines).max(1)
                        );
                    }
                    current_changes.push(change.change_type.clone());
                }
            }
        }
        
        // Handle final region
        if let Some(start) = current_region_start {
            if !current_changes.is_empty() {
                let end_line = lines.len();
                regions.push(self.create_region(
                    start,
                    end_line,
                    &lines,
                    current_changes,
                )?);
            }
        }
        
        Ok(regions)
    }

    async fn process_file_change(&self, path: &Path) -> Result<Vec<FileDiff>> {
        // For file system changes, we need to compare with the last known state
        // For now, we'll just create a basic diff structure
        let new_content = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read file {}: {}", path.display(), e))?;
        
        // TODO: Get old content from cache/git
        let old_content = String::new(); // Placeholder
        
        let changes = self.text_differ.diff_lines(&old_content, &new_content)?;
        
        Ok(vec![FileDiff {
            path: path.to_path_buf(),
            old_content: Some(old_content),
            new_content,
            changes,
        }])
    }

    async fn process_git_commit(&self, hash: &str, _files: &[PathBuf]) -> Result<Vec<FileDiff>> {
        let git = self.git.as_ref()
            .ok_or_else(|| anyhow!("Git integration not available"))?;
        
        git.get_commit_diffs(hash).await
    }

    async fn process_git_diff(&self, from: &str, to: &str) -> Result<Vec<FileDiff>> {
        let git = self.git.as_ref()
            .ok_or_else(|| anyhow!("Git integration not available"))?;
        
        git.diff_commits(from, to).await
    }

    fn create_region(
        &self,
        start_line: usize,
        end_line: usize,
        lines: &[&str],
        change_types: Vec<LineChangeType>,
    ) -> Result<CodeRegion> {
        let content = lines
            .get(start_line.saturating_sub(1)..end_line)
            .ok_or_else(|| anyhow!("Invalid line range"))?
            .join("\n");
        
        Ok(CodeRegion {
            start_line,
            end_line,
            content,
            has_changes: !change_types.is_empty(),
            change_types,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_process_file_change() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        
        fs::write(&file_path, "fn hello() {}\n").unwrap();
        
        let processor = DiffProcessor::new(None).unwrap();
        let source = ChangeSource::FileSystem {
            path: file_path,
            modified: SystemTime::now(),
        };
        
        let change_event = processor.process_change(source).await.unwrap();
        assert_eq!(change_event.diffs.len(), 1);
    }
}
