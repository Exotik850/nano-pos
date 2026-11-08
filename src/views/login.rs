use dioxus::prelude::*;

use crate::auth::{login, AuthContext};
use crate::components::{form_field, AuthField, AuthForm, AuthLink};
use crate::Route;

/// The login page. Submits the username/password form to the [`login`] server
/// function, stores the resulting session in the [`AuthContext`], and routes
/// to the home page on success.
#[component]
pub fn Login() -> Element {
    let mut auth = use_context::<AuthContext>();
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    rsx! {
        AuthForm {
            title: "Log in".to_string(),
            subtitle: "Welcome back! Enter your credentials to continue.".to_string(),
            submit_label: if loading() { "Logging in...".to_string() } else { "Log in".to_string() },
            error: error(),
            footer: rsx! {
                AuthLink { to: (Route::Signup {}).into(), label: "Create an account" }
            },
            onsubmit: move |evt| async move {
                if loading() {
                    return;
                }
                let username = form_field(&evt, "username");
                let password = form_field(&evt, "password");
                loading.set(true);
                error.set(None);
                match login(username, password).await {
                    Ok(session) => {
                        auth.set_session(session);
                        navigator.replace(Route::Home {});
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
                loading.set(false);
            },

            AuthField {
                label: "Username".to_string(),
                name: "username".to_string(),
                r#type: "text".to_string(),
                placeholder: "your_username".to_string(),
                autocomplete: "username".to_string(),
            }
            AuthField {
                label: "Password".to_string(),
                name: "password".to_string(),
                r#type: "password".to_string(),
                placeholder: "••••••••".to_string(),
                autocomplete: "current-password".to_string(),
            }
        }
    }
}
