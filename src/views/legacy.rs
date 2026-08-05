use crate::components::{Echo, Hero, HeroContent};
use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

// if CSS is needed, create a css file on below path and unconmment
// both this line and the socument::Link line inside the body of
// About component
// const HOME_CSS: Asset = asset!("/assets/styling/about.css");

#[component]
pub fn Legacy() -> Element {
    rsx! {
        // if CSS is needed, uncomment this line and the line outside
        // this component
        // document::Link { rel: "stylesheet", href: HOME_CSS }

        main {
            id: "home-page",
            class: "grow",

        Hero {
            class: "bg-base-200",
            HeroContent {
                div {
                    // Attributes should be defined in the element before any children
                    id: "hero",
                    // After all attributes are defined, we can define child elements and components
                    img { src: HEADER_SVG, id: "header" }
                    div { id: "links",
                        // The RSX macro also supports text nodes surrounded by quotes
                        a { href: "https://dioxuslabs.com/learn/0.7/", "⚠️ LEGACY HOMPAGE" }
                        a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                        a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                        a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                        a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                        a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                        a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
                    }
                }
            }
        }
        Echo {}
        }
    }
}
