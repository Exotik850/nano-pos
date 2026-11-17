use chrono::NaiveDateTime;
use dioxus::prelude::*;
use dioxus_primitives::select::SelectGroup;

use crate::components::select::{FormSelect, SelectGroupLabel, SelectOption};
use crate::components::{
    ProductLineDraft, ProductLineEditor, datetime_input, input::FormInput, textarea::FormTextarea,
};
use crate::models::{
    Contact, Product, SalesReceiptDetails, SalesReceiptDraft, SalesReceiptForm,
    SalesReceiptProductForm,
};

#[component]
pub fn SalesReceiptFormView(
    #[props(default)] initial: Option<SalesReceiptDetails>,
    contacts: Vec<Contact>,
    products: Vec<Product>,
    submit_label: String,
    onsubmit: EventHandler<Result<SalesReceiptDraft, String>>,
) -> Element {
    let mut customer_id = use_signal(|| {
        initial
            .as_ref()
            .map(|value| value.sales_receipt.customer_id)
    });
    let doc_number = use_signal(|| {
        initial
            .as_ref()
            .map_or_else(String::new, |value| value.sales_receipt.doc_number.clone())
    });
    let description = use_signal(|| {
        initial
            .as_ref()
            .and_then(|value| value.sales_receipt.description.clone())
            .unwrap_or_default()
    });
    let receipt_date = use_signal(|| {
        initial.as_ref().map_or_else(
            || "2026-01-01T00:00".to_string(),
            |value| datetime_input(value.sales_receipt.sales_receipt_date),
        )
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
                let Ok(date) = NaiveDateTime::parse_from_str(&receipt_date(), "%Y-%m-%dT%H:%M")
                else {
                    onsubmit.call(Err("Enter a valid sales receipt date and time.".to_string()));
                    return;
                };
                let Some(customer) = customer_id() else {
                    onsubmit.call(Err("Select a customer.".to_string()));
                    return;
                };
                if doc_number().trim().is_empty() {
                    onsubmit.call(Err("Document number is required.".to_string()));
                    return;
                }
                onsubmit
                    .call(
                        Ok(SalesReceiptDraft {
                            sales_receipt: SalesReceiptForm {
                                customer_id: customer,
                                user_id: 0,
                                doc_number: doc_number(),
                                description: (!description().is_empty()).then(&*description),
                                sales_receipt_date: date,
                            },
                            products: lines()
                                .into_iter()
                                .map(|line| SalesReceiptProductForm {
                                    sales_receipt_id: 0,
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
                FormInput {
                    label: "Sales receipt date",
                    name: "sales_receipt_date",
                    r#type: "datetime-local",
                    required: true,
                    value: receipt_date,
                }
            }
            FormTextarea {
                label: "Description",
                name: "description",
                value: description,
                placeholder: "Add notes for this sales receipt",
                resize: "none",
                class: "min-h-24 w-full",
            }
            ProductLineEditor {
                products,
                lines,
                heading: "Sales receipt products".to_string(),
                description: "Attach products and quantities to this sales receipt.".to_string(),
            }
            crate::components::Button { class: "self-center px-5 py-2.5", r#type: "submit", "{submit_label}" }
        }
    }
}
