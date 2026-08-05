use std::collections::HashSet;

use dioxus::prelude::*;

use crate::utils::classes::{concat_with_optional_condition, merge_classes};

#[component]
pub fn Controls(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        nav {
            id: id,
            class: "flex justify-center",
            div {
                class: "flex-none",
                //
                Link {
                    class: "btn btn-secondary",
                    to: "#",
                    "Edit"
                }
                Link {
                    class: "btn btn-secondary",
                    to: "#",
                    "View"
                }
            }
            div {
                //
                class: "grow flex justify-center",
                Link {
                    class: "btn btn-secondary",
                    to: "#",
                    "Play"
                }
            }
            div {
                //
                class: "flex-none",
                Link {
                    class: "btn btn-secondary",
                    to: "#",
                    "Upload"
                }
                Link {
                    class: "btn btn-secondary",
                    to: "#",
                    "Create"
                }
                Link {
                    class: "btn btn-secondary",
                    to: "#",
                    "Examples"
                }
            }
        }
    }
}
