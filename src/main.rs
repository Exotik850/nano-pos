// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::*;
use auth::AuthProvider;

use crate::{auth::AuthGaurd, views::Sidebar};

mod auth;
/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

#[cfg(feature = "server")]
mod server;

mod models;
mod products;

#[component]
fn LoadingFallback() -> Element {
    rsx! {
      SuspenseBoundary {
        fallback: |_: SuspenseContext| {
          rsx! {
            div {
              class: "flex min-h-screen flex-col items-center justify-center gap-2 px-4",
              h1 {
                class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                "Loading..."
              }
            }
          }
        },
        Outlet::<Route> {}
      }
    }
}

#[component]
fn ErrorFallback() -> Element {
    rsx! {
      ErrorBoundary {
        handle_error: |e: ErrorContext| {
          let err = e.error().map(|s| s.to_string()).unwrap_or_else(|| "Unknown error".to_string());
          rsx! {
            div {
              class: "flex min-h-screen flex-col items-center justify-center gap-2 px-4",
              h1 {
                class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                "Something went wrong."
              }
              h3 {
                class: "text-lg text-zinc-500 dark:text-zinc-400",
                "details: {err}"
              }
            }
          }
        },
        Outlet::<Route> {}
      }
    }
}


/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AuthProvider)]
    #[layout(Sidebar)]
    #[layout(Navbar)]
    #[layout(ErrorFallback)]
    #[layout(LoadingFallback)]
        #[route("/")]
        Home {},
        #[route("/login")]
        Login {},
        #[route("/signup")]
        Signup {},
        #[layout(AuthGaurd)]
        #[route("/products")]
        IndexProducts {},
        #[route("/products/new")]
        NewProduct {},
        #[route("/products/:id/edit")]
        EditProduct { id: i32 },
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DX_COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");

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
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DX_COMPONENTS_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
