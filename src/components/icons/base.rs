use dioxus::prelude::*;

use crate::{
    components::icons::{icons_path_d::SvgPathD, Icon},
    utils::classes::merge_classes,
};

#[derive(Props, Clone, PartialEq)]
pub struct TunedIconProps {
    #[props(default)]
    id: Option<String>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    size: Option<String>,
    #[props(default)]
    width: Option<String>,
    #[props(default)]
    height: Option<String>,
    #[props(default)]
    view_box: Option<String>,
    #[props(default)]
    d: Option<String>,
    #[props(default)]
    icon: Option<Icon>,
    #[props(default)]
    fill: Option<String>,
}

#[component]
pub fn TunedIcon(props: TunedIconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            id: props.id,
            class: merge_classes(
                format!(
                    //
                    "{} {} shrink-0",
                    props.size.unwrap_or("size-[1.3em]".to_string()),
                    props.fill.unwrap_or("fill-current".to_string()),
                ).as_str(),
                props.class.as_deref()
            ),
            width: props.width,
            height: props.height,
            view_box: props.view_box.unwrap_or("0 0 640 640".into()),
            path {
                d: props.icon.unwrap_or(Icon::Unknown).d(),
            }
        }
    }
}
