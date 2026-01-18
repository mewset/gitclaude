use anyhow::Result;
use std::path::Path;

/// Monorepo types we can detect
#[derive(Debug, Clone, PartialEq)]
pub enum MonorepoType {
    Pnpm,
    Lerna,
    Npm,
    Yarn,
    CargoWorkspace,
    GoWorkspace,
    Generic,
    None,
}

/// Detect if repository is a monorepo
pub fn detect_monorepo_type(repo_root: &Path) -> MonorepoType {
    if repo_root.join("pnpm-workspace.yaml").exists() {
        return MonorepoType::Pnpm;
    }

    if repo_root.join("lerna.json").exists() {
        return MonorepoType::Lerna;
    }

    if repo_root.join("package.json").exists() {
        // Check for workspaces in package.json
        if let Ok(content) = std::fs::read_to_string(repo_root.join("package.json")) {
            if content.contains("\"workspaces\"") {
                return MonorepoType::Npm;
            }
        }
    }

    if repo_root.join("Cargo.toml").exists() {
        if let Ok(content) = std::fs::read_to_string(repo_root.join("Cargo.toml")) {
            if content.contains("[workspace]") {
                return MonorepoType::CargoWorkspace;
            }
        }
    }

    if repo_root.join("go.work").exists() {
        return MonorepoType::GoWorkspace;
    }

    // Check for common package directories
    for dir in &["packages", "apps", "libs", "crates", "services"] {
        if repo_root.join(dir).is_dir() {
            return MonorepoType::Generic;
        }
    }

    MonorepoType::None
}

/// Detect which packages are affected by changed files
pub fn detect_affected_packages(
    repo_root: &Path,
    affected_files: &[String],
) -> Result<Vec<String>> {
    let monorepo_type = detect_monorepo_type(repo_root);

    if monorepo_type == MonorepoType::None {
        return Ok(vec![]);
    }

    let mut packages = std::collections::HashSet::new();

    for file in affected_files {
        if let Some(package) = find_package_for_file(repo_root, file) {
            packages.insert(package);
        }
    }

    Ok(packages.into_iter().collect())
}

/// Find which package a file belongs to
fn find_package_for_file(repo_root: &Path, file: &str) -> Option<String> {
    let file_path = Path::new(file);
    let components: Vec<_> = file_path.components().collect();

    // Check common package directory patterns
    for (i, component) in components.iter().enumerate() {
        let dir_name = component.as_os_str().to_string_lossy();

        if ["packages", "apps", "libs", "crates", "services"].contains(&dir_name.as_ref()) {
            // Next component is the package name
            if let Some(package_component) = components.get(i + 1) {
                return Some(package_component.as_os_str().to_string_lossy().to_string());
            }
        }
    }

    // Check for Cargo.toml or package.json in parent directories
    let mut current = repo_root.join(file);
    while let Some(parent) = current.parent() {
        if parent == repo_root {
            break;
        }

        if parent.join("Cargo.toml").exists() || parent.join("package.json").exists() {
            return parent.file_name().map(|n| n.to_string_lossy().to_string());
        }

        current = parent.to_path_buf();
    }

    None
}
