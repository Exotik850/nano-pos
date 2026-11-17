use dioxus::fullstack::Form;
use dioxus::prelude::*;

use crate::Route;
use crate::components::{ContactFormView, FormPage, submit_form};
use crate::contacts::create_contact;
use crate::models::ContactForm;

#[component]
pub fn NewContact() -> Element {
    let navigator = use_navigator();
    let error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);

    rsx! {
        FormPage {
            title: "Add contact",
            description: "Create a customer contact.",
            max_width: "max-w-3xl",
            error: error(),
            ContactFormView {
                submit_label: if loading() { "Creating..." } else { "Create contact" },
                onsubmit: move |draft: ContactForm| async move {
                    submit_form(
                            loading,
                            error,
                            navigator,
                            Route::IndexContacts {},
                            || async move {
                                create_contact(Form(draft))
                                    .await
                                    .map(|_| ())
                                    .map_err(|message| message.to_string())
                            },
                        )
                        .await;
                },
            }
        }
    }
}
