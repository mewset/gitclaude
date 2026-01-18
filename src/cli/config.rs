use anyhow::Result;

pub async fn run(global: bool, edit: bool) -> Result<()> {
    let config_path = if global {
        // TODO: Get actual XDG path
        "~/.config/gitclaude/config.toml"
    } else {
        ".gitclaude/config.toml"
    };

    if edit {
        // TODO: Open in $EDITOR
        println!("Opening {} in editor...", config_path);
    } else {
        // TODO: Print current config
        println!("Configuration: {}", config_path);
        println!();
        println!("# TODO: Print config contents");
    }

    Ok(())
}
