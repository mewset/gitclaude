use anyhow::Result;

pub async fn run(events: Option<Vec<String>>) -> Result<()> {
    // TODO: Implement enable logic
    // 1. Check if in git repo
    // 2. Install hooks
    // 3. Create .gitclaude/config.toml if needed

    println!("✅ Gitclaude aktiverat i detta repo");

    if let Some(events) = events {
        println!("   Events: {}", events.join(", "));
    } else {
        println!("   Events: post-commit (default)");
    }

    Ok(())
}
