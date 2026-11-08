use crate::Route;
use crate::auth::{AuthContext, logout};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the routes in a common navbar. The contents of each route will be rendered
/// under the outlet inside this component. The navbar reflects the current login state: anonymous visitors see
/// Log in / Sign up links, while a logged-in user sees their username and a Log out button.
#[component]
pub fn Navbar() -> Element {
    let mut auth = use_context::<AuthContext>();
    let navigator = use_navigator();
    let session = auth.session();

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            class: "flex flex-wrap items-center gap-4 border-b border-zinc-200 px-6 py-3 dark:border-zinc-700",
            id: "navbar",
            Link {
                class: "font-semibold text-zinc-900 hover:underline dark:text-zinc-100",
                to: Route::Home {},
                "Home"
            }

            // Right-aligned auth controls.
            div {
                class: "ml-auto flex items-center gap-4",
                if let Some(session) = session.clone() {
                    span {
                        class: "text-sm text-zinc-600 dark:text-zinc-400",
                        "Signed in as "
                        strong { class: "text-zinc-900 dark:text-zinc-100", "{session.username}" }
                    }
                    button {
                        class: "rounded-lg border border-zinc-300 px-3 py-1.5 text-sm font-medium text-zinc-800 transition 
                        hover:bg-zinc-100 dark:border-zinc-600 dark:text-zinc-200 dark:hover:bg-zinc-800 cursor-pointer mr-4",
                        r#type: "button",
                        onclick: move |_| async move {
                            auth.clear_session().await;
                            navigator.replace(Route::Home {});
                        },
                        "Log out"
                    }
                } else {
                    Link {
                        class: "text-sm text-zinc-700 transition hover:underline dark:text-zinc-300",
                        to: Route::Login {},
                        "Log in"
                    }
                    Link {
                        class: "rounded-lg bg-zinc-900 px-3 py-1.5 text-sm font-medium dark:text-zinc-900 transition 
                        hover:bg-zinc-700 dark:bg-zinc-100 dark:text-zinc-900 dark:hover:bg-zinc-300",
                        to: Route::Signup {},
                        "Sign up"
                    }
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render
        // the component for the active route.
        Outlet::<Route> {}
    }
}
