use dioxus::prelude::*;

use crate::auth::{signup, AuthContext};
use crate::components::{form_field, AuthField, AuthForm, AuthLink};
use crate::Route;

/// The signup page. Submits the registration form to the [`signup`] server
/// function, stores the resulting session in the [`AuthContext`], and routes
/// to the home page on success.
#[component]
pub fn Signup() -> Element {
    let mut auth = use_context::<AuthContext>();
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    rsx! {
        AuthForm {
            title: "Sign up".to_string(),
            subtitle: "Create an account to get started.".to_string(),
            submit_label: if loading() { "Creating account...".to_string() } else { "Sign up".to_string() },
            error: error(),
            footer: rsx! {
                AuthLink { to: (Route::Login {}).into(), label: "Already have an account? Log in" }
            },
            onsubmit: move |evt| async move {
                if loading() {
                    return;
                }
                let username = form_field(&evt, "username");
                let email = form_field(&evt, "email");
                let password = form_field(&evt, "password");
                loading.set(true);
                error.set(None);
                match signup(username, email, password).await {
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
                label: "Email".to_string(),
                name: "email".to_string(),
                r#type: "email".to_string(),
                placeholder: "you@example.com".to_string(),
                autocomplete: "email".to_string(),
            }
            AuthField {
                label: "Password".to_string(),
                name: "password".to_string(),
                r#type: "password".to_string(),
                placeholder: "At least 8 characters".to_string(),
                autocomplete: "new-password".to_string(),
            }
        }
    }
}
