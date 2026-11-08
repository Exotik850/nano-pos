use dioxus::fullstack::Form;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Route;
use crate::auth::{AuthContext, login};
use crate::components::{AuthField, AuthForm, AuthLink};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

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
            title: "Log in",
            subtitle: "Welcome back! Enter your credentials to continue.",
            submit_label: if loading() { "Logging in..." } else { "Log in" },
            error: error(),
            footer: rsx! {
                AuthLink { to: Route::Signup {}, label: "Create an account" }
            },
            onsubmit: move |evt: FormEvent| async move {
                if loading() {
                    return;
                }
                let form = evt.parsed_values().expect("Failed to parse form values");
                loading.set(true);
                error.set(None);
                match login(Form(form)).await {
                    Ok(session) => {
                        auth.set_session(session);
                        navigator.replace(Route::Home {});
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
                loading.set(false);
            },

            AuthField {
                label: "Username",
                name: "username",
                r#type: "text",
                placeholder: "your_username",
                autocomplete: "username",
            }
            AuthField {
                label: "Password",
                name: "password",
                r#type: "password",
                placeholder: "••••••••",
                autocomplete: "current-password",
            }
        }
    }
}
