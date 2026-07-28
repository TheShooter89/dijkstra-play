use crate::{
    components::{Hero, HeroContent},
    Route,
};
use dioxus::prelude::*;

const INSTRUCTIONS_CSS: Asset = asset!("/assets/styling/instructions.css");

#[component]
pub fn Instructions(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: INSTRUCTIONS_CSS }

        main {
            id: "instructions-page",
            class: "grow",

            Hero {
                //
                class: "bg-base-200 min-h-[25vh]",
                HeroContent {
                    div {
                        class: "max-w-md",
                        h1 {
                            //
                            class: "text-xl font-bold",
                            "MOSCA BRUCIA, BASTARDI"
                        }
                        p {
                            class: "py-6",
                            "mettete semi di girasole nelle tasche dei vostri pantaloni, così quando creperete almeno sarete concime per qualcosa di bello"
                        }
                        Link {
                            //
                            class: "btn btn-primary",
                            to: Route::Home {  },
                            "BOMB A REFINERY"
                        }
                    }
                }
            }
        }
    }
}
