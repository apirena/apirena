use crate::{FileDiff, LineChange, LineChangeType};
use anyhow::Result;
use similar::{ChangeTag, TextDiff};
use std::path::PathBuf;

/// Text diffing using the `similar` crate for high-quality diffs
#[derive(Clone)]
pub struct TextDiffer;

impl TextDiffer {
    pub fn new() -> Self {
        Self
    }

    /// Generate line-by-line diff between two text contents
    pub fn diff_lines(&self, old_content: &str, new_content: &str) -> Result<Vec<LineChange>> {
        let diff = TextDiff::from_lines(old_content, new_content);
        let mut changes = Vec::new();
        let mut new_line_number = 1;

        for change in diff.iter_all_changes() {
            let content = change.value().to_string();
            
            match change.tag() {
                ChangeTag::Equal => {
                    changes.push(LineChange {
                        line_number: new_line_number,
                        change_type: LineChangeType::Context,
                        content: content.trim_end_matches('\n').to_string(),
                    });
                    new_line_number += 1;
                }
                ChangeTag::Delete => {
                    // For deletions, we'll mark them but not increment line number
                    changes.push(LineChange {
                        line_number: new_line_number,
                        change_type: LineChangeType::Removed,
                        content: content.trim_end_matches('\n').to_string(),
                    });
                }
                ChangeTag::Insert => {
                    changes.push(LineChange {
                        line_number: new_line_number,
                        change_type: LineChangeType::Added,
                        content: content.trim_end_matches('\n').to_string(),
                    });
                    new_line_number += 1;
                }
            }
        }

        Ok(changes)
    }

    /// Generate a unified diff between two file contents
    pub fn diff_files(&self, old_path: &PathBuf, old_content: &str, new_content: &str) -> Result<FileDiff> {
        let changes = self.diff_lines(old_content, new_content)?;
        
        Ok(FileDiff {
            path: old_path.clone(),
            old_content: Some(old_content.to_string()),
            new_content: new_content.to_string(),
            changes,
        })
    }

    /// Get a summary of changes (added/removed/modified lines)
    pub fn get_change_summary(&self, changes: &[LineChange]) -> ChangeSummary {
        let mut summary = ChangeSummary::default();
        
        for change in changes {
            match change.change_type {
                LineChangeType::Added => summary.added += 1,
                LineChangeType::Removed => summary.removed += 1,
                LineChangeType::Modified { .. } => summary.modified += 1,
                LineChangeType::Context => summary.context += 1,
            }
        }
        
        summary
    }
}

#[derive(Debug, Default)]
pub struct ChangeSummary {
    pub added: usize,
    pub removed: usize,
    pub modified: usize,
    pub context: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_lines_addition() {
        let differ = TextDiffer::new();
        let old = "line 1\nline 2\n";
        let new = "line 1\nline 2\nline 3\n";
        
        let changes = differ.diff_lines(old, new).unwrap();
        
        // Should have 2 context lines and 1 addition
        assert!(changes.iter().any(|c| matches!(c.change_type, LineChangeType::Added)));
        assert_eq!(changes.iter().filter(|c| matches!(c.change_type, LineChangeType::Context)).count(), 2);
    }

    #[test]
    fn test_diff_lines_modification() {
        let differ = TextDiffer::new();
        let old = "line 1\nold line\nline 3\n";
        let new = "line 1\nnew line\nline 3\n";
        
        let changes = differ.diff_lines(old, new).unwrap();
        
        // Should detect the change
        assert!(changes.iter().any(|c| matches!(c.change_type, LineChangeType::Removed)));
        assert!(changes.iter().any(|c| matches!(c.change_type, LineChangeType::Added)));
    }

    #[test]
    fn test_change_summary() {
        let differ = TextDiffer::new();
        let changes = vec![
            LineChange {
                line_number: 1,
                change_type: LineChangeType::Context,
                content: "unchanged".to_string(),
            },
            LineChange {
                line_number: 2,
                change_type: LineChangeType::Added,
                content: "new line".to_string(),
            },
        ];
        
        let summary = differ.get_change_summary(&changes);
        assert_eq!(summary.added, 1);
        assert_eq!(summary.context, 1);
    }
}
