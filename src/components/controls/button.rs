use dioxus::prelude::*;

use crate::{utils::classes::merge_classes, Route};

#[derive(Props, Clone, PartialEq)]
pub struct ControlsButtonProps {
    #[props(default)]
    id: Option<String>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    active_class: Option<String>,
    #[props(into)]
    to: NavigationTarget<Route>,
    #[props(default)]
    new_tab: Option<bool>,
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(default)]
    onclick_only: Option<bool>,
    #[props(default)]
    rel: Option<String>,
    #[props(default)]
    children: Option<Element>,
}

#[component]
pub fn ControlsButton(props: ControlsButtonProps) -> Element {
    rsx! {
        Link {
            id: props.id,
            class: merge_classes(
                "btn btn-xs btn-accent",
                props.class.as_deref(),
            ),
            active_class: props.active_class,
            to: props.to,
            new_tab: props.new_tab.unwrap_or(false),
            onclick: props.onclick,
            onmounted: props.onmounted,
            onclick_only: props.onclick_only.unwrap_or(false),
            rel: props.rel,
            {props.children}
        }
    }
}
