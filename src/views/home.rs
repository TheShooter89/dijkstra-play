use crate::components::{home::HomeHero, App};
use dioxus::prelude::*;

// if CSS is needed, create a css file on below path and unconmment
// both this line and the socument::Link line inside the body of
// About component
// const HOME_CSS: Asset = asset!("/assets/styling/about.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        // if CSS is needed, uncomment this line and the line outside
        // this component
        // document::Link { rel: "stylesheet", href: HOME_CSS }

        main {
            id: "home-page",
            class: "grow",

            HomeHero { id: "home-hero", class: "mt-8 mb-6 bg-transparent" }
            div {
                class: "w-full flex justify-center",
                App { id: "app", class: "w-[88vw] min-h-[40vh]", }
            }
        }
    }
}
