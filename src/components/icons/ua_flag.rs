use dioxus::prelude::*;

#[component]
pub fn UkraineFlag(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            id: id,
            class: class,
            view_box: "0 0 640 480",
            g {
                fill_rule: "evenodd",
                stroke_width: "1pt",
                path {
                    fill: "#0057b8",
                    d: "M0 0h640v240H0z",
                }
                path {
                    fill: "gold",
                    d: "M0 240h640v240H0z",
                }
            }
        }
    }
}
