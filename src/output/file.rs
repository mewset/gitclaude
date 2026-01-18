use anyhow::Result;
use std::path::Path;

use crate::config::FileOutputConfig;

/// Save response to file
pub fn save_response(
    response: &str,
    event: &str,
    commit_hash: &str,
    repo_root: &Path,
    config: &FileOutputConfig,
) -> Result<std::path::PathBuf> {
    let output_dir = repo_root.join(&config.path);
    std::fs::create_dir_all(&output_dir)?;

    let filename = if config.timestamp {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        format!("{}_{}_{}.", event, commit_hash, timestamp)
    } else {
        format!("{}_{}.md", event, commit_hash)
    };

    let extension = match config.format.as_str() {
        "json" => "json",
        "plain" | "txt" => "txt",
        _ => "md",
    };

    let filepath = output_dir.join(format!("{}{}", filename, extension));

    let content = match config.format.as_str() {
        "json" => {
            serde_json::json!({
                "event": event,
                "commit": commit_hash,
                "timestamp": chrono::Local::now().to_rfc3339(),
                "response": response,
            })
            .to_string()
        }
        _ => response.to_string(),
    };

    std::fs::write(&filepath, content)?;

    Ok(filepath)
}
