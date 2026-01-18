use anyhow::Result;
use crate::TemplateActions;

pub async fn run(action: Option<TemplateActions>) -> Result<()> {
    match action {
        None | Some(TemplateActions::List) => {
            println!("📝 Tillgängliga templates");
            println!("─────────────────────────");
            println!();
            println!("  review      Code review efter commit");
            println!("  changelog   Generera changelog vid push");
            println!("  validate    Validera före commit (blocking)");
            println!("  summary     Sammanfatta merge");
            println!("  context     Branch-kontext vid checkout");
            println!();
            println!("Kör 'gitclaude templates show <name>' för att se innehåll.");
        }
        Some(TemplateActions::Show { name }) => {
            println!("📄 Template: {}", name);
            println!("─────────────────────────");
            // TODO: Read and show actual template
            println!("# TODO: Show template contents");
        }
        Some(TemplateActions::Edit { name }) => {
            println!("Öppnar template '{}' i editor...", name);
            // TODO: Open in $EDITOR
        }
        Some(TemplateActions::New { name }) => {
            println!("Skapar ny template '{}'...", name);
            // TODO: Create template file and open in editor
        }
    }

    Ok(())
}
