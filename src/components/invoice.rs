use chrono::NaiveDateTime;
use dioxus::prelude::*;
use dioxus_primitives::select::{SelectGroup, SelectOption};

use crate::Route;
use crate::components::select::{FormSelect, SelectGroupLabel};
use crate::components::{
    ProductLineDraft, ProductLineEditor, input::FormInput, textarea::FormTextarea,
};
use crate::models::{
    Contact, InvoiceDetails, InvoiceDraft, InvoiceForm, InvoiceProductForm, Product,
};

fn datetime_input(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%dT%H:%M").to_string()
}

#[component]
pub fn InvoiceFormView(
    #[props(default)] initial: Option<InvoiceDetails>,
    contacts: Vec<Contact>,
    products: Vec<Product>,
    submit_label: String,
    onsubmit: EventHandler<Result<InvoiceDraft, String>>,
) -> Element {
    let mut customer_id = use_signal(|| initial.as_ref().map(|value| value.invoice.customer_id));
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

    rsx! { form { class: "flex w-full max-w-5xl flex-col gap-6", onsubmit: move |event| {
        event.prevent_default();
        let parse_date = |value: &str| NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M").map_err(|_| "Enter a valid date and time.".to_string());
        let Ok(order) = parse_date(&order_date()) else { onsubmit.call(Err("Enter a valid date and time.".to_string())); return; };
        let delivery = if delivery_date().is_empty() { Ok(None) } else { parse_date(&delivery_date()).map(Some) };
        let Ok(delivery) = delivery else { onsubmit.call(Err("Enter a valid date and time.".to_string())); return; };
        let Some(customer) = customer_id() else { onsubmit.call(Err("Select a customer.".to_string())); return; };
        onsubmit.call(Ok(InvoiceDraft { form: InvoiceForm { customer_id: customer, user_id: 0, doc_number: doc_number(), description: (!description().is_empty()).then(&*description), order_date: order, delivery_date: delivery }, products: lines().into_iter().map(|line| InvoiceProductForm { invoice_id: 0, product_id: line.product_id, quantity: line.quantity, price: line.price, tax_rate: None, description: None }).collect() }));
    },
        div { class: "grid gap-4 md:grid-cols-2",
            FormInput { label: "Document number", name: "doc_number", required: true, value: doc_number }
            FormSelect::<i32> { label: "Customer", name: "customer_id", required: true, on_value_change: move |value| customer_id.set(value), SelectGroup { class: "w-full", SelectGroupLabel { "Select a customer" } for (index, contact) in contacts.into_iter().enumerate() { SelectOption::<i32> { index, value: contact.id, text_value: "{contact.first_name} {contact.last_name} ({contact.email})", "{contact.first_name} {contact.last_name} ({contact.email})" } } } }
            FormInput { label: "Order date", name: "order_date", r#type: "datetime-local", required: true, value: order_date }
            FormInput { label: "Delivery date", name: "delivery_date", r#type: "datetime-local", value: delivery_date }
        }
        FormTextarea { label: "Description", name: "description", value: description, placeholder: "Add notes for this invoice", resize: "none", class: "min-h-24 w-full" }
        ProductLineEditor { products, lines, heading: "Invoice products".to_string(), description: "Attach products and quantities to this invoice.".to_string(), action: rsx! { crate::components::ButtonLink { to: Route::IndexProducts {}, variant: crate::components::ButtonVariant::Secondary, class: "px-3 py-1.5 text-sm", "Manage products" } } }
        crate::components::Button { class: "self-center px-5 py-2.5", r#type: "submit", "{submit_label}" }
    } }
}
