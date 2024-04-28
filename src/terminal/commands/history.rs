use clap::{command, Parser};

/// View and clear the command history
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct History {
    #[arg(long)]
    pub(crate) clear: bool,
}
