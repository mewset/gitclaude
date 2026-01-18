use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::process::Command;

use crate::config::ClaudeConfig;

/// Response from Claude
#[derive(Debug)]
pub struct ClaudeResponse {
    pub content: String,
    pub success: bool,
}

/// Run Claude with the given prompt
pub async fn run_claude(prompt: &str, config: &ClaudeConfig) -> Result<ClaudeResponse> {
    let binary = config
        .binary
        .clone()
        .unwrap_or_else(|| which::which("claude").unwrap_or_else(|_| "claude".into()));

    let mut cmd = Command::new(binary);

    cmd.arg("--print")
        .arg(prompt);

    // Add extra args if configured
    for arg in &config.extra_args {
        cmd.arg(arg);
    }

    cmd.stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(config.timeout),
        cmd.output(),
    )
    .await
    .context("Claude timed out")?
    .context("Failed to run claude")?;

    let content = String::from_utf8_lossy(&output.stdout).to_string();
    let success = output.status.success();

    if !success {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::warn!("Claude returned non-zero: {}", stderr);
    }

    Ok(ClaudeResponse { content, success })
}

/// Run Claude in async/background mode
pub async fn run_claude_async(prompt: &str, config: &ClaudeConfig) -> Result<()> {
    let binary = config
        .binary
        .clone()
        .unwrap_or_else(|| which::which("claude").unwrap_or_else(|_| "claude".into()));

    let mut cmd = Command::new(binary);

    cmd.arg("--print")
        .arg(prompt);

    for arg in &config.extra_args {
        cmd.arg(arg);
    }

    // Detach from current process
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    cmd.spawn().context("Failed to spawn claude")?;

    Ok(())
}

/// Check if Claude CLI is available
pub fn is_claude_available() -> bool {
    which::which("claude").is_ok()
}
