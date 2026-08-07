use std::collections::HashSet;

use dioxus::prelude::*;

use crate::{
    components::{
        controls::ControlsButton,
        icons::{Icon, TunedIcon},
    },
    utils::classes::{concat_with_optional_condition, merge_classes},
    Route,
};

#[component]
pub fn Controls(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    let py = "pb-0.75 pt-0.5";
    let px = "px-1.25";

    rsx! {
        nav {
            id: id,
            class: "flex justify-center font-sans mb-2",
            div {
                class: "flex-none {py} {px} bg-yellow-200/64 rounded-xl",
                ControlsButton {
                    to: "#",
                    class: "mr-1",
                    TunedIcon {
                        icon: Icon::ViewIcon,
                    }
                    "View"
                }
                ControlsButton {
                    to: "#",
                    class: "btn-ghost text-black",
                    TunedIcon {
                        icon: Icon::EditIcon,
                    }
                    "Edit"
                }
            }
            div {
                //
                class: "grow flex justify-center {py} {px}",
                ControlsButton {
                    to: "#",
                    TunedIcon {
                        icon: Icon::CirclePlay,
                    }
                    "Play"
                }
            }
            div {
                //
                class: "flex-none join {py} {px}",
                ControlsButton {
                    to: "#",
                    class: "join-item",
                    TunedIcon {
                        icon: Icon::Upload,
                    }
                    "Upload"
                }
                ControlsButton {
                    to: "#",
                    class: "join-item",
                    TunedIcon {
                        icon: Icon::CirclePlus,
                    }
                    "Create"
                }
                ControlsButton {
                    to: "#",
                    class: "join-item",
                    TunedIcon {
                        icon: Icon::WandMagicSparkles,
                    }
                    "Examples"
                }
            }
        }
    }
}
