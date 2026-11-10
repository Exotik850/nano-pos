use crate::Route;
use crate::auth::AuthContext;
use crate::components::{ButtonLink, ButtonVariant};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
///
/// For logged-out visitors the home page is a prompt to sign up or log in. For
/// logged-in users it is a blank page with a short welcome message.
#[component]
pub fn Home() -> Element {
    let auth = use_context::<AuthContext>();

    rsx! {
        if let Some(session) = auth.session() {
            div { class: "flex min-h-[70vh] flex-col items-center justify-center gap-2 px-4",
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                    "Welcome back, {session.username}!"
                }
                p { class: "text-zinc-500 dark:text-zinc-400", "You are signed in." }
            }
        } else {
            div { class: "flex min-h-[70vh] flex-col items-center justify-center gap-6 px-4 text-center",
                div { class: "flex flex-col gap-2",
                    h1 { class: "text-4xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Welcome to nano-pos"
                    }
                    p { class: "text-zinc-500 dark:text-zinc-400",
                        "Create an account to get started, or log in to your existing one."
                    }
                }
                div { class: "flex items-center gap-4",
                    ButtonLink { to: Route::Signup {}, "Sign up" }
                    ButtonLink {
                        variant: ButtonVariant::Secondary,
                        to: Route::Login {},
                        "Log in"
                    }
                }
            }
        }
    }
}
