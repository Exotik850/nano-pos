use chrono::NaiveDateTime;
use dioxus::prelude::*;

use crate::Route;
use crate::components::{
    ButtonLink, ButtonVariant, DocumentForm, ProductLineDraft, datetime_input, input::FormInput,
};
use crate::models::{
    Contact, InvoiceDetails, InvoiceDraft, InvoiceForm, InvoiceProductForm, Product,
};

#[component]
pub fn InvoiceFormView(
    #[props(default)] initial: Option<InvoiceDetails>,
    contacts: Vec<Contact>,
    products: Vec<Product>,
    submit_label: String,
    onsubmit: EventHandler<Result<InvoiceDraft, String>>,
) -> Element {
    let customer_id = use_signal(|| initial.as_ref().map(|value| value.invoice.customer_id));
    let doc_number = use_signal(|| {
        initial
            .as_ref()
            .map_or_else(String::new, |value| value.invoice.doc_number.clone())
    });
    let description = use_signal(|| {
        initial
            .as_ref()
            .and_then(|value| value.invoice.description.clone())
            .unwrap_or_default()
    });
    let order_date = use_signal(|| {
        initial.as_ref().map_or_else(
            || "2026-01-01T00:00".to_string(),
            |value| datetime_input(value.invoice.order_date),
        )
    });
    let delivery_date = use_signal(|| {
        initial
            .as_ref()
            .and_then(|value| value.invoice.delivery_date.map(datetime_input))
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
        DocumentForm {
            doc_number,
            customer_id,
            contacts,
            description,
            products,
            lines,
            heading: "Invoice products",
            line_description: "Attach products and quantities to this invoice.",
            description_placeholder: "Add notes for this invoice",
            submit_label,
            customer_required: true,
            action: rsx! {
                ButtonLink {
                    to: Route::IndexProducts {},
                    variant: ButtonVariant::Secondary,
                    class: "px-3 py-1.5 text-sm",
                    "Manage products"
                }
            },
            onsubmit: move |_| {
                let parse_date = |value: &str| {
                    NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M")
                        .map_err(|_| "Enter a valid date and time.".to_string())
                };
                let Ok(order) = parse_date(&order_date()) else {
                    onsubmit.call(Err("Enter a valid date and time.".to_string()));
                    return;
                };
                let delivery = if delivery_date().is_empty() {
                    Ok(None)
                } else {
                    parse_date(&delivery_date()).map(Some)
                };
                let Ok(delivery) = delivery else {
                    onsubmit.call(Err("Enter a valid date and time.".to_string()));
                    return;
                };
                let Some(customer) = customer_id() else {
                    onsubmit.call(Err("Select a customer.".to_string()));
                    return;
                };
                onsubmit
                    .call(
                        Ok(InvoiceDraft {
                            form: InvoiceForm {
                                customer_id: customer,
                                user_id: 0,
                                doc_number: doc_number(),
                                description: (!description().is_empty()).then(&*description),
                                order_date: order,
                                delivery_date: delivery,
                            },
                            products: lines()
                                .into_iter()
                                .map(|line| InvoiceProductForm {
                                    invoice_id: 0,
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
            FormInput {
                label: "Order date",
                name: "order_date",
                r#type: "datetime-local",
                required: true,
                value: order_date,
            }
            FormInput {
                label: "Delivery date",
                name: "delivery_date",
                r#type: "datetime-local",
                value: delivery_date,
            }
        }
    }
}
