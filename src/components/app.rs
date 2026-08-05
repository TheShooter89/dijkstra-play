use std::collections::HashSet;

use dioxus::prelude::*;

use crate::{
    components::controls::Controls,
    utils::classes::{concat_with_optional_condition, merge_classes},
};

#[component]
pub fn App(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        section {
            // class: "hero",
            id: id,
            class: merge_classes(
                "p-4 flex flex-col",
                class.as_deref(),
            ),
            Controls { id: "app_controls" }
            div {
                //
                class: "grow grid grid-cols-12 gap-4",
                section {
                    //
                    class: "col-span-9 bg-green-500",
                    p {
                        class: "font-bold",
                        "Map Container"
                    }
                }
                section {
                    //
                    class: "col-span-3 bg-red-500",
                    p {
                        class: "font-bold",
                        "Sidepanel Container"
                    }
                }
            }
            {children}
        }
    }
}
