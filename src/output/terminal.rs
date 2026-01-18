use anyhow::Result;
use std::process::Command;

use crate::config::TerminalConfig;

/// Open response in a terminal window
pub fn open_in_terminal(response: &str, config: &TerminalConfig) -> Result<()> {
    // Create a temporary file with the response
    let temp_file = std::env::temp_dir().join("gitclaude_response.md");
    std::fs::write(&temp_file, response)?;

    let terminal = &config.terminal;

    match terminal.as_str() {
        "alacritty" => {
            Command::new("alacritty")
                .args(["-e", "less", temp_file.to_str().unwrap()])
                .spawn()?;
        }
        "kitty" => {
            Command::new("kitty")
                .args(["less", temp_file.to_str().unwrap()])
                .spawn()?;
        }
        "ghostty" => {
            Command::new("ghostty")
                .args(["-e", "less", temp_file.to_str().unwrap()])
                .spawn()?;
        }
        "foot" => {
            Command::new("foot")
                .args(["less", temp_file.to_str().unwrap()])
                .spawn()?;
        }
        _ => {
            // Try to use $TERMINAL or fall back to xterm
            let terminal_env = std::env::var("TERMINAL").unwrap_or_else(|_| "xterm".to_string());
            Command::new(terminal_env)
                .args(["-e", "less", temp_file.to_str().unwrap()])
                .spawn()?;
        }
    }

    Ok(())
}
