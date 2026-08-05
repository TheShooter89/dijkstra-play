use std::collections::HashSet;

use dioxus::prelude::*;

use crate::utils::classes::{concat_with_optional_condition, merge_classes};

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
                "p-4",
                class.as_deref(),
            ),
            nav {
                //
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
            div {
                //
                class: "grid grid-cols-12 gap-4",
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
