use clap::{command, Parser};

use crate::terminal::filesystem::Filesystem;

/// Print the working directory
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Pwd;

pub(crate) fn pwd(filesystem: &Filesystem) -> &'static str {
    filesystem.cwd()
}
