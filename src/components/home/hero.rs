use crate::{
    components::{
        icons::{CirclePlay, CratesIoLogo, FilePage, Icon, TunedIcon},
        Hero, HeroContent,
    },
    utils::classes::merge_classes,
    Route,
};
use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

// if CSS is needed, create a css file on below path and unconmment
// both this line and the socument::Link line inside the body of
// About component
// const HOME_CSS: Asset = asset!("/assets/styling/about.css");

#[component]
pub fn HomeHero(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    let accented_link =
        "link link-accent link-accent link-hover hover:text-accent text-yellow-200 font-bold";

    rsx! {
        // if CSS is needed, uncomment this line and the line outside
        // this component
        // document::Link { rel: "stylesheet", href: HOME_CSS }

        Hero {
            //
            id: id,
            class: merge_classes("bg-base-200 min-h-[25vh] font-sans", class.as_deref()),
            HeroContent {
                div {
                    class: "text-md max-w-[65vw]",
                    h1 {
                        //
                        class: "text-6xl font-bold font-serif",
                        "find "
                        span {
                            class: "text-yellow-300",
                            "your"
                        }
                        " path"
                    }
                    p {
                        class: "text-xl pt-6 text-shadow-lg/20",
                        "try the power of "
                        Link {
                            class: "{accented_link}",
                            to: Route::Home {  },
                            "dijkstra-suite"
                        }
                        " crate to find the shortest path in the map using Dijkstra algorithm"
                    }
                    p {
                        class: "text-xl py-10 text-shadow-lg/20",
                        "just press "
                        Link {
                            class: "{accented_link}",
                            to: Route::Home {  },
                            "play"
                        }
                    }
                    p {
                        class: "text-md pb-0 text-shadow-lg/20",
                        Link {
                            class: "{accented_link}",
                            to: Route::Home {  },
                            "upload your map"
                        }
                        " as csv or choose one from the "
                        Link {
                            class: "{accented_link}",
                            to: Route::Home {  },
                            "available examples"
                        }
                        "."
                    }
                    p {
                        class: "text-md pt-0 mb-12 text-shadow-lg/20",
                        "if you're still not satisfied, you can "
                        Link {
                            class: "{accented_link}",
                            to: Route::Home {  },
                            "edit them"
                        }
                        " or even create a new map "
                        Link {
                            class: "{accented_link}",
                            to: Route::Home {  },
                            "entirely from scratch"
                        }
                    }
                    Link {
                        //
                        class: "btn btn-neutral bg-white text-black",
                        new_tab: true,
                        to: "https://crates.io/crates/dijkstra-suite",
                        TunedIcon {
                            icon: Icon::CratesIoLogo,
                            size: "size-[1.8em]",
                        }
                        "DIJKSTRA-SUITE"
                    }
                    Link {
                        //
                        class: "btn btn-neutral bg-white text-black ml-3",
                        to: Route::Home {  },
                        "TRY"
                        // CirclePlay {
                        //     class: "flex h-6",
                        // }
                        TunedIcon {
                            icon: Icon::CirclePlay,
                            size: "size-[1.6em]",
                        }
                    }
                    Link {
                        //
                        class: "btn btn-neutral bg-white text-black ml-3",
                        to: Route::Instructions { id: 1 },
                        TunedIcon {
                            icon: Icon::FilePage,
                            size: "size-[1.8em]",
                        }
                        "Getting Started"
                    }
                }
            }
        }
        {children}
    }
}
