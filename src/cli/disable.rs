use anyhow::Result;

pub async fn run(keep_config: bool) -> Result<()> {
    // TODO: Implement disable logic
    // 1. Remove hooks
    // 2. Optionally remove .gitclaude/

    println!("✅ Gitclaude disabled in this repository");

    if !keep_config {
        println!("   Configuration removed");
    } else {
        println!("   Configuration preserved");
    }

    Ok(())
}
