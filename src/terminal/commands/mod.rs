use std::error::Error;

use clap::Parser;
use dioxus::signals::{Signal, Writable};

use crate::GlobalAppState;

mod cd;
mod clear;
mod help;
mod ls;
mod pwd;

#[derive(Debug, Clone)]
pub(crate) enum CommandResult {
    Unknown(String),
    Failed(String, String),
    Help(String),
    Pwd(String),
    Cd(String),
    Ls(String),
    Clear,
    None,
}

impl CommandResult {
    fn from_err<T>(value: T, cmd: &str) -> Self
    where
        T: Error,
    {
        CommandResult::Failed(
            cmd.to_string(),
            value
                .to_string()
                .replace("<", "&lt;")
                .replace(">", "&gt;")
                .replace("\n", "<br />"),
        )
    }
}

pub(crate) fn exec(cmd: String, mut state: Signal<GlobalAppState>) {
    if cmd.is_empty() {
        state.write().buffer_mut().process(CommandResult::None);
        return;
    }
    let cmd_parts = cmd.split(" ").collect::<Vec<_>>();
    let res = match *cmd_parts.get(0).unwrap() {
        "help" => match help::Help::try_parse_from(cmd_parts) {
            Ok(_) => CommandResult::Help(help::help(state().theme)),
            Err(err) => CommandResult::from_err(err, "help"),
        },
        "clear" => match clear::Clear::try_parse_from(cmd_parts) {
            Ok(_) => CommandResult::Clear,
            Err(err) => CommandResult::from_err(err, "clear"),
        },
        "pwd" => match pwd::Pwd::try_parse_from(cmd_parts) {
            Ok(_) => CommandResult::Pwd(pwd::pwd(state().filesystem)),
            Err(err) => CommandResult::from_err(err, "pwd"),
        },
        "cd" => match cd::Cd::try_parse_from(cmd_parts) {
            Ok(cd) => match state.write().filesystem_mut().cd(cd.path) {
                Ok(_) => CommandResult::Cd(cmd),
                Err(err) => CommandResult::Failed("cd".to_string(), err),
            },
            Err(err) => CommandResult::from_err(err, "cd"),
        },
        "ls" => match ls::Ls::try_parse_from(cmd_parts) {
            Ok(_) => CommandResult::Ls(ls::ls(state().filesystem, state().theme)),
            Err(err) => CommandResult::from_err(err, "ls"),
        },
        other => CommandResult::Unknown(other.to_string()),
    };
    state.write().buffer_mut().process(res);
}
