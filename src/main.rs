use clap::{Parser, Subcommand};

mod cli;
mod config;
mod hooks;
mod context;
mod rate_limit;
mod claude;
mod output;
mod templates;

#[derive(Parser)]
#[command(name = "gitclaude")]
#[command(author, version, about = "Git hook daemon that triggers Claude Code sessions")]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive setup wizard
    Init {
        /// Force re-initialization
        #[arg(short, long)]
        force: bool,
    },

    /// Enable gitclaude in current repository
    Enable {
        /// Enable specific events only
        #[arg(short, long)]
        events: Option<Vec<String>>,
    },

    /// Disable gitclaude in current repository
    Disable {
        /// Keep configuration file
        #[arg(short, long)]
        keep_config: bool,
    },

    /// Show or edit configuration
    Config {
        /// Edit global configuration
        #[arg(short, long)]
        global: bool,

        /// Open in editor
        #[arg(short, long)]
        edit: bool,
    },

    /// Show current status
    Status {
        /// Show verbose status
        #[arg(short, long)]
        verbose: bool,
    },

    /// View response history
    Logs {
        /// Number of entries to show
        #[arg(short, long, default_value = "10")]
        count: usize,

        /// Filter by event type
        #[arg(short, long)]
        event: Option<String>,
    },

    /// Manually trigger an event
    Run {
        /// Event to trigger
        event: String,

        /// Dry run (show what would be sent)
        #[arg(short, long)]
        dry_run: bool,
    },

    /// Manage templates
    Templates {
        #[command(subcommand)]
        action: Option<TemplateActions>,
    },
}

#[derive(Subcommand)]
enum TemplateActions {
    /// List available templates
    List,
    /// Edit a template
    Edit { name: String },
    /// Create a new template
    New { name: String },
    /// Show template content
    Show { name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();

    match cli.command {
        Commands::Init { force } => {
            cli::init::run(force).await?;
        }
        Commands::Enable { events } => {
            cli::enable::run(events).await?;
        }
        Commands::Disable { keep_config } => {
            cli::disable::run(keep_config).await?;
        }
        Commands::Config { global, edit } => {
            cli::config::run(global, edit).await?;
        }
        Commands::Status { verbose } => {
            cli::status::run(verbose).await?;
        }
        Commands::Logs { count, event } => {
            cli::logs::run(count, event).await?;
        }
        Commands::Run { event, dry_run } => {
            cli::run::run(&event, dry_run).await?;
        }
        Commands::Templates { action } => {
            cli::templates::run(action).await?;
        }
    }

    Ok(())
}
