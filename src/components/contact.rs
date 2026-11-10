use dioxus::prelude::*;

use crate::{
    components::{input::FormInput, textarea::FormTextarea},
    models::{Contact, ContactForm},
};

use super::Button;

#[component]
pub fn ContactFormView(
    #[props(default)] initial: Option<Contact>,
    submit_label: String,
    onsubmit: EventHandler<ContactForm>,
) -> Element {
    let first_name = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.first_name.clone())
            .unwrap_or_default()
    });
    let last_name = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.last_name.clone())
            .unwrap_or_default()
    });
    let phone_number = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.phone_number.clone())
            .unwrap_or_default()
    });
    let email = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.email.clone())
            .unwrap_or_default()
    });
    let message = use_signal(|| {
        initial
            .as_ref()
            .and_then(|contact| contact.message.clone())
            .unwrap_or_default()
    });
    let street = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.street.clone())
            .unwrap_or_default()
    });
    let city = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.city.clone())
            .unwrap_or_default()
    });
    let state = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.state.clone())
            .unwrap_or_default()
    });
    let zip_code = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.zip_code.clone())
            .unwrap_or_default()
    });
    let country = use_signal(|| {
        initial
            .as_ref()
            .map(|contact| contact.country.clone())
            .unwrap_or_default()
    });

    rsx! {
        form {
            class: "flex w-full max-w-4xl flex-col gap-6",
            onsubmit: move |event| {
                event.prevent_default();
                onsubmit
                    .call(ContactForm {
                        first_name: first_name(),
                        last_name: last_name(),
                        phone_number: phone_number(),
                        email: email(),
                        message: (!message().is_empty()).then(&*message),
                        street: street(),
                        city: city(),
                        state: state(),
                        zip_code: zip_code(),
                        country: country(),
                    });
            },
            div { class: "flex flex-row gap-4",
                FormInput {
                    label: "First name",
                    name: "first_name",
                    r#type: "text",
                    required: true,
                    value: first_name,
                }
                FormInput {
                    label: "Last name",
                    name: "last_name",
                    r#type: "text",
                    required: true,
                    value: last_name,
                }
            }
            div { class: "flex flex-row gap-4",
                FormInput {
                    label: "Phone number",
                    name: "phone_number",
                    r#type: "tel",
                    required: true,
                    value: phone_number,
                }
                FormInput {
                    label: "Email",
                    name: "email",
                    r#type: "email",
                    required: true,
                    value: email,
                }
            }
            div { class: "flex flex-row gap-4",
                FormInput {
                    label: "Street",
                    name: "street",
                    r#type: "text",
                    value: street,
                }
                FormInput {
                    label: "City",
                    name: "city",
                    r#type: "text",
                    value: city,
                }
            }
            div { class: "flex flex-row gap-4",
                FormInput {
                    label: "State",
                    name: "state",
                    r#type: "text",
                    value: state,
                }
                FormInput {
                    label: "ZIP code",
                    name: "zip_code",
                    r#type: "text",
                    value: zip_code,
                }
                FormInput {
                    label: "Country",
                    name: "country",
                    r#type: "text",
                    value: country,
                }
            }
            FormTextarea {
                label: "Message",
                name: "message",
                value: message,
                placeholder: "Enter a message for the contact",
                resize: "none",
                class: "min-h-32 w-full",
            }
            Button { class: "self-center w-1/4 px-5 py-2.5", r#type: "submit", "{submit_label}" }
        }
    }
}
