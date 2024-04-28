#![allow(non_snake_case)]

use dioxus::prelude::*;
use gloo_storage::Storage;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use terminal::{buffer::TerminalBuffer, filesystem::Filesystem, history::History};
use ui::themes::TerminalTheme;
use web_sys::js_sys::Function;

use crate::ui::{
    components::{CmdOutput, Prompt},
    themes::{LATTE, MOCHA},
};

pub(crate) mod projects;
pub(crate) mod terminal;
pub(crate) mod ui;

#[macro_use]
extern crate log;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LocalStorageSettings {
    theme: String,
    history: Vec<String>,
}

impl Default for LocalStorageSettings {
    fn default() -> Self {
        Self {
            theme: "mocha".to_string(),
            history: vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct AppState {
    theme: TerminalTheme,
    buffer: TerminalBuffer,
    filesystem: Filesystem,
    history: History,
    localstorage: LocalStorageSettings,
}

impl AppState {
    pub(crate) fn theme(&self) -> &TerminalTheme {
        &self.theme
    }

    pub(crate) fn change_theme(&mut self, theme: TerminalTheme) {
        self.localstorage.theme = theme.name.to_string();
        let _ = gloo_storage::LocalStorage::set("settings", self.localstorage.clone());
        self.theme = theme
    }

    pub(crate) fn buffer(&self) -> &TerminalBuffer {
        &self.buffer
    }

    pub(crate) fn buffer_mut(&mut self) -> &mut TerminalBuffer {
        &mut self.buffer
    }

    pub(crate) fn fs(&self) -> &Filesystem {
        &self.filesystem
    }

    pub(crate) fn fs_mut(&mut self) -> &mut Filesystem {
        &mut self.filesystem
    }

    pub(crate) fn history(&self) -> &History {
        &self.history
    }

    pub(crate) fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    pub(crate) fn update_history(&mut self) {
        self.localstorage.history = self.history.entries().clone();
        let _ = gloo_storage::LocalStorage::set("settings", self.localstorage.clone());
    }
}

#[component]
fn App() -> Element {
    eval("document.body.onclick = () => {document.getElementById('input').focus()}");
    eval(
        r#"
        document.body.onload = () => {
            document.getElementById('input').focus()
        }
        "#,
    );

    let settings =
        if let Ok(settings) = gloo_storage::LocalStorage::get::<LocalStorageSettings>("settings") {
            settings
        } else {
            LocalStorageSettings::default()
        };

    use_context_provider(|| {
        Signal::new(AppState {
            theme: match settings.theme.as_str() {
                "latte" => LATTE,
                _ => MOCHA,
            },
            buffer: TerminalBuffer::new(),
            filesystem: Filesystem::new(),
            history: History::from(&settings),
            localstorage: settings,
        })
    });

    let document = web_sys::window().unwrap().document().unwrap();

    if let Some(elem) = document.get_element_by_id("history") {
        let mut observer_options = web_sys::MutationObserverInit::new();
        observer_options
            .attributes(true)
            .child_list(true)
            .character_data(true);

        let observer = web_sys::MutationObserver::new(&Function::new_with_args(
            "mutations",
            "console.log('test');window.scrollTo(0,document.body.scrollHeight)",
        ))
        .unwrap();
        observer
            .observe_with_options(&elem, &observer_options)
            .unwrap();
    }

    let state = consume_context::<Signal<AppState>>();

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        main {
            id: "main-container",
            style: "background-color: {state().theme().crust.hex()}; color: {state().theme().text.hex()}",
            if !state().buffer().is_empty() {
                div { id: "history",
                    for res in state().buffer().commands() {
                        CmdOutput { cmd: &res.cmd, cmd_output: res.output.render(), failed: res.failed }
                    }
                }
            }
            Prompt {}
            input {
                id: "hidden-tab",
                tabindex: 0,
                onfocus: |_| {
                    eval("document.getElementById('input').focus()");
                }
            }
        }
    }
}
