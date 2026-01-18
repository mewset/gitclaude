use anyhow::Result;
use std::path::Path;

use crate::config::Config;

/// Context levels
#[derive(Debug, Clone, Copy)]
pub enum ContextLevel {
    Minimal,   // Just commit message
    Standard,  // Message + diff
    Extended,  // Message + diff + related files
    Full,      // Full repo context
}

/// Built context ready for template rendering
#[derive(Debug, Clone)]
pub struct Context {
    pub commit_hash: String,
    pub commit_message: String,
    pub author: String,
    pub date: String,
    pub branch: String,
    pub diff: String,
    pub diff_stat: String,
    pub staged_diff: Option<String>,
    pub staged_count: Option<usize>,
    pub affected_files: Vec<String>,
    pub affected_packages: Vec<String>,
    pub recent_commits: Vec<CommitInfo>,
}

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

/// Build context for an event
pub fn build_context(
    repo_path: &Path,
    event: &str,
    level: ContextLevel,
    config: &Config,
) -> Result<Context> {
    let repo = git2::Repository::open(repo_path)?;

    let head = repo.head()?;
    let commit = head.peel_to_commit()?;

    let branch = head
        .shorthand()
        .unwrap_or("HEAD")
        .to_string();

    let commit_hash = commit.id().to_string()[..7].to_string();
    let commit_message = commit.message().unwrap_or("").to_string();
    let author = commit.author().name().unwrap_or("Unknown").to_string();

    let date = chrono::DateTime::from_timestamp(commit.time().seconds(), 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default();

    // Get diff
    let (diff, diff_stat) = match level {
        ContextLevel::Minimal => (String::new(), String::new()),
        _ => super::diff::get_commit_diff(&repo, &commit, config)?,
    };

    // Get affected files
    let affected_files = super::diff::get_affected_files(&repo, &commit)?;

    // Get affected packages (monorepo)
    let affected_packages = if config.monorepo.enabled {
        super::monorepo::detect_affected_packages(repo_path, &affected_files)?
    } else {
        vec![]
    };

    // Get recent commits for extended context
    let recent_commits = match level {
        ContextLevel::Extended | ContextLevel::Full => {
            get_recent_commits(&repo, config.context.smart.include_recent_commits)?
        }
        _ => vec![],
    };

    Ok(Context {
        commit_hash,
        commit_message,
        author,
        date,
        branch,
        diff,
        diff_stat,
        staged_diff: None,
        staged_count: None,
        affected_files,
        affected_packages,
        recent_commits,
    })
}

fn get_recent_commits(repo: &git2::Repository, count: usize) -> Result<Vec<CommitInfo>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let commits: Vec<_> = revwalk
        .take(count + 1) // +1 because we skip HEAD
        .skip(1)
        .filter_map(|oid| oid.ok())
        .filter_map(|oid| repo.find_commit(oid).ok())
        .map(|commit| CommitInfo {
            hash: commit.id().to_string()[..7].to_string(),
            message: commit.message().unwrap_or("").lines().next().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("Unknown").to_string(),
            date: chrono::DateTime::from_timestamp(commit.time().seconds(), 0)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
        })
        .collect();

    Ok(commits)
}
