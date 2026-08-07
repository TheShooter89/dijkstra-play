use dioxus::prelude::*;

use crate::utils::classes::merge_classes;

#[derive(Props, Clone, PartialEq)]
pub struct MapProps {
    #[props(default)]
    id: Option<String>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    bg: Option<String>,
    #[props(default)]
    padding: Option<String>,
    #[props(default)]
    col_span: Option<String>,
    #[props(default)]
    rounded: Option<String>,
    #[props(default)]
    children: Option<Element>,
}

#[component]
pub fn Map(props: MapProps) -> Element {
    let card = "card card-xl shadow-md";
    let shadow = "shadow-lg";

    rsx! {
        section {
            id: props.id,
            // class: "col-span-9 bg-yellow-200/60 rounded-xl",
            class: merge_classes(
                format!(
                    //
                    "{card} {} {} {} {} {shadow}",
                    props.col_span.unwrap_or("col-span-9".to_string()),
                    props.padding.unwrap_or("p-0".to_string()),
                    props.bg.unwrap_or("bg-yellow-200/60".to_string()),
                    props.rounded.unwrap_or("rounded-xl".to_string()),
                ).as_str(),
                props.class.as_deref()
            ),
            p {
                class: "font-bold",
                "Map Container"
            }
        }
    }
}
