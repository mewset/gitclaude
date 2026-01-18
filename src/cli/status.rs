use anyhow::Result;

pub async fn run(verbose: bool) -> Result<()> {
    println!("📊 Gitclaude Status");
    println!("─────────────────────");

    // TODO: Implement actual status checks
    println!("Global listening: ❌ Disabled");
    println!("Current repo:     ✅ Enabled");
    println!("Active events:    post-commit");
    println!("Rate limiting:    debounce (30s)");
    println!("Context level:    standard");
    println!("Output:           notify");

    if verbose {
        println!();
        println!("📁 Config Paths");
        println!("   Global: ~/.config/gitclaude/config.toml");
        println!("   Repo:   .gitclaude/config.toml");
        println!();
        println!("🪝 Installed Hooks");
        println!("   .git/hooks/post-commit → gitclaude");
    }

    Ok(())
}
