use clap::{command, Parser};

use crate::{
    terminal::filesystem::{Filesystem, LsResult, LsResultType},
    ui::themes::TerminalTheme,
};

/// List files in the current directory
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Ls;

fn longest_size(entries: &Vec<LsResult>) -> usize {
    entries
        .iter()
        .map(|lsr| lsr.size().map_or("-".to_string(), |x| x.to_string()).len())
        .max()
        .unwrap_or(1)
}

fn pad(num: Option<u64>, max: usize) -> String {
    let str = num.map_or("-".to_string(), |x| x.to_string());

    if str.len() == max {
        return str;
    }

    let spaces = " "
        .chars()
        .into_iter()
        .cycle()
        .take((max - str.len()) as usize)
        .collect::<String>();

    spaces + &str
}

fn format_name(name: &str, ty: &LsResultType, theme: &TerminalTheme) -> String {
    match ty {
        LsResultType::Directory => {
            format!(
                "<span style='{}'>{}/</span>",
                theme.peach.style_text(),
                name
            )
        }
        LsResultType::File => {
            format!("<span style='{}'>{}</span>", theme.text.style_text(), name)
        }
        LsResultType::Link => {
            format!(
                "<span style='{}'>{}</span>",
                theme.sapphire.style_text(),
                name
            )
        }
    }
}

pub(crate) fn ls(filesystem: &Filesystem, theme: &TerminalTheme) -> String {
    let res = match filesystem.ls() {
        Ok(entries) => {
            let max_n = longest_size(&entries);
            entries
                .iter()
                .map(|entry| {
                    format!(
                        "{} {} {}",
                        entry.permissions(),
                        pad(entry.size(), max_n),
                        format_name(entry.name(), &entry.ty(), &theme)
                    )
                })
                .fold("".to_string(), |acc, e| acc + &e + "<br />")
        }
        Err(err) => err,
    };
    res
}
