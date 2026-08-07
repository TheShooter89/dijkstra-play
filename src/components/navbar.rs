use crate::{
    components::icons::{Icon, TunedIcon, UkraineFlag},
    Route,
};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const APP_ICON: Asset = asset!("/assets/app_icon.svg");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav {
            class: "navbar px-8 pt-6",
            section {
                class: "flex-none",
                Link {
                    to: Route::Home {},
                    img {
                        class: "h-8",
                        src: "{APP_ICON}"
                    }
                }
            }
            section {
                class: "flex-none",
                Link {
                    class: "pl-1 logo-link text-2xl",
                    to: Route::Home {},
                    "ijkstra-play"
                }
            }
            section {
                class: "flex-1",
                Link {
                    class: "pl-10 btn btn-ghost text-white font-sans text-l text-shadow-sm/20",
                    to: Route::Instructions { id: 1 },
                    "Instructions"
                }
                Link {
                    class: "pl-4 btn btn-ghost text-white font-sans text-l text-shadow-sm/20",
                    to: Route::About {},
                    "About"
                }
            }
            section {
                class: "flex fill-white",
                Link {
                    class: "btn btn-xs btn-ghost text-white border-none font-sans text-sm text-shadow-sm/20 flex hover:bg-white hover:text-black hover:text-shadow-none hover:fill-black",
                    new_tab: true,
                    to: "https://github.com/TheShooter89/dijkstra-suite",
                    // GithubLogo {
                    //     // class: "h-4 flex",
                    // }
                    TunedIcon { icon: Icon::GithubLogo }
                    "Github"
                }
                Link {
                    class: "btn btn-xs btn-ghost text-white border-none font-sans text-sm text-shadow-sm/20 flex hover:bg-white hover:text-black hover:text-shadow-none hover:fill-black",
                    new_tab: true,
                    to: "https://crates.io/crates/dijkstra-suite",
                    // CratesIoLogo {
                    //     class: "h-4 flex",
                    // }
                    TunedIcon { icon: Icon::CratesIoLogo, size: "size-[1.4em]" }
                    "Crates.io"
                }
                Link {
                    class: "btn btn-xs btn-ghost text-white bg-indigo-400 border-none font-sans ml-6 text-[0.85rem] text-shadow-sm/20 flex hover:bg-white hover:text-black hover:text-shadow-none",
                    new_tab: true,
                    to: "https://u24.gov.ua/",
                    UkraineFlag {
                        class: "h-2 flex",
                    }
                    "STAND WITH UKRAINE"
                    // "SUPPORT UKRAINE"
                }
            }
        }
    }
}
