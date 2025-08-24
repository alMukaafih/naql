use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use naql_build::{BuildArgs, Builder};
use std::io::{IsTerminal, stdout};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser)]
#[command(name = "naql")]
#[command(bin_name = "naql")]
#[command(version, about)]
#[command(propagate_version = true)]
#[command(styles = CLAP_STYLING)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Build an acode plugin from vscode extension
    Build(BuildArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        .with_ansi(stdout().is_terminal())
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match cli.command {
        Command::Build(args) => {
            let mut builder = Builder::new(args);
            builder.build()?;
        }
    }

    Ok(())
}
