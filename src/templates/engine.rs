use anyhow::{Context, Result};
use handlebars::Handlebars;
use std::path::Path;

use crate::config::TemplatesConfig;
use crate::context::Context as GitContext;

/// Render a template with the given context
pub fn render_template(
    template_name: &str,
    context: &GitContext,
    config: &TemplatesConfig,
) -> Result<String> {
    let mut handlebars = Handlebars::new();

    // Try to load custom template first
    let template_content = load_template(template_name, config)?;

    handlebars
        .register_template_string(template_name, &template_content)
        .context("Failed to register template")?;

    // Build template data
    let data = build_template_data(context);

    handlebars
        .render(template_name, &data)
        .context("Failed to render template")
}

fn load_template(name: &str, config: &TemplatesConfig) -> Result<String> {
    // Try custom directory first
    let custom_path = expand_path(&config.directory).join(format!("{}.md", name));
    if custom_path.exists() {
        return std::fs::read_to_string(custom_path).context("Failed to read custom template");
    }

    // Fall back to built-in templates
    if config.fallback_builtin {
        return Ok(get_builtin_template(name).to_string());
    }

    anyhow::bail!("Template '{}' not found", name)
}

fn expand_path(path: &Path) -> std::path::PathBuf {
    let path_str = path.to_string_lossy();
    if path_str.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path_str[2..]);
        }
    }
    path.to_path_buf()
}

fn get_builtin_template(name: &str) -> &'static str {
    match name {
        "review" => include_str!("../../templates/review.md"),
        "changelog" => include_str!("../../templates/changelog.md"),
        "validate" => include_str!("../../templates/validate.md"),
        "summary" => include_str!("../../templates/summary.md"),
        _ => "# Unknown Template\n\n{{commit_message}}\n\n{{diff}}",
    }
}

fn build_template_data(context: &GitContext) -> serde_json::Value {
    serde_json::json!({
        "commit_hash": context.commit_hash,
        "commit_message": context.commit_message,
        "author": context.author,
        "date": context.date,
        "branch": context.branch,
        "diff": context.diff,
        "diff_stat": context.diff_stat,
        "staged_diff": context.staged_diff,
        "staged_count": context.staged_count,
        "affected_files": context.affected_files,
        "affected_packages": context.affected_packages,
        "recent_commits": context.recent_commits.iter().map(|c| {
            serde_json::json!({
                "hash": c.hash,
                "message": c.message,
                "author": c.author,
                "date": c.date,
            })
        }).collect::<Vec<_>>(),
    })
}

/// List available templates
pub fn list_templates(config: &TemplatesConfig) -> Result<Vec<String>> {
    let mut templates = vec![
        "review".to_string(),
        "changelog".to_string(),
        "validate".to_string(),
        "summary".to_string(),
    ];

    // Add custom templates
    let custom_dir = expand_path(&config.directory);
    if custom_dir.exists() {
        for entry in std::fs::read_dir(custom_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "md") {
                if let Some(stem) = path.file_stem() {
                    let name = stem.to_string_lossy().to_string();
                    if !templates.contains(&name) {
                        templates.push(name);
                    }
                }
            }
        }
    }

    Ok(templates)
}
