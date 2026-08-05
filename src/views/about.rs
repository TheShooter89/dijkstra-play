use crate::{
    components::{Hero, HeroContent},
    Route,
};
use dioxus::prelude::*;

// if CSS is needed, create a css file on below path and unconmment
// both this line and the socument::Link line inside the body of
// About component
// const ABOUT_CSS: Asset = asset!("/assets/styling/about.css");

#[component]
pub fn About() -> Element {
    rsx! {
        // if CSS is needed, uncomment this line and the line outside
        // this component
        // document::Link { rel: "stylesheet", href: ABOUT_CSS }

        main {
            id: "about-page",
            class: "grow",

            Hero {
                //
                class: "bg-base-200 min-h-[25vh]",
                HeroContent {
                    div {
                        class: "max-w-lg",
                        h1 {
                            //
                            class: "text-xl font-bold",
                            "I NEED AMMUNITION, NOT A RIDE"
                        }
                        p {
                            class: "py-6",
                            "Una mattina mi son svegliato / Oh Bella Ciao, Bella Ciao, Bella Ciao Ciao / Una mattina mi son svegliato / E ho trovato l'Invasor"
                        }
                        Link {
                            //
                            class: "btn btn-primary",
                            to: Route::Legacy {  },
                            "BOMB THE KERCH BRIDGE"
                        }
                    }
                }
            }
        }
    }
}
