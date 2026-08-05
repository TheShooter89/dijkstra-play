// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{About, Blog, Home, Instructions, Layout, Legacy};

mod utils;

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Layout)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/instructions/:id")]
        Instructions { id: i32 },
        #[route("/about")]
        About {},
        #[route("/blog/:id")]
        Blog { id: i32 },
        #[route("/legacy")]
        Legacy {},
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const TT_FONT: Asset = asset!("/assets/fonts/tt_interphases_pro_regular.ttf");
const HARPER_FONT: Asset = asset!("/assets/fonts/harper.otf");
const BLADE_FONT: Asset = asset!("/assets/fonts/blade.ttf");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // +++++++++++++ INJECT HTML ELEMENT ATTRIBUTES +++++++++++++
    // we need to set classes on html and body elements, but rsx!
    // macro only affects elements children of body, so we need to
    // use document::eval() that makes a JS call once both elements
    // has been mounted on DOM
    //
    // also, we set Tailwind class 'contents' on the wrapping div
    // that dioxus automatically injects:
    // display: contents makes the div itself disappear from the
    // layout box model entirely (it stops participating in
    // flex/grid sizing, stacking, etc.) while its children still
    // render normally
    use_effect(|| {
        //
        document::eval(
            r#"
            document.documentElement.classList.add("min-h-[100vh]");
            document.body.className = "flex flex-col min-h-[100vh]";
            document.getElementById("main").classList.add("contents");
            "#,
        );
    });

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Style { {format!(
            "@font-face {{
                font-family: 'TT Interphase';
                src: url('{TT_FONT}') format('truetype');
            }}
            @font-face {{
                font-family: 'Harper';
                src: url('{HARPER_FONT}') format('opentype');
            }}
            @font-face {{
                font-family: 'Blade';
                src: url('{BLADE_FONT}') format('truetype');
            }}"
        )} }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
