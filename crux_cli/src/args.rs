use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "crux",
    bin_name = "crux",
    author,
    version,
    about,
    long_about = None,
    arg_required_else_help(true),
    propagate_version = true
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(long, short, action = ArgAction::Count)]
    pub verbose: u8,

    #[arg(long, short, default_value = "false")]
    pub include_source_code: bool,

    /// temporary
    #[arg(long, short)]
    pub template_dir: PathBuf,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[command(visible_alias = "doc")]
    Doctor {
        #[arg(long, short)]
        fix: Option<PathBuf>,
    },
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn test_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
