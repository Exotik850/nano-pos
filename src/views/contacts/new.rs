use dioxus::fullstack::Form;
use dioxus::prelude::*;

use crate::Route;
use crate::components::{ContactFormView, PageError, PageHeader, PageShell};
use crate::contacts::create_contact;
use crate::models::ContactForm;

#[component]
pub fn NewContact() -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    rsx! {
        PageShell { max_width: "max-w-3xl",
            PageHeader {
                title: "Add contact",
                description: "Create a customer contact.",
            }
            if let Some(message) = error() {
                PageError { message }
            }
            ContactFormView {
                submit_label: if loading() { "Creating..." } else { "Create contact" },
                onsubmit: move |draft: ContactForm| async move {
                    if loading() {
                        return;
                    }
                    loading.set(true);
                    error.set(None);
                    match create_contact(Form(draft)).await {
                        Ok(_) => {
                            navigator.replace(Route::IndexContacts {});
                        }
                        Err(message) => {
                            error.set(Some(message.to_string()));
                            loading.set(false);
                        }
                    }
                },
            }
        }
    }
}
