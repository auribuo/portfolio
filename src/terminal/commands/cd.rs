use clap::{command, Parser};

/// Change directory
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cd {
    /// The path to change directory to
    pub(crate) path: String,
}
