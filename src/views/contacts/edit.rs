use dioxus::prelude::*;

use crate::Route;
use crate::components::{ContactFormView, EditPage, submit_form};
use crate::contacts::{get_contact, update_contact};
use crate::models::ContactForm;

#[component]
pub fn EditContact(id: i32) -> Element {
    let navigator = use_navigator();
    let error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let contact = use_server_future(move || get_contact(id))?.suspend()?;

    let result = contact();
    let load_error = result.as_ref().err().map(ToString::to_string);
    let initial = result.ok().flatten();
    let found = initial.is_some();
    let description = initial.as_ref().map_or_else(
        || "Update this contact.".to_string(),
        |contact| {
            format!(
                "Update the details for {} {}.",
                contact.first_name, contact.last_name
            )
        },
    );

    rsx! {
        EditPage {
            title: "Edit contact",
            description,
            resource: "Contact",
            back_to: Route::IndexContacts {},
            back_label: "Back to contacts",
            error: error(),
            load_error,
            max_width: "max-w-3xl",
            found,
            if let Some(contact) = initial {
                ContactFormView {
                    initial: contact,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
                    onsubmit: move |draft: ContactForm| async move {
                        submit_form(
                                loading,
                                error,
                                navigator,
                                Route::IndexContacts {},
                                || async move {
                                    update_contact(id, draft)
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
}
