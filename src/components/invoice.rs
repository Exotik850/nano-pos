use chrono::NaiveDateTime;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{input::FormInput, textarea::FormTextarea};
use crate::models::{Contact, InvoiceDetails, InvoiceForm, InvoiceProductForm, Product};

use super::{
    Button, ButtonLink, ButtonVariant, Table, TableBody, TableCell, TableHead, TableHeaderCell,
    TableInput, TableRow, TableSelect,
};
use crate::Route;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InvoiceDraft {
    pub form: InvoiceForm,
    pub products: Vec<InvoiceProductForm>,
}

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
    let mut customer_id = use_signal(|| {
        initial
            .as_ref()
            .map(|value| value.invoice.customer_id.to_string())
            .unwrap_or_default()
    });
    let doc_number = use_signal(|| {
        initial
            .as_ref()
            .map(|value| value.invoice.doc_number.clone())
            .unwrap_or_default()
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
    let mut lines: Signal<Vec<InvoiceProductForm>> = use_signal(|| {
        initial
            .as_ref()
            .map(|value| value.products.clone())
            .map(|prods| prods.into_iter().map(|l: crate::models::InvoiceProduct| l.into()).collect())
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
                let parse_date = |value: &str| {
                    NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M")
                        .map_err(|_| "Enter a valid date and time.".to_string())
                };
                let order = match parse_date(&order_date()) {
                    Ok(value) => value,
                    Err(error) => {
                        onsubmit.call(Err(error));
                        return;
                    }
                };
                let delivery = if delivery_date().is_empty() {
                    Ok(None)
                } else {
                    parse_date(&delivery_date()).map(Some)
                };
                let delivery = match delivery {
                    Ok(value) => value,
                    Err(error) => {
                        onsubmit.call(Err(error));
                        return;
                    }
                };
                let customer = match customer_id().parse::<i32>() {
                    Ok(value) if value > 0 => value,
                    _ => {
                        onsubmit.call(Err("Select a customer.".to_string()));
                        return;
                    }
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
                            products: lines(),
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
                    label {
                        class: "text-sm font-medium text-zinc-700 dark:text-zinc-300",
                        r#for: "customer_id",
                        "Customer"
                    }
                    select {
                        class: "w-full rounded-lg border border-zinc-300 bg-white px-3 py-2 text-zinc-900 outline-none focus:border-zinc-500 dark:border-zinc-600 dark:bg-zinc-800 dark:text-zinc-100",
                        id: "customer_id",
                        value: "{customer_id}",
                        onchange: move |event| customer_id.set(event.value()),
                        option { value: "", "Select a customer" }
                        for contact in contacts {
                            option { value: "{contact.id}",
                                "{contact.first_name} {contact.last_name} ({contact.email})"
                            }
                        }
                    }
                }
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
            FormTextarea {
                label: "Description",
                name: "description",
                value: description,
                placeholder: "Add notes for this invoice",
                resize: "none",
                class: "min-h-24 w-full",
            }
            div { class: "flex flex-col gap-4 rounded-xl border border-zinc-200 p-4 dark:border-zinc-700",
                div { class: "flex flex-wrap items-end justify-between gap-3",
                    div {
                        h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100",
                            "Invoice products"
                        }
                        p { class: "text-sm text-zinc-500 dark:text-zinc-400",
                            "Attach products and quantities to this invoice."
                        }
                    }
                    ButtonLink {
                        to: Route::IndexProducts {},
                        variant: ButtonVariant::Secondary,
                        class: "px-3 py-1.5 text-sm",
                        "Manage products"
                    }
                }
                Table {
                    TableHead {
                        tr {
                            TableHeaderCell { "Product" }
                            TableHeaderCell { align_end: true, "Quantity" }
                            TableHeaderCell { align_end: true, "Price" }
                            TableHeaderCell { align_end: true, "Actions" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40",
                                TableSelect { value: selected_product,
                                    option { value: "", "Select" }
                                    for product in products.iter() {
                                        option { value: "{product.id}", "{product.name} ({product.sku})" }
                                    }
                                }
                            }
                            TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40",
                                TableInput {
                                    name: "line_quantity",
                                    r#type: "number",
                                    placeholder: "1",
                                    value: quantity,
                                }
                            }
                            TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40",
                                TableInput {
                                    name: "line_price",
                                    r#type: "number",
                                    value: price,
                                }
                            }
                            TableCell { align_end: true, class: "bg-zinc-50 dark:bg-zinc-800/40",
                                Button {
                                    class: "whitespace-nowrap",
                                    r#type: "button",
                                    onclick: move |_| {
                                        let Ok(product_id) = selected_product().parse::<i32>() else {
                                            return;
                                        };
                                        let Ok(parsed_quantity) = quantity().parse::<i32>() else {
                                            return;
                                        };
                                        let parsed_price = (!price().is_empty())
                                            .then(|| price().parse::<i32>().ok())
                                            .flatten();
                                        lines.push(InvoiceProductForm {
                                            invoice_id: 0,
                                            product_id,
                                            quantity: parsed_quantity,
                                            price: parsed_price,
                                            tax_rate: None,
                                            description: None,
                                        });
                                        selected_product.set(String::new());
                                        quantity.set("1".to_string());
                                        price.set("0".to_string());
                                    },
                                    "Add line"
                                }
                            }
                        }
                        for line in lines() {
                            {
                                let product = products.iter().find(|product| product.id == line.product_id);
                                let product_name = product
                                    .map_or_else(
                                        || format!("Product #{}", line.product_id),
                                        |value| value.name.clone(),
                                    );
                                let line_price = line
                                    .price
                                    .map_or_else(
                                        || "-".to_string(),
                                        |value| format!("${:.2}", f64::from(value) / 100.0),
                                    );
                                rsx! {
                                    TableRow {
                                        TableCell { class: "text-zinc-700 dark:text-zinc-300", "{product_name}" }
                                        TableCell { align_end: true, "{line.quantity}" }
                                        TableCell { align_end: true, "{line_price}" }
                                        TableCell { align_end: true,
                                            Button {
                                                class: "px-3 py-1.5 text-sm",
                                                variant: ButtonVariant::Secondary,
                                                r#type: "button",
                                                onclick: move |_| lines.retain(|value| value.product_id != line.product_id),
                                                "Remove"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Button { class: "self-center px-5 py-2.5", r#type: "submit", "{submit_label}" }
        }
    }
}
