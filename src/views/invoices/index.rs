use crate::Route;
use crate::components::{
    IndexPage, Table, TableActions, TableBody, TableCell, TableHead, TableHeaderCell, TableRow,
    customer_name, format_date,
};
use crate::contacts::list_contacts;
use crate::invoices::{delete_invoice, list_invoices};
use crate::models::{Contact, Invoice};
use dioxus::prelude::*;

#[component]
fn InvoiceTableRow(
    invoice: Invoice,
    contact: Contact,
    ondelete: EventHandler<i32>,
) -> Element {
    let customer_name = customer_name(&[contact], Some(invoice.customer_id));
    let order_date = format_date(invoice.order_date);
    let id = invoice.id;
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
                TableActions {
                    edit_to: Route::EditInvoice { id },
                    ondelete: move |_| ondelete.call(id),
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
        IndexPage {
            title: "Invoices",
            description: "Create and manage customer invoices.",
            empty_title: "No invoices yet",
            empty_description: "Create your first invoice to start tracking orders.",
            is_empty: invoices.is_empty(),
            action: rsx! {
                crate::components::ButtonLink { to: Route::NewInvoice {}, "Add invoice" }
            },
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
                            contact: contacts_ready
                                .iter()
                                .find(|c| c.id == invoice.customer_id)
                                .cloned()
                                .unwrap_or_default(),
                            invoice,
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
