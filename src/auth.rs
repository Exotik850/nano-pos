//! Shared authentication module: the server functions for signup / login /
//! logout / current user, plus the reactive [`AuthContext`] used by the UI.
//!
//! This module is compiled on *both* the client and server targets. The
//! `#[server]` macro generates an HTTP client stub on the client and an axum
//! handler on the server; the heavy lifting (Diesel, Argon2, cookies) lives in
//! the server-only `crate::server::auth` module, gated behind
//! `#[cfg(feature = "server")]`.

use dioxus::{fullstack::Form, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    Route, components::{ButtonLink, ButtonVariant}, views::{LoginForm, SignupForm},
};

/// The currently logged-in user, returned by the auth endpoints.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
    pub username: String,
}

/// Register a new account and start a session for the new user.
#[server]
pub async fn signup(form: Form<SignupForm>) -> Result<Session, ServerFnError> {
    let SignupForm {
        username,
        email,
        password,
    } = form.0;
    crate::server::auth::signup_impl(&username, &email, &password)
}

/// Log in with a username and password, starting a session on success.
#[server]
pub async fn login(form: Form<LoginForm>) -> Result<Session, ServerFnError> {
    let LoginForm { username, password } = form.0;
    crate::server::auth::login_impl(&username, &password)
}

/// Load the session belonging to the caller's session cookie, if any.
///
/// `cookies` is a server-only extractor: the macro declares it in the
/// `#[server(...)]` attribute, so the generated client stub takes no argument
/// while the server handler extracts the request headers via
/// `FullstackContext::extract`.
#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn current_user() -> Result<Option<Session>, ServerFnError> {
    crate::server::auth::current_user_impl(&cookies)
}

/// Clear the caller's session cookie.
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    crate::server::auth::logout_impl();
    Ok(())
}

/// Reactive login state shared through the Dioxus context system.
///
/// The [`AuthProvider`] component keeps this up to date: it is `None` while
/// anonymous, and `Some(session)` once a login/signup succeeds (or an existing
/// session cookie is validated on first load).
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
    let initial_session = use_server_future(|| async {
        match current_user().await {
            Ok(session) => session,
            Err(_) => None,
        }
    })?.suspend()?;
    let session = use_signal(|| initial_session());

    use_context_provider(|| AuthContext { session });

    rsx! { Outlet::<Route> {} }
}

#[component]
pub fn AuthGaurd() -> Element {
	let auth = use_context::<AuthContext>();

	if auth.session().is_none() {
		return rsx! {
			div { class: "flex min-h-[70vh] flex-col items-center justify-center gap-4 px-6 text-center",
				h1 { class: "text-2xl font-bold text-zinc-900 dark:text-zinc-100", "Sign in required" }
				ButtonLink { to: Route::Login {}, variant: ButtonVariant::Primary, "Sign in" }
			}
		};
	}

	rsx! { Outlet::<Route> {} }
}
