use chrono::NaiveDateTime;
use dioxus::prelude::*;
use dioxus_primitives::select::SelectGroup;

use crate::components::select::{FormSelect, SelectGroupLabel, SelectOption};
use crate::components::{input::FormInput, textarea::FormTextarea};
use crate::models::{
    Contact, Product, SalesReceiptDetails, SalesReceiptDraft, SalesReceiptForm,
    SalesReceiptProductForm,
};

use super::{
    Button, ButtonVariant, Table, TableBody, TableCell, TableHead, TableHeaderCell, TableInput,
    TableRow, TableSelect,
};

fn datetime_input(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%dT%H:%M").to_string()
}

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
    let mut lines: Signal<Vec<SalesReceiptProductForm>> = use_signal(|| {
        initial
            .as_ref()
            .map(|value| value.products.clone())
            .map(|items| items.into_iter().map(Into::into).collect())
            .unwrap_or_default()
    });
    let mut selected_product = use_signal(String::new);
    let mut quantity = use_signal(|| "1".to_string());
    let mut price = use_signal(|| "0".to_string());

    rsx! {
        form {
            class: "flex w-full max-w-5xl flex-col gap-6",
            onsubmit: move |event| {
                event.prevent_default();
                let date = match NaiveDateTime::parse_from_str(&receipt_date(), "%Y-%m-%dT%H:%M") {
                    Ok(value) => value,
                    Err(_) => { onsubmit.call(Err("Enter a valid sales receipt date and time.".to_string())); return; }
                };
                let Some(customer) = customer_id() else { onsubmit.call(Err("Select a customer.".to_string())); return; };
                if doc_number().trim().is_empty() { onsubmit.call(Err("Document number is required.".to_string())); return; }
                onsubmit.call(Ok(SalesReceiptDraft {
                    sales_receipt: SalesReceiptForm { customer_id: customer, user_id: 0, doc_number: doc_number(), description: (!description().is_empty()).then(&*description), sales_receipt_date: date },
                    products: lines(),
                }));
            },
            div { class: "grid gap-4 md:grid-cols-2",
                FormInput { label: "Document number", name: "doc_number", required: true, value: doc_number }
                div { class: "flex w-full flex-col gap-1", FormSelect::<i32> { label: "Customer", name: "customer_id", on_value_change: move |value| customer_id.set(value), SelectGroup { class: "w-full", SelectGroupLabel { "Select a customer" } for (index, contact) in contacts.into_iter().enumerate() { SelectOption::<i32> { index, value: contact.id, text_value: "{contact.first_name} {contact.last_name} ({contact.email})", "{contact.first_name} {contact.last_name} ({contact.email})" } } } } }
                FormInput { label: "Sales receipt date", name: "sales_receipt_date", r#type: "datetime-local", required: true, value: receipt_date }
            }
            FormTextarea { label: "Description", name: "description", value: description, placeholder: "Add notes for this sales receipt", resize: "none", class: "min-h-24 w-full" }
            div { class: "flex flex-col gap-4 rounded-xl border border-zinc-200 p-4 dark:border-zinc-700",
                div { h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100", "Sales receipt products" } p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Attach products and quantities to this sales receipt." } }
                Table {
                    TableHead { tr { TableHeaderCell { "Product" } TableHeaderCell { align_end: true, "Quantity" } TableHeaderCell { align_end: true, "Price" } TableHeaderCell { align_end: true, "Actions" } } }
                    TableBody {
                        TableRow {
                            TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40", TableSelect { value: selected_product, option { value: "", "Select" } for product in products.iter() { option { value: "{product.id}", "{product.name} ({product.sku})" } } } }
                            TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40", TableInput { name: "line_quantity", r#type: "number", placeholder: "1", value: quantity } }
                            TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40", TableInput { name: "line_price", r#type: "number", value: price } }
                            TableCell { align_end: true, class: "bg-zinc-50 dark:bg-zinc-800/40", Button { r#type: "button", onclick: move |_| { let Ok(product_id) = selected_product().parse::<i32>() else { return; }; let Ok(parsed_quantity) = quantity().parse::<i32>() else { return; }; let parsed_price = (!price().is_empty()).then(|| price().parse::<i32>().ok()).flatten(); lines.push(SalesReceiptProductForm { sales_receipt_id: 0, product_id, quantity: parsed_quantity, price: parsed_price, tax_rate: None, description: None }); selected_product.set(String::new()); quantity.set("1".to_string()); price.set("0".to_string()); }, "Add line" } }
                        }
                        for line in lines() {
                            { let product_name = products.iter().find(|product| product.id == line.product_id).map_or_else(|| format!("Product #{}", line.product_id), |product| product.name.clone()); let line_price = line.price.map_or_else(|| "-".to_string(), |value| format!("${:.2}", f64::from(value) / 100.0)); rsx! { TableRow { TableCell { "{product_name}" } TableCell { align_end: true, "{line.quantity}" } TableCell { align_end: true, "{line_price}" } TableCell { align_end: true, Button { class: "px-3 py-1.5 text-sm", variant: ButtonVariant::Secondary, r#type: "button", onclick: move |_| lines.retain(|value| value.product_id != line.product_id), "Remove" } } } } }
                        }
                    }
                }
            }
            Button { class: "self-center px-5 py-2.5", r#type: "submit", "{submit_label}" }
        }
    }
}
