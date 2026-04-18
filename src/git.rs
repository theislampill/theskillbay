use git2::{Repository, Signature, Commit, Oid};
use std::path::Path;
use anyhow::Result;

/// Initialize a new Git repo at path
pub fn init_repo(path: &Path) -> Result<Repository> {
    let repo = Repository::init(path)?;
    Ok(repo)
}

/// Clone a Git repo from url to path
pub fn clone_repo(url: &str, path: &Path) -> Result<Repository> {
    let repo = Repository::clone(url, path)?;
    Ok(repo)
}

/// Get the current commit hash
pub fn get_head_hash(repo: &Repository) -> Result<String> {
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    Ok(commit.id().to_string())
}

/// Create a commit with message
pub fn commit(repo: &Repository, message: &str) -> Result<Oid> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent_commit = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
    let sig = Signature::now("theskillbay", "agent@theskillbay")?;
    let commit_id = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        message,
        &tree,
        &parent_commit.as_ref().map(|c| vec![c]).unwrap_or(vec![]),
    )?;
    Ok(commit_id)
}