use anyhow::Result;

pub async fn run(keep_config: bool) -> Result<()> {
    // TODO: Implement disable logic
    // 1. Remove hooks
    // 2. Optionally remove .gitclaude/

    println!("✅ Gitclaude inaktiverat i detta repo");

    if !keep_config {
        println!("   Konfiguration borttagen");
    } else {
        println!("   Konfiguration bevarad");
    }

    Ok(())
}
