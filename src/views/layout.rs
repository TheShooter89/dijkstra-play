use crate::components::{
    icons::{Icon, TunedIcon},
    Footer, FooterAside, FooterContactsNav, Navbar,
};
use crate::Route;
use dioxus::prelude::*;

const LAYOUT_CSS: Asset = asset!("/assets/styling/layout.css");

/// The Layout component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Layout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LAYOUT_CSS }

        Navbar {}

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}

        Footer {
            id: "footer",
            FooterAside {
                p {
                    class: "text-blue-900 text-shadow-xs/20",
                    "made with 💛️💙️ and ☕️ by "
                    Link {
                        to: "https://github.com/TheShooter89",
                        new_tab: true,
                        span {
                            class: "font-bold",
                            "tanque"
                        }
                    }
                    " - Copyright © 2026 - All right reserved"
                }
            }
            FooterContactsNav {
                class: "items-center",
                p {
                    class: "text-blue-900 text-shadow-xs/20",
                    "Wanna hire me? Find me on:"
                }
                Link {
                    to: "https://github.com/TheShooter89",
                    new_tab: true,
                    TunedIcon {
                        icon: Icon::GithubLogo,
                        size: "size-[2em]",
                        fill: "fill-blue-900",
                    }
                }
                Link {
                    to: "https://www.instagram.com/theshooter89/",
                    new_tab: true,
                    TunedIcon {
                        icon: Icon::InstagramLogo,
                        size: "size-[2.2em]",
                        fill: "fill-blue-900",
                    }
                }
                Link {
                    to: "https://www.linkedin.com/in/francesco-paoletti-79b50849/",
                    new_tab: true,
                    TunedIcon {
                        icon: Icon::LinkedinLogo,
                        size: "size-[2.2em]",
                        fill: "fill-blue-900",
                    }
                }
            }
        }
    }
}
