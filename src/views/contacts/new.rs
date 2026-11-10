use dioxus::fullstack::Form;
use dioxus::prelude::*;

use crate::Route;
use crate::components::ContactFormView;
use crate::contacts::create_contact;
use crate::models::ContactForm;

#[component]
pub fn NewContact() -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    rsx! {
        div { class: "mx-auto flex w-full max-w-3xl flex-col gap-8 px-6 py-8",
            div {
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                    "Add contact"
                }
                p { class: "mt-1 text-zinc-500 dark:text-zinc-400", "Create a customer contact." }
            }
            if let Some(message) = error() {
                p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                    "{message}"
                }
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
