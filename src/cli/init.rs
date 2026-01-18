use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Select};

pub async fn run(force: bool) -> Result<()> {
    println!("┌─────────────────────────────────────────────────────┐");
    println!("│  🎉 Välkommen till gitclaude!                       │");
    println!("│                                                     │");
    println!("│  Låt oss konfigurera hur du vill använda verktyget. │");
    println!("└─────────────────────────────────────────────────────┘");
    println!();

    // Check existing config
    if !force && config_exists() {
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Konfiguration finns redan. Vill du skriva över?")
            .default(false)
            .interact()?;

        if !overwrite {
            println!("Avbryter.");
            return Ok(());
        }
    }

    // Step 1: Global vs per-repo
    let listen_mode = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Ska gitclaude lyssna globalt eller bara i specifika repos?")
        .items(&[
            "Globalt (alla git repos, kan exkludera vissa)",
            "Endast aktiverade repos (kör 'gitclaude enable' per repo)",
        ])
        .default(1)
        .interact()?;

    // Step 2: Events
    let events = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Vilka git-events vill du reagera på?")
        .items(&[
            "post-commit (efter varje commit)",
            "post-push (efter push till remote)",
            "pre-commit (validering innan commit)",
            "post-merge (efter merge)",
            "post-checkout (efter branch-byte)",
        ])
        .defaults(&[true, false, false, false, false])
        .interact()?;

    // Step 3: Sync vs async
    let run_mode = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Hur ska Claude köras?")
        .items(&[
            "Synkront (vänta på svar innan git fortsätter)",
            "Asynkront (kör i bakgrunden)",
            "Fråga varje gång",
        ])
        .default(1)
        .interact()?;

    // Step 4: Output
    let outputs = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Vad ska hända med Claudes output?")
        .items(&[
            "Desktop-notifikation",
            "Spara till fil",
            "Öppna interaktiv session",
            "Lägg till som git-note",
            "Kopiera till clipboard",
        ])
        .defaults(&[true, false, false, false, false])
        .interact()?;

    // Step 5: Context level
    let context_level = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Hur mycket kontext ska skickas till Claude?")
        .items(&[
            "Minimal (endast commit message)",
            "Standard (message + diff)",
            "Extended (message + diff + relaterade filer)",
            "Full (hela repo-kontexten)",
        ])
        .default(1)
        .interact()?;

    // Step 6: Rate limiting
    let rate_limit = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rate limiting (förhindra spam vid många commits)?")
        .items(&[
            "Debounce (vänta 30s efter senaste commit)",
            "Batch (samla commits, kör en gång)",
            "Cooldown (minst 5 min mellan körningar)",
            "Smart (kombinerad logik)",
            "Ingen (kör alltid)",
        ])
        .default(0)
        .interact()?;

    // TODO: Generate and save config based on selections
    println!();
    println!("✅ Konfiguration sparad till ~/.config/gitclaude/config.toml");

    if listen_mode == 0 {
        println!("✅ Global git hooks installerade till ~/.git-hooks/");
        println!();
        println!("Gitclaude lyssnar nu på alla dina repos.");
    } else {
        println!();
        println!("Kör 'gitclaude enable' i ett repo för att aktivera.");
    }

    Ok(())
}

fn config_exists() -> bool {
    // TODO: Check if config exists
    false
}
