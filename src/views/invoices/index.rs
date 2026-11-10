use dioxus::prelude::*;
use crate::components::{Button, ButtonLink, ButtonVariant, Table, TableBody, TableCell, TableHead, TableHeaderCell, TableRow};
use crate::contacts::list_contacts;
use crate::invoices::{delete_invoice, list_invoices};
use crate::models::{Contact, Invoice};
use crate::Route;

#[component]
fn InvoiceTableRow(invoice: Invoice, contacts: Vec<Contact>, ondelete: EventHandler<i32>) -> Element {
    let customer = contacts.iter().find(|contact| contact.id == invoice.customer_id);
    let customer_name = customer.map_or_else(|| format!("Customer #{}", invoice.customer_id), |contact| format!("{} {}", contact.first_name, contact.last_name));
    let order_date = invoice.order_date.format("%Y-%m-%d").to_string();
    rsx! {
        TableRow {
            TableCell {
                div { class: "font-medium text-zinc-900 dark:text-zinc-100", "{invoice.doc_number}" }
                div { class: "mt-1 text-sm text-zinc-500 dark:text-zinc-400", "{customer_name}" }
            }
            TableCell { class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{order_date}" }
            TableCell { class: "max-w-md truncate text-zinc-600 dark:text-zinc-300",
                "{invoice.description.clone().unwrap_or_else(|| \"No description\".to_string())}"
            }
            TableCell {
                div { class: "flex justify-end gap-2",
                    ButtonLink {
                        class: "px-3 py-1.5 text-sm",
                        to: Route::EditInvoice {
                            id: invoice.id,
                        },
                        variant: ButtonVariant::Secondary,
                        "Edit"
                    }
                    Button {
                        class: "px-3 py-1.5 text-sm",
                        variant: ButtonVariant::Secondary,
                        r#type: "button",
                        onclick: move |_| ondelete.call(invoice.id),
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
pub fn IndexInvoices() -> Element {
    let mut error = use_signal(|| None::<String>);
    let invoices = use_server_future(list_invoices)?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let invoices_ready = invoices().context("Failed to get invoices")?;
    let contacts_ready = contacts().context("Failed to get contacts")?;
    let mut invoices = use_signal(|| invoices_ready.clone());

    if let Some(message) = error() {
        return Err(ServerFnError::Response(message).into());
    }

    rsx! {
        div { class: "mx-auto flex w-full max-w-6xl flex-col gap-8 px-6 py-8",
            div { class: "flex flex-wrap items-end justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Invoices"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Create and manage customer invoices."
                    }
                }
                ButtonLink { to: Route::NewInvoice {}, "Add invoice" }
            }
            if invoices.is_empty() {
                div { class: "rounded-xl border border-dashed border-zinc-300 px-6 py-16 text-center dark:border-zinc-700",
                    h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100",
                        "No invoices yet"
                    }
                    p { class: "mt-2 text-sm text-zinc-500 dark:text-zinc-400",
                        "Create your first invoice to start tracking orders."
                    }
                }
            } else {
                Table {
                    TableHead {
                        tr {
                            TableHeaderCell { "Invoice" }
                            TableHeaderCell { "Order date" }
                            TableHeaderCell { "Description" }
                            TableHeaderCell { align_end: true, "Actions" }
                        }
                    }
                    TableBody {
                        for invoice in invoices() {
                            InvoiceTableRow {
                                invoice,
                                contacts: contacts_ready.clone(),
                                ondelete: move |invoice_id| async move {
                                    if let Err(message) = delete_invoice(invoice_id).await {
                                        error.set(Some(message.to_string()));
                                    } else {
                                        invoices.retain(|invoice| invoice.id != invoice_id);
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}