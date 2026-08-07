use std::collections::HashSet;

use dioxus::prelude::*;

use crate::{
    components::{
        controls::ControlsButton,
        editor::{Map, Panel},
        icons::{Icon, TunedIcon},
    },
    utils::classes::{concat_with_optional_condition, merge_classes},
    Route,
};

#[component]
pub fn Editor(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        div {
            id: id,
            class: "grow grid grid-cols-12 gap-4 rounded-xl",
            Map { padding: "p-3" }
            Panel { padding: "p-3" }
        }
    }
}
