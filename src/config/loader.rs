use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::types::Config;

/// Load configuration with proper precedence:
/// 1. Repo-specific (.gitclaude/config.toml)
/// 2. Global (~/.config/gitclaude/config.toml)
pub fn load_config() -> Result<Config> {
    let global_config = load_global_config()?;
    let repo_config = load_repo_config()?;

    Ok(merge_configs(global_config, repo_config))
}

/// Load global configuration
pub fn load_global_config() -> Result<Option<Config>> {
    let path = global_config_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)
        .context("Failed to read global config")?;

    let config: Config = toml::from_str(&content)
        .context("Failed to parse global config")?;

    Ok(Some(config))
}

/// Load repository-specific configuration
pub fn load_repo_config() -> Result<Option<Config>> {
    let path = repo_config_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)
        .context("Failed to read repo config")?;

    let config: Config = toml::from_str(&content)
        .context("Failed to parse repo config")?;

    Ok(Some(config))
}

/// Merge configs with repo taking precedence
fn merge_configs(global: Option<Config>, repo: Option<Config>) -> Config {
    match (global, repo) {
        (None, None) => Config::default(),
        (Some(g), None) => g,
        (None, Some(r)) => r,
        (Some(_g), Some(r)) => {
            // TODO: Implement proper merging where repo overrides global
            // For now, just use repo config if it exists
            r
        }
    }
}

/// Get path to global config file
pub fn global_config_path() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("", "", "gitclaude")
        .context("Could not determine config directory")?;

    Ok(dirs.config_dir().join("config.toml"))
}

/// Get path to repo-specific config file
pub fn repo_config_path() -> Result<PathBuf> {
    let repo_root = find_repo_root()?;
    Ok(repo_root.join(".gitclaude").join("config.toml"))
}

/// Find the root of the current git repository
pub fn find_repo_root() -> Result<PathBuf> {
    let current = std::env::current_dir()?;

    let mut path = current.as_path();
    loop {
        if path.join(".git").exists() {
            return Ok(path.to_path_buf());
        }
        match path.parent() {
            Some(parent) => path = parent,
            None => anyhow::bail!("Not in a git repository"),
        }
    }
}

/// Save configuration to file
pub fn save_config(config: &Config, path: &Path) -> Result<()> {
    let content = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .context("Failed to create config directory")?;
    }

    std::fs::write(path, content)
        .context("Failed to write config file")?;

    Ok(())
}
