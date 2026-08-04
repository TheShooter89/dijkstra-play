use crate::{
    components::{App, Echo, Hero, HeroContent},
    Route,
};
use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

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

            Hero {
                //
                class: "bg-base-200 min-h-[25vh]",
                HeroContent {
                    div {
                        class: "max-w-[65vw]",
                        h1 {
                            //
                            class: "text-6xl font-bold",
                            "find "
                            span {
                                class: "text-secondary",
                                "your"
                            }
                            " path"
                        }
                        p {
                            class: "py-6",
                            "try the power of "
                            Link {
                                //
                                // class: "btn btn-primary",
                                to: Route::Home {  },
                                "dijkstra-suite"
                            }
                            " crate to find the shortest path in the map using Dijkstra algorithm."
                        }
                        p {
                            class: "py-2",
                            "just press "
                            Link {
                                //
                                // class: "btn btn-primary",
                                to: Route::Home {  },
                                "play"
                            }
                        }
                        p {
                            class: "py-6",
                            Link {
                                //
                                // class: "btn btn-primary",
                                to: Route::Home {  },
                                "upload your map"
                            }
                            " as csv or choose one from the "
                            Link {
                                //
                                // class: "btn btn-primary",
                                to: Route::Home {  },
                                "available examples"
                            }
                            "."
                        }
                        p {
                            class: "py-6",
                            "if you're still not satisfied, you can "
                            Link {
                                //
                                // class: "btn btn-primary",
                                to: Route::Home {  },
                                "edit them"
                            }
                            " or even create a new map "
                            Link {
                                //
                                // class: "btn btn-primary",
                                to: Route::Home {  },
                                "entirely from scratch"
                            }
                        }
                        Link {
                            //
                            class: "btn btn-primary",
                            to: Route::Home {  },
                            "dijkstra-suite crate"
                        }
                        Link {
                            //
                            class: "btn btn-primary ml-3",
                            to: Route::Home {  },
                            "TRY"
                        }
                        Link {
                            //
                            class: "btn btn-primary ml-3",
                            to: Route::Home {  },
                            "Getting Started"
                        }
                    }
                }
            }
            App {  }
        }
    }
}
