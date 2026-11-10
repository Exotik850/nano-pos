use dioxus::fullstack::Form;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Route;
use crate::auth::{AuthContext, signup};
use crate::components::{AuthForm, AuthLink, FormInput};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

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
            title: "Sign up",
            subtitle: "Create an account to get started.",
            submit_label: if loading() { "Creating account..." } else { "Sign up" },
            error: error(),
            footer: rsx! {
                AuthLink { to: Route::Login {}, label: "Already have an account? Log in" }
            },
            onsubmit: move |evt: FormEvent| async move {
                if loading() {
                    return;
                }
                let form = evt.parsed_values().expect("Failed to parse form values");
                loading.set(true);
                error.set(None);
                match signup(Form(form)).await {
                    Ok(session) => {
                        auth.set_session(session);
                        navigator.replace(Route::Home {});
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
                loading.set(false);
            },

            FormInput {
                label: "Username",
                name: "username",
                r#type: "text",
                required: true,
                placeholder: "your_username",
                autocomplete: "username",
            }
            FormInput {
                label: "Email",
                name: "email",
                r#type: "email",
                required: true,
                placeholder: "you@example.com",
                autocomplete: "email",
            }
            FormInput {
                label: "Password",
                name: "password",
                r#type: "password",
                required: true,
                placeholder: "At least 8 characters",
                autocomplete: "new-password",
            }
        }
    }
}
