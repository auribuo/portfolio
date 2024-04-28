use clap::{command, Parser};

use crate::ui::themes::TerminalTheme;

/// List available commands
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Help;

pub(crate) fn help(theme: &TerminalTheme) -> String {
    format!(
        r#"
        <b style='{}'>Available commands:</b>
        <br />
        - help <br />
        - pwd <br />
        - cd <br />
        - ls <br />
        - theme <br />
        - history <br />
        <br />
        Run [command] --help to get help for a specific command"#,
        theme.peach.style_text()
    )
}
