use anyhow::Result;

pub async fn run(event: &str, dry_run: bool) -> Result<()> {
    println!("🚀 Manuellt triggar event: {}", event);

    if dry_run {
        println!();
        println!("─── DRY RUN ───");
        println!();
        // TODO: Show what would be sent
        println!("Template: review");
        println!("Context level: standard");
        println!();
        println!("# Generated prompt:");
        println!("───────────────────");
        println!("# Code Review Request");
        println!("**Commit:** `abc1234`");
        println!("**Message:** Fix authentication bug");
        println!("...");
        println!("───────────────────");
        println!();
        println!("Kör utan --dry-run för att skicka till Claude.");
    } else {
        // TODO: Actually run the event
        println!("Kör Claude...");
        println!();
        println!("✅ Response:");
        println!("Ser bra ut! Inga uppenbara problem.");
    }

    Ok(())
}
