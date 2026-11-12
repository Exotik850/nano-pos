use crate::Route;
use crate::auth::AuthContext;
use crate::components::{Button, ButtonLink, ButtonVariant, SearchBar};
use crate::views::sidebar::Hidden;
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronsRight;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the routes in a common navbar. The contents of each route will be rendered
/// under the outlet inside this component. The navbar reflects the current login state: anonymous visitors see
/// Log in / Sign up links, while a logged-in user sees their username and a Log out button.
#[component]
pub fn Navbar() -> Element {
    let mut auth = use_context::<AuthContext>();
    let mut sidebar_hidden = use_context::<Hidden>();
    let navigator = use_navigator();
    let session = auth.session();
    let signed_in = session.is_some();

    let invis_sign_in = if signed_in {
        ""
    } else {
        "invisible !w-0 !p-0 !opacity-0"
    };
    let hidden = if sidebar_hidden.0() {
        ""
    } else {
        "invisible !w-0 !p-0 !opacity-0"
    };

    rsx! {
        div {
            class: "flex flex-wrap items-center gap-4 border-b border-zinc-200 px-6 py-3 dark:border-zinc-700
            transition-all duration-300 ease-in-out",
            id: "navbar",

            Button {
                class: "px-2 py-1 text-xs opacity-100 {invis_sign_in} {hidden}",
                variant: ButtonVariant::Secondary,
                r#type: "button",
                onclick: move |_| sidebar_hidden.0.set(false),
                ChevronsRight { size: 16 }
            }

            Link {
                class: "font-semibold text-zinc-900 hover:underline dark:text-zinc-100 {hidden}",
                to: Route::Home {},
                "Home"
            }

            SearchBar { class: "{invis_sign_in}" }

            // Right-aligned auth controls.
            div { class: "ml-auto flex items-center gap-4",
                if let Some(session) = session.clone() {
                    span { class: "text-sm text-zinc-600 dark:text-zinc-400",
                        "Signed in as "
                        strong { class: "text-zinc-900 dark:text-zinc-100", "{session.username}" }
                    }
                    Button {
                        class: "mr-4",
                        variant: ButtonVariant::Secondary,
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
                    ButtonLink {
                        class: "px-3 py-1.5 text-sm font-medium",
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
