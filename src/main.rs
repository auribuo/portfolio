#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use terminal::{buffer::TerminalBuffer, filesystem::Filesystem};
use ui::themes::TerminalTheme;

use crate::ui::components::{CmdOutput, Prompt};

pub(crate) mod terminal;
pub(crate) mod ui;

#[macro_use]
extern crate log;

#[derive(Debug, Clone)]
pub(crate) struct GlobalAppState {
    theme: TerminalTheme,
    buffer: TerminalBuffer,
    filesystem: Filesystem,
}

impl GlobalAppState {
    pub(crate) fn theme(&self) -> &TerminalTheme {
        &self.theme
    }

    pub(crate) fn cwd(&self) -> String {
        self.filesystem.cwd().clone()
    }

    pub(crate) fn buffer_mut(&mut self) -> &mut TerminalBuffer {
        &mut self.buffer
    }

    pub(crate) fn filesystem_mut(&mut self) -> &mut Filesystem {
        &mut self.filesystem
    }
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    let state = GlobalAppState {
        theme: ui::themes::MOCHA,
        buffer: TerminalBuffer::new(),
        filesystem: Filesystem::new(),
    };
    use_context_provider(|| Signal::new(state));
    let state = consume_context::<Signal<GlobalAppState>>();

    eval("document.body.onclick = () => {document.getElementById('input').focus()}");
    eval("document.body.onload  = () => {document.getElementById('input').focus()}");

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        main {
            id: "main-container",
            style: "background-color: {state().theme().crust.hex()}; color: {state().theme().text.hex()}",
            if !state().buffer_mut().is_empty() {
                div { id: "history",
                    for res in state().buffer_mut().commands() {
                        CmdOutput { cmd: &res.cmd, cmd_output: res.output.render(), failed: res.failed }
                    }
                }
            }
            Prompt {}
        }
    }
}
