use std::path::Path;
use git2::{Repository, RemoteCallbacks, PushOptions, FetchOptions, Direction};
use crate::error::{Result, GitHubSyncError};
use crate::logger;

pub struct GitSync {
    repo: Repository,
    remote_url: String,
    branch: String,
}

impl GitSync {
    pub fn new<P: AsRef<Path>>(path: P, remote_url: &str, branch: &str) -> Result<Self> {
        let repo = match Repository::open(path.as_ref()) {
            Ok(repo) => repo,
            Err(_) => {
                logger::info("Initializing new Git repository...");
                let repo = Repository::init(path.as_ref())?;
                
                // Configure remote
                repo.remote("origin", remote_url)?;
                
                // Create initial commit if needed
                let mut index = repo.index()?;
                if index.write_tree()?.is_empty() {
                    let tree_id = index.write_tree()?;
                    let tree = repo.find_tree(tree_id)?;
                    let signature = repo.signature()?;
                    repo.commit(
                        Some("HEAD"),
                        &signature,
                        &signature,
                        "Initial commit",
                        &tree,
                        &[],
                    )?;
                }
                
                repo
            }
        };

        Ok(Self {
            repo,
            remote_url: remote_url.to_string(),
            branch: branch.to_string(),
        })
    }

    pub fn sync(&self) -> Result<()> {
        // Pull changes first
        self.pull()?;

        // Then push our changes
        self.push()?;

        Ok(())
    }

    fn pull(&self) -> Result<()> {
        let mut remote = self.repo.find_remote("origin")?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Fetch from remote
        remote.fetch(&[&self.branch], Some(&mut fo), None)?;

        // Get remote branch
        let fetch_head = self.repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)?;

        // Do the merge analysis
        let analysis = self.repo.merge_analysis(&[&fetch_commit])?;

        if analysis.0.is_up_to_date() {
            logger::info("Already up to date");
            return Ok(());
        }

        if analysis.0.is_fast_forward() {
            // Fast-forward changes
            let refname = format!("refs/heads/{}", self.branch);
            let mut reference = self.repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            self.repo.set_head(&refname)?;
            self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            logger::success("Fast-forwarded changes");
        } else {
            // Create backup branch and reset to remote
            let backup_branch = format!("backup_{}", chrono::Local::now().format("%Y%m%d_%H%M%S"));
            let head = self.repo.head()?;
            let head_commit = head.peel_to_commit()?;
            self.repo.branch(&backup_branch, &head_commit, false)?;
            logger::info(&format!("Created backup branch: {}", backup_branch));

            // Reset to remote state
            let obj = self.repo.find_object(fetch_commit.id(), None)?;
            self.repo.reset(&obj, git2::ResetType::Hard, None)?;
            logger::warn("Reset to remote state");
        }

        Ok(())
    }

    fn push(&self) -> Result<()> {
        let mut remote = self.repo.find_remote("origin")?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });

        let mut po = PushOptions::new();
        po.remote_callbacks(callbacks);

        // Get the current branch reference
        let head = self.repo.head()?;
        let branch_name = head.shorthand().ok_or_else(|| {
            GitHubSyncError::GitError(git2::Error::from_str("Could not get branch name"))
        })?;

        // Check if we have any changes to push
        let status = self.repo.statuses(None)?;
        if !status.is_empty() {
            // Stage all changes
            let mut index = self.repo.index()?;
            index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            index.write()?;

            // Create commit
            let tree_id = index.write_tree()?;
            let tree = self.repo.find_tree(tree_id)?;
            let head_commit = self.repo.head()?.peel_to_commit()?;
            let signature = self.repo.signature()?;

            self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                "GitHub Sync: Auto-commit changes",
                &tree,
                &[&head_commit],
            )?;
        }

        // Push changes
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
        remote.push(&[&refspec], Some(&mut po))?;

        logger::success("Changes pushed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_git_init() {
        let temp = tempdir().unwrap();
        let git = GitSync::new(
            temp.path(),
            "git@github.com:test/repo.git",
            "main"
        ).unwrap();
        
        assert!(temp.path().join(".git").exists());
    }
} 