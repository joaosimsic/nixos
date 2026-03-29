mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "amber", about = "Amber manager")]
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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Theme { color, dry_run } => commands::theme::run(color, dry_run),
        Commands::Sync => commands::sync::run(),
    }
}

