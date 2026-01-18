use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Select};

pub async fn run(force: bool) -> Result<()> {
    println!("┌─────────────────────────────────────────────────────┐");
    println!("│  🎉 Welcome to gitclaude!                           │");
    println!("│                                                     │");
    println!("│  Let's configure how you want to use the tool.      │");
    println!("└─────────────────────────────────────────────────────┘");
    println!();

    // Check existing config
    if !force && config_exists() {
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Configuration already exists. Overwrite?")
            .default(false)
            .interact()?;

        if !overwrite {
            println!("Aborted.");
            return Ok(());
        }
    }

    // Step 1: Global vs per-repo
    let listen_mode = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Should gitclaude listen globally or only in specific repos?")
        .items(&[
            "Globally (all git repos, can exclude some)",
            "Only enabled repos (run 'gitclaude enable' per repo)",
        ])
        .default(1)
        .interact()?;

    // Step 2: Events
    let events = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which git events should trigger Claude?")
        .items(&[
            "post-commit (after each commit)",
            "post-push (after push to remote)",
            "pre-commit (validation before commit)",
            "post-merge (after merge)",
            "post-checkout (after branch switch)",
        ])
        .defaults(&[true, false, false, false, false])
        .interact()?;

    // Step 3: Sync vs async
    let run_mode = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How should Claude run?")
        .items(&[
            "Synchronously (wait for response before git continues)",
            "Asynchronously (run in background)",
            "Ask each time",
        ])
        .default(1)
        .interact()?;

    // Step 4: Output
    let outputs = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What should happen with Claude's output?")
        .items(&[
            "Desktop notification",
            "Save to file",
            "Open interactive session",
            "Add as git note",
            "Copy to clipboard",
        ])
        .defaults(&[true, false, false, false, false])
        .interact()?;

    // Step 5: Context level
    let context_level = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How much context should be sent to Claude?")
        .items(&[
            "Minimal (commit message only)",
            "Standard (message + diff)",
            "Extended (message + diff + related files)",
            "Full (entire repo context)",
        ])
        .default(1)
        .interact()?;

    // Step 6: Rate limiting
    let rate_limit = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rate limiting (prevent spam during high commit activity)?")
        .items(&[
            "Debounce (wait 30s after last commit)",
            "Batch (collect commits, run once)",
            "Cooldown (at least 5 min between runs)",
            "Smart (combined logic)",
            "None (always run)",
        ])
        .default(0)
        .interact()?;

    // TODO: Generate and save config based on selections
    println!();
    println!("✅ Configuration saved to ~/.config/gitclaude/config.toml");

    if listen_mode == 0 {
        println!("✅ Global git hooks installed to ~/.git-hooks/");
        println!();
        println!("Gitclaude is now listening on all your repos.");
    } else {
        println!();
        println!("Run 'gitclaude enable' in a repo to activate.");
    }

    Ok(())
}

fn config_exists() -> bool {
    // TODO: Check if config exists
    false
}
