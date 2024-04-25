use clap::{command, Parser};

/// Clear the console
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Clear;
