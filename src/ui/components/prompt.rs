use dioxus::core_macro::component;
use dioxus::prelude::*;

use crate::AppState;

#[component]
pub(crate) fn Prompt() -> Element {
    let mut state = consume_context::<Signal<AppState>>();
    let mut command = use_signal(|| "".to_string());
    let mut typed = use_signal(|| "".to_string());
    let mut hidden_buf = use_signal(|| "".to_string());

    let last_failed = state()
        .buffer()
        .commands()
        .last()
        .map_or(false, |cmd| cmd.failed);

    rsx! {
        div { id: "prompt-container",
            div { id: "prompt",
                PromptText { command_failed: last_failed }
            }
            div { id: "input-container",
                input {
                    id: "input",
                    style: "color: {state().theme().text.hex()}",
                    value: command,
                    oninput: move |event| command.set(event.value()),
                    onkeydown: move |event| {
                        match event.data.key() {
                            Key::Enter => {
                                crate::terminal::commands::exec(command());
                                state.write().history_mut().push(command());
                                state.write().update_history();
                                command.set("".to_string());
                                typed.set("".to_string());
                                hidden_buf.set("".to_string());
                            }
                            Key::Tab => {
                                eval("document.getElementById('input').focus()");
                            }
                            Key::ArrowUp => {
                                if let Some(cmd) = state.write().history_mut().nav_back() {
                                    if typed().is_empty() {
                                        typed.set(hidden_buf())
                                    }
                                    command.set(cmd.clone());
                                }
                            }
                            Key::ArrowDown => {
                                if let Some(cmd) = state.write().history_mut().nav_front() {
                                    command.set(cmd.clone());
                                } else {
                                    command.set(typed());
                                }
                            }
                            Key::Character(chr) => {
                                if typed().is_empty() {
                                    hidden_buf.set(hidden_buf() + &chr);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn PromptText(command_failed: bool) -> Element {
    let state = consume_context::<Signal<AppState>>();
    let dollar_color = if !command_failed {
        state().theme().green.hex()
    } else {
        state().theme().red.hex()
    };
    rsx! {
        span { style: "color: {state().theme().sapphire.hex()}", "user@aureliobuonomo.it" }
        span { style: "white-space: pre", ": " }
        span { style: "color: {state().theme().red.hex()}", {state().fs().cwd()} }
        span { style: "white-space: pre; color: {dollar_color}", " ❯ " }
    }
}

#[component]
pub(crate) fn SimplePromptText(command_failed: bool) -> Element {
    let state = consume_context::<Signal<AppState>>();
    let dollar_color = if !command_failed {
        state().theme().green.hex()
    } else {
        state().theme().red.hex()
    };
    rsx! {
        span { style: "white-space: pre; color: {dollar_color}", "❯ " }
    }
}
