use std::collections::HashSet;

use dioxus::prelude::*;

use crate::utils::classes::{concat_with_optional_condition, merge_classes};

#[component]
pub fn Footer(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        footer {
            id: id,
            class: merge_classes(
                "footer sm:footer-horizontal items-center p-4",
                class.as_deref(),
            ),
            {children}
        }
    }
}

#[component]
pub fn FooterAside(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        aside {
            id: id,
            class: merge_classes(
                "grid-flow-col items-center",
                class.as_deref(),
            ),
            { children }
        }
    }
}

#[component]
pub fn FooterContactsNav(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        nav {
            id: id,
            class: merge_classes(
                "grid-flow-col gap-4 md:place-self-center md:justify-self-end",
                class.as_deref(),
            ),
            { children }
        }
    }
}
