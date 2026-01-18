use anyhow::Result;
use std::path::Path;

/// Install hooks for a repository
pub fn install_hooks(repo_path: &Path, events: &[String]) -> Result<()> {
    let hooks_dir = repo_path.join(".git").join("hooks");

    for event in events {
        let hook_path = hooks_dir.join(event);
        let script = super::scripts::generate_hook_script(event);

        std::fs::write(&hook_path, script)?;

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&hook_path, perms)?;
        }
    }

    Ok(())
}

/// Remove hooks from a repository
pub fn remove_hooks(repo_path: &Path, events: &[String]) -> Result<()> {
    let hooks_dir = repo_path.join(".git").join("hooks");

    for event in events {
        let hook_path = hooks_dir.join(event);
        if hook_path.exists() {
            // Check if it's our hook before removing
            let content = std::fs::read_to_string(&hook_path)?;
            if content.contains("gitclaude") {
                std::fs::remove_file(hook_path)?;
            }
        }
    }

    Ok(())
}

/// Install global hooks
pub fn install_global_hooks(events: &[String]) -> Result<()> {
    let hooks_dir = directories::BaseDirs::new()
        .map(|d| d.home_dir().join(".git-hooks"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.git-hooks"));

    std::fs::create_dir_all(&hooks_dir)?;

    for event in events {
        let hook_path = hooks_dir.join(event);
        let script = super::scripts::generate_hook_script(event);
        std::fs::write(&hook_path, script)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&hook_path, perms)?;
        }
    }

    // Set global hooks path
    std::process::Command::new("git")
        .args(["config", "--global", "core.hooksPath", hooks_dir.to_str().unwrap()])
        .output()?;

    Ok(())
}
