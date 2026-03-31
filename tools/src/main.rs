mod amber_dir;
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
    Grave {
        #[arg(long, short)]
        switch: bool,
        #[command(subcommand)]
        sub: Option<GraveSubcommand>,
    },
}

#[derive(Subcommand)]
enum GraveSubcommand {
    Switch,
    Reattach,
    Toggle,
    Clean {
        #[arg(long, short, default_value_t = 10)]
        keep: usize,
    },
    Kill {
        #[arg(long, short, default_value_t = 0)]
        keep: usize,
    },
    List,
    #[command(name = "list-lines")]
    ListLines {
        #[arg(long, short)]
        exclude_current: bool,
    },
    Delete {
        name: String,
    },
    Preview {
        name: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Theme { color, dry_run } => commands::theme::run(color, dry_run),
        Commands::Sync => commands::sync::run(),
        Commands::Status => commands::sync::status(),
        Commands::Grave { switch, sub } => {
            let exe = std::env::current_exe()?;
            let r = match sub {
                None => commands::grave::run_main(switch),
                Some(GraveSubcommand::Switch) => commands::grave::run_switch(),
                Some(GraveSubcommand::Reattach) => commands::grave::run_reattach(),
                Some(GraveSubcommand::Toggle) => commands::grave::run_toggle(&exe),
                Some(GraveSubcommand::Clean { keep }) => commands::grave::run_clean(keep),
                Some(GraveSubcommand::Kill { keep }) => commands::grave::run_kill(keep),
                Some(GraveSubcommand::List) => commands::grave::list_sessions_print(),
                Some(GraveSubcommand::ListLines { exclude_current }) => {
                    commands::grave::print_list_display(exclude_current, false)
                }
                Some(GraveSubcommand::Delete { name }) => commands::grave::delete_session(&name),
                Some(GraveSubcommand::Preview { name }) => commands::grave::print_preview(&name),
            };
            if let Err(e) = r {
                eprintln!("Grave: {}", e);
                std::process::exit(1);
            }
            Ok(())
        }
    }
}
