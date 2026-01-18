use anyhow::Result;
use git2::{Commit, DiffOptions, Repository};

use crate::config::Config;

/// Get diff for a commit
pub fn get_commit_diff(
    repo: &Repository,
    commit: &Commit,
    config: &Config,
) -> Result<(String, String)> {
    let parent = commit.parent(0).ok();
    let parent_tree = parent.as_ref().and_then(|p| p.tree().ok());
    let commit_tree = commit.tree()?;

    let mut opts = DiffOptions::new();
    opts.context_lines(3);

    let diff = repo.diff_tree_to_tree(
        parent_tree.as_ref(),
        Some(&commit_tree),
        Some(&mut opts),
    )?;

    // Generate diff stat
    let stats = diff.stats()?;
    let diff_stat = format!(
        "{} files changed, {} insertions(+), {} deletions(-)",
        stats.files_changed(),
        stats.insertions(),
        stats.deletions()
    );

    // Generate diff content
    let mut diff_content = String::new();
    let mut current_lines = 0;
    let max_lines = config.context.smart.truncate_at;

    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        if current_lines >= max_lines {
            return true;
        }

        let prefix = match line.origin() {
            '+' => "+",
            '-' => "-",
            ' ' => " ",
            _ => "",
        };

        if !prefix.is_empty() {
            if let Ok(content) = std::str::from_utf8(line.content()) {
                diff_content.push_str(prefix);
                diff_content.push_str(content);
                current_lines += 1;
            }
        }

        true
    })?;

    if current_lines >= max_lines {
        diff_content.push_str("\n... [truncated] ...\n");
    }

    Ok((diff_content, diff_stat))
}

/// Get list of affected files in a commit
pub fn get_affected_files(repo: &Repository, commit: &Commit) -> Result<Vec<String>> {
    let parent = commit.parent(0).ok();
    let parent_tree = parent.as_ref().and_then(|p| p.tree().ok());
    let commit_tree = commit.tree()?;

    let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&commit_tree), None)?;

    let mut files = Vec::new();

    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                files.push(path.to_string_lossy().to_string());
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(files)
}
