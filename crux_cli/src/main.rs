use anyhow::Result;
use args::Commands;
use clap::Parser;

use args::Cli;

mod args;
mod config;
mod diff;
mod doctor;
mod template;
mod workspace;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Doctor { .. }) => {
            doctor::doctor(&cli.template_dir, cli.verbose, cli.include_source_code)
        }
        None => Ok(()),
    }
}
