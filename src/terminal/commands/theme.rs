use clap::{command, Parser, ValueEnum};

/// Clear the console
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Theme {
    #[arg(value_enum, short, long)]
    pub(crate) theme: ThemeName,
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum ThemeName {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}
