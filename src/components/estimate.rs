use dioxus::prelude::*;
use dioxus_primitives::select::SelectGroup;

use crate::Route;
use crate::components::select::{FormSelect, SelectGroupLabel, SelectOption};
use crate::components::{
    ProductLineDraft, ProductLineEditor, input::FormInput, textarea::FormTextarea,
};
use crate::models::{
    Contact, EstimateDetails, EstimateDraft, EstimateForm, EstimateProductForm, Product,
};

#[component]
pub fn EstimateFormView(
    #[props(default)] initial: Option<EstimateDetails>,
    contacts: Vec<Contact>,
    products: Vec<Product>,
    submit_label: String,
    onsubmit: EventHandler<Result<EstimateDraft, String>>,
) -> Element {
    let mut customer_id = use_signal(|| {
        initial
            .as_ref()
            .and_then(|value| value.estimate.customer_id)
    });
    let doc_number = use_signal(|| {
        initial
            .as_ref()
            .map_or_else(String::new, |value| value.estimate.doc_number.clone())
    });
    let description = use_signal(|| {
        initial
            .as_ref()
            .and_then(|value| value.estimate.description.clone())
            .unwrap_or_default()
    });
    let lines = use_signal(|| {
        initial
            .as_ref()
            .map(|value| {
                value
                    .products
                    .iter()
                    .map(|line| ProductLineDraft {
                        product_id: line.product_id,
                        quantity: line.quantity,
                        price: line.price,
                    })
                    .collect()
            })
            .unwrap_or_default()
    });

    rsx! {
        form {
            class: "flex w-full max-w-5xl flex-col gap-6",
            onsubmit: move |event| {
                event.prevent_default();
                if doc_number().trim().is_empty() {
                    onsubmit.call(Err("Document number is required.".to_string()));
                    return;
                }
                onsubmit
                    .call(
                        Ok(EstimateDraft {
                            estimate: EstimateForm {
                                customer_id: customer_id(),
                                user_id: 0,
                                doc_number: doc_number(),
                                description: (!description().is_empty()).then(&*description),
                            },
                            products: lines()
                                .into_iter()
                                .map(|line| EstimateProductForm {
                                    estimate_id: 0,
                                    product_id: line.product_id,
                                    quantity: line.quantity,
                                    price: line.price,
                                    tax_rate: None,
                                    description: None,
                                })
                                .collect(),
                        }),
                    );
            },
            div { class: "grid gap-4 md:grid-cols-2",
                FormInput {
                    label: "Document number",
                    name: "doc_number",
                    required: true,
                    value: doc_number,
                }
                div { class: "flex w-full flex-col gap-1",
                    FormSelect::<i32> {
                        label: "Customer",
                        name: "customer_id",
                        on_value_change: move |value| customer_id.set(value),
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
            FormTextarea {
                label: "Description",
                name: "description",
                value: description,
                placeholder: "Add notes for this estimate",
                resize: "none",
                class: "min-h-24 w-full",
            }
            ProductLineEditor {
                products,
                lines,
                heading: "Estimate products",
                description: "Attach products and quantities to this estimate.",
                action: rsx! {
                    crate::components::ButtonLink {
                        to: Route::IndexProducts {},
                        variant: crate::components::ButtonVariant::Secondary,
                        class: "px-3 py-1.5 text-sm",
                        "Manage products"
                    }
                },
            }
            crate::components::Button { class: "self-center px-5 py-2.5", r#type: "submit", "{submit_label}" }
        }
    }
}
