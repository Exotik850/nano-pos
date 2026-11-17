//! Shared building blocks for the document forms (invoice, estimate, credit,
//! sales receipt). These forms all share the same shape: a document number, a
//! customer select, a description, and a product line editor.

use dioxus::prelude::*;
use dioxus_primitives::select::SelectGroup;

use crate::components::select::{FormSelect, SelectGroupLabel, SelectOption};
use crate::components::{
    Button, ProductLineDraft, ProductLineEditor, input::FormInput, textarea::FormTextarea,
};
use crate::models::{Contact, Product};

/// A customer picker populated from the loaded contacts.
#[component]
pub fn CustomerSelect(
    contacts: Vec<Contact>,
    #[props(default, into)] value: ReadSignal<Option<i32>>,
    #[props(default)] required: bool,
    on_value_change: Callback<Option<i32>>,
) -> Element {
    rsx! {
        FormSelect::<i32> {
            label: "Customer",
            name: "customer_id",
            required,
            value,
            on_value_change,
            SelectGroup { class: "w-full",
                SelectGroupLabel { "Select a customer" }
                for (index, contact) in contacts.into_iter().enumerate() {
                    SelectOption::<i32> {
                        index,
                        value: contact.id,
                        text_value: "{contact.first_name} {contact.last_name} ({contact.email})",
                        "{contact.first_name} {contact.last_name} ({contact.email})"
                    }
                }
            }
        }
    }
}

/// The shared layout for a document form.
///
/// Renders the document number, customer, description and product lines, with
/// `children` slotted in for any resource-specific fields (such as dates).
#[component]
pub fn DocumentForm(
    doc_number: Signal<String>,
    mut customer_id: Signal<Option<i32>>,
    contacts: Vec<Contact>,
    description: Signal<String>,
    products: Vec<Product>,
    lines: Signal<Vec<ProductLineDraft>>,
    #[props(into)] heading: String,
    #[props(into)] line_description: String,
    #[props(into)] description_placeholder: String,
    #[props(into)] submit_label: String,
    #[props(default)] customer_required: bool,
    #[props(default)] action: Option<Element>,
    onsubmit: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        form {
            class: "flex w-full max-w-5xl flex-col gap-6",
            onsubmit: move |event| {
                event.prevent_default();
                onsubmit.call(());
            },
            div { class: "grid gap-4 md:grid-cols-2",
                FormInput {
                    label: "Document number",
                    name: "doc_number",
                    required: true,
                    value: doc_number,
                }
                CustomerSelect {
                    contacts,
                    value: customer_id,
                    required: customer_required,
                    on_value_change: move |value| customer_id.set(value),
                }
                {children}
            }
            FormTextarea {
                label: "Description",
                name: "description",
                value: description,
                placeholder: description_placeholder,
                resize: "none",
                class: "min-h-24 w-full",
            }
            ProductLineEditor {
                products,
                lines,
                heading,
                description: line_description,
                action,
            }
            Button { class: "self-center px-5 py-2.5", r#type: "submit", "{submit_label}" }
        }
    }
}
