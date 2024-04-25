use dioxus::core_macro::component;
use dioxus::prelude::*;

use crate::GlobalAppState;

#[component]
pub(crate) fn Prompt() -> Element {
    let state = consume_context::<Signal<GlobalAppState>>();
    let mut command = use_signal(|| "".to_string());

    let last_failed = state()
        .buffer_mut()
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
                                crate::terminal::commands::exec(command.to_string(), state);
                                command.set("".to_string())
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
    let state = consume_context::<Signal<GlobalAppState>>();
    let dollar_color = if !command_failed {
        state().theme().green.hex()
    } else {
        state().theme().red.hex()
    };
    rsx! {
        span { style: "color: {state().theme().sapphire.hex()}", "user@aureliobuonomo.it" }
        span { style: "white-space: pre", ": " }
        span { style: "color: {state().theme().red.hex()}", {state().cwd()} }
        span { style: "white-space: pre; color: {dollar_color}", " ❯ " }
    }
}

#[component]
pub(crate) fn SimplePromptText(command_failed: bool) -> Element {
    let state = consume_context::<Signal<GlobalAppState>>();
    let dollar_color = if !command_failed {
        state().theme().green.hex()
    } else {
        state().theme().red.hex()
    };
    rsx! {
        span { style: "white-space: pre; color: {dollar_color}", "❯ " }
    }
}
