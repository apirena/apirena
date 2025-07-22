use crate::FileDiff;
use anyhow::{anyhow, Result};
use git2::{Diff, DiffOptions, Repository};
use std::path::Path;

/// Git integration for handling repository-based diffs
pub struct GitIntegration {
    repo: Repository,
}

impl GitIntegration {
    /// Create a new git integration for the given repository path
    pub fn new(repo_path: &Path) -> Result<Self> {
        let repo = Repository::open(repo_path)
            .or_else(|_| Repository::discover(repo_path))
            .map_err(|e| anyhow!("Failed to open git repository at {}: {}", repo_path.display(), e))?;
        
        Ok(Self { repo })
    }

    /// Get diffs for a specific commit
    pub async fn get_commit_diffs(&self, hash: &str) -> Result<Vec<FileDiff>> {
        let commit = self.repo.find_commit(git2::Oid::from_str(hash)?)
            .map_err(|e| anyhow!("Failed to find commit {}: {}", hash, e))?;
        
        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff = self.repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut DiffOptions::new()),
        )?;

        self.extract_file_diffs(&mut diff).await
    }

    /// Get diffs between two commits
    pub async fn diff_commits(&self, from: &str, to: &str) -> Result<Vec<FileDiff>> {
        let from_commit = self.repo.find_commit(git2::Oid::from_str(from)?)
            .map_err(|e| anyhow!("Failed to find commit {}: {}", from, e))?;
        let to_commit = self.repo.find_commit(git2::Oid::from_str(to)?)
            .map_err(|e| anyhow!("Failed to find commit {}: {}", to, e))?;

        let from_tree = from_commit.tree()?;
        let to_tree = to_commit.tree()?;

        let mut diff = self.repo.diff_tree_to_tree(
            Some(&from_tree),
            Some(&to_tree),
            Some(&mut DiffOptions::new()),
        )?;

        self.extract_file_diffs(&mut diff).await
    }

    /// Get the current working directory changes
    pub async fn get_working_dir_changes(&self) -> Result<Vec<FileDiff>> {
        let head = self.repo.head()?.peel_to_tree()?;
        let mut diff = self.repo.diff_tree_to_workdir_with_index(
            Some(&head),
            Some(&mut DiffOptions::new()),
        )?;

        self.extract_file_diffs(&mut diff).await
    }

    /// Check if a path is ignored by git
    pub fn is_ignored(&self, path: &Path) -> bool {
        self.repo.status_file(path)
            .map(|status| status.contains(git2::Status::IGNORED))
            .unwrap_or(false)
    }

    async fn extract_file_diffs(&self, diff: &mut Diff<'_>) -> Result<Vec<FileDiff>> {
        let mut file_diffs = Vec::new();

        diff.foreach(
            &mut |delta, _progress| {
                if let Some(old_file) = delta.old_file().path() {
                    if let Some(new_file) = delta.new_file().path() {
                        // For now, create a placeholder FileDiff
                        // In a real implementation, we'd extract the actual content changes
                        let file_diff = FileDiff {
                            path: new_file.to_path_buf(),
                            old_content: None, // TODO: Extract from git
                            new_content: String::new(), // TODO: Extract from git
                            changes: Vec::new(), // TODO: Parse git diff lines
                        };
                        file_diffs.push(file_diff);
                    }
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(file_diffs)
    }

    /// Get the content of a file at a specific commit
    pub fn get_file_at_commit(&self, commit_hash: &str, file_path: &Path) -> Result<String> {
        let commit = self.repo.find_commit(git2::Oid::from_str(commit_hash)?)?;
        let tree = commit.tree()?;
        let entry = tree.get_path(file_path)?;
        let blob = self.repo.find_blob(entry.id())?;
        
        std::str::from_utf8(blob.content())
            .map(|s| s.to_string())
            .map_err(|e| anyhow!("File content is not valid UTF-8: {}", e))
    }

    /// Get the current HEAD commit hash
    pub fn get_head_hash(&self) -> Result<String> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.id().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_git_integration_creation() {
        // Create a temporary git repo for testing
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        
        // Initialize git repo
        Repository::init(repo_path).unwrap();
        
        // Test git integration creation
        let git_integration = GitIntegration::new(repo_path);
        assert!(git_integration.is_ok());
    }

    #[test]
    fn test_is_ignored() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        
        // Initialize git repo
        Repository::init(repo_path).unwrap();
        
        // Create .gitignore
        fs::write(repo_path.join(".gitignore"), "*.log\ntarget/\n").unwrap();
        
        let git_integration = GitIntegration::new(repo_path).unwrap();
        
        // Test file paths (this is a simplified test)
        // In practice, git ignore checking is more complex
        assert!(!git_integration.is_ignored(Path::new("src/main.rs")));
    }
}
