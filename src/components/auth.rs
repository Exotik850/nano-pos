use crate::Route;
use crate::api::auth::{Session, current_user, logout};
use crate::components::{ButtonLink, ButtonVariant};
use dioxus::prelude::*;
#[derive(Clone, Copy)]
pub struct AuthContext {
    session: Signal<Option<Session>>,
}

impl AuthContext {
    /// The active session, if the user is logged in.
    pub fn session(&self) -> Option<Session> {
        (self.session)()
    }

    /// Whether a user is currently logged in.
    #[allow(dead_code)]
    pub fn is_logged_in(&self) -> bool {
        (self.session)().is_some()
    }

    /// Install a fresh session after a successful signup/login.
    pub fn set_session(&mut self, session: Session) {
        self.session.set(Some(session));
    }

    /// Clear the session after a logout.
    pub async fn clear_session(&mut self) {
        self.session.set(None);
        let _ = logout().await;
    }
}

/// Provides an [`AuthContext`] to the whole component tree and validates any
/// existing session cookie on first load. Wrap the router with this component.
#[component]
pub fn AuthProvider() -> Element {
    // Return the session from the server future so the initial client render
    // uses the same value as SSR during hydration.
    let initial_session = use_server_future(current_user)?;
    // let initial_session = use_server_future(current_user)?.suspend()?;
    let session = use_signal(|| initial_session().transpose().ok().flatten().flatten());

    use_context_provider(|| AuthContext { session });

    rsx! {
        Outlet::<Route> {}

    }
}

#[component]
pub fn AuthGaurd() -> Element {
    let auth = use_context::<AuthContext>();

    if auth.session().is_none() {
        return rsx! {
            div { class: "flex min-h-[70vh] flex-col items-center justify-center gap-4 px-6 text-center",
                h1 { class: "text-2xl font-bold text-zinc-900 dark:text-zinc-100",
                    "Sign in required"
                }
                ButtonLink { to: Route::Login {}, variant: ButtonVariant::Primary, "Sign in" }
            }
        };
    }

    rsx! {
        Outlet::<Route> {}

    }
}
