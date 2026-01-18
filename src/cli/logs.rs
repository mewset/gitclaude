use anyhow::Result;

pub async fn run(count: usize, event: Option<String>) -> Result<()> {
    println!("📜 Senaste {} responses", count);

    if let Some(ref e) = event {
        println!("   (filtrerat på: {})", e);
    }

    println!("─────────────────────────────");

    // TODO: Read actual logs from .gitclaude/responses/
    println!();
    println!("2024-01-15 14:32:01 | post-commit | abc1234");
    println!("✅ Ser bra ut! Inga uppenbara problem.");
    println!();
    println!("2024-01-15 13:15:42 | post-commit | def5678");
    println!("💡 Överväg att extrahera duplicerad logik på rad 45-52.");
    println!();

    Ok(())
}
