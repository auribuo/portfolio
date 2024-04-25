use dioxus::{
    core_macro::{component, rsx},
    dioxus_core::Element,
    prelude::*,
};

use crate::ui::components::prompt::SimplePromptText;
mod prompt;
pub(crate) use prompt::Prompt;

#[component]
pub(crate) fn CmdOutput(cmd: String, cmd_output: String, failed: bool) -> Element {
    rsx! {
        div {
            SimplePromptText { command_failed: failed }
            "{cmd}"
        }
        div { class: "cmd-output", dangerous_inner_html: cmd_output }
    }
}
