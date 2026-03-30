mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "amber", about = "Amber - Config Management")]
#[command(after_help = "Versioning: Use git directly in ~/.config/amber/
  git diff                              - See uncommitted changes
  git commit -am 'message'              - Save a checkpoint
  git log --oneline                     - List checkpoints
  git checkout <id> -- domains/<app>/config  - Restore")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Theme {
        color: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
    Sync,
    Status,
    Grave,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Theme { color, dry_run } => commands::theme::run(color, dry_run),
        Commands::Sync => commands::sync::run(),
        Commands::Status => commands::sync::status(),
        Commands::Grave => {
            if let Err(e) = commands::grave::run() {
                eprintln!("Grave Error: {}", e);
                std::process::exit(1);
            }
            Ok(())
        }
    }
}
