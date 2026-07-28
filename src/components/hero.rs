use std::collections::HashSet;

use dioxus::prelude::*;

use crate::utils::classes::{concat_with_optional_condition, merge_classes};

#[component]
pub fn Hero(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        div {
            // class: "hero",
            class: merge_classes(
                "hero",
                class.as_deref(),
            ),
            {children}
        }
    }
}

#[component]
pub fn HeroContent(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] centered: Option<bool>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        div {
            id: id,
            class: merge_classes(
                concat_with_optional_condition(
                    "hero-content",
                    centered,
                    "text-center",
                ).as_str(),
                class.as_deref(),
            ),
            { children }
        }
    }
}

#[component]
pub fn HeroOverlay(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        div {
            id: id,
            class: merge_classes(
                "hero-overlay",
                class.as_deref(),
            ),
            { children }
        }
    }
}

#[component]
pub fn HeroWithOverlay(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] overlay: Option<bool>,
    #[props(default)] overlay_class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        div {
            id: id,
            class: merge_classes(
                "hero",
                class.as_deref(),
            ),
            if overlay.unwrap_or(false) {
                HeroOverlay {
                    class: overlay_class,
                }
            }
            {children}
        }
    }
}
