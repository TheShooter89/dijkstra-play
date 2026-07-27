use crate::components::Navbar;
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

        div {
            id: "footer",
            // class: "mt-10",
            h1 { "FOOTER!!!" }
        }
    }
}
