//! Shared authentication module: the server functions for signup / login /
//! logout / current user, plus the reactive [`AuthContext`] used by the UI.

use dioxus::{fullstack::Form, prelude::*};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::server::auth::SessionCookieJar;
#[cfg(feature = "server")]
use crate::server::db::DbHandle;
use crate::views::{LoginForm, SignupForm};

/// The currently logged-in user, returned by the auth endpoints.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
    pub username: String,
}

/// Register a new account and start a session for the new user.
#[server(db: DbHandle, jar: SessionCookieJar)]
pub async fn signup(form: Form<SignupForm>) -> Result<Session, ServerFnError> {
    let SignupForm {
        username,
        email,
        password,
    } = form.0;
    crate::server::auth::signup_impl(&username, &email, &password, db, jar)
}

/// Log in with a username and password, starting a session on success.
#[server(db: DbHandle, jar: SessionCookieJar)]
pub async fn login(form: Form<LoginForm>) -> Result<Session, ServerFnError> {
    let LoginForm { username, password } = form.0;
    crate::server::auth::login_impl(&username, &password, db, jar)
}

/// Load the session belonging to the caller's session cookie, if any.
///
/// `jar` is a server-only extractor: the macro declares it in the
/// `#[server(...)]` attribute, so the generated client stub takes no argument
/// while the server handler extracts the request headers.
#[server(db: DbHandle, jar: SessionCookieJar)]
pub async fn current_user() -> Result<Option<Session>, ServerFnError> {
    Ok(jar.session(&db).ok())
}

/// Clear the caller's session cookie.
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    crate::server::auth::logout_impl();
    Ok(())
}
