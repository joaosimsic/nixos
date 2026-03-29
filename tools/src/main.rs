mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "amber", about = "Amber dotfiles manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the color theme
    Theme {
        /// Base hex color e.g. #ff6600. Reads from ./theme if omitted.
        color: Option<String>,
        /// Print palette without writing any files
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Theme { color, dry_run } => commands::theme::run(color, dry_run),
    }
}