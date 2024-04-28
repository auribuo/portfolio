use std::error::Error;

use clap::Parser;
use dioxus::{
    prelude::consume_context,
    signals::{Signal, Writable},
};

use crate::{
    ui::themes::{LATTE, MOCHA},
    AppState,
};

mod cd;
mod clear;
mod help;
mod history;
mod ls;
mod pwd;
mod theme;

#[derive(Debug, Clone)]
pub(crate) enum CommandResult {
    Unknown(String),
    Failed(String, String),
    Help(String),
    Pwd(&'static str),
    Cd(String),
    Ls(String),
    Theme(String),
    History(String, String),
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

pub(crate) fn exec(mut cmd: String) {
    let mut state = consume_context::<Signal<AppState>>();

    if cmd.is_empty() {
        state.write().buffer_mut().process(CommandResult::None);
        return;
    }
    cmd = cmd.trim().to_string();
    let cmd_parts = cmd.split(" ").collect::<Vec<_>>();
    let res = match *cmd_parts.get(0).unwrap() {
        "help" => match help::Help::try_parse_from(cmd_parts) {
            Ok(_) => CommandResult::Help(help::help(state().theme())),
            Err(err) => CommandResult::from_err(err, "help"),
        },
        "clear" => match clear::Clear::try_parse_from(cmd_parts) {
            Ok(_) => CommandResult::Clear,
            Err(err) => CommandResult::from_err(err, "clear"),
        },
        "pwd" => match pwd::Pwd::try_parse_from(cmd_parts) {
            Ok(_) => CommandResult::Pwd(pwd::pwd(state().fs())),
            Err(err) => CommandResult::from_err(err, "pwd"),
        },
        "cd" => match cd::Cd::try_parse_from(cmd_parts) {
            Ok(cd) => match state.write().fs_mut().cd(cd.path) {
                Ok(_) => CommandResult::Cd(cmd),
                Err(err) => CommandResult::Failed("cd".to_string(), err),
            },
            Err(err) => CommandResult::from_err(err, "cd"),
        },
        "theme" => match theme::Theme::try_parse_from(cmd_parts) {
            Ok(theme) => {
                let theme = match theme.theme {
                    theme::ThemeName::Latte => LATTE,
                    theme::ThemeName::Frappe => todo!(),
                    theme::ThemeName::Macchiato => todo!(),
                    theme::ThemeName::Mocha => MOCHA,
                };
                state.write().change_theme(theme);
                CommandResult::Theme(cmd)
            }
            Err(err) => CommandResult::from_err(err, "cd"),
        },
        "ls" => match ls::Ls::try_parse_from(cmd_parts) {
            Ok(_) => CommandResult::Ls(ls::ls(state().fs(), state().theme())),
            Err(err) => CommandResult::from_err(err, "ls"),
        },
        "history" => match history::History::try_parse_from(cmd_parts) {
            Ok(history) => {
                if history.clear {
                    state.write().history_mut().clear();
                    state.write().update_history();
                }

                CommandResult::History(
                    cmd,
                    state()
                        .history()
                        .entries()
                        .iter()
                        .fold("".to_string(), |acc, e| acc + e + "<br />"),
                )
            }
            Err(err) => CommandResult::from_err(err, "clear-history"),
        },
        other => CommandResult::Unknown(other.to_string()),
    };
    state.write().buffer_mut().process(res);
}
