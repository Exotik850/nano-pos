use crate::Route;
use crate::components::{
    Button, ButtonLink, ButtonVariant, EmptyState, PageHeader, PageShell, Table, TableBody,
    TableCell, TableHead, TableHeaderCell, TableRow,
};
use crate::contacts::list_contacts;
use crate::models::{Contact, SalesReceipt};
use crate::sales_receipts::{delete_sales_receipt, list_sales_receipts};
use dioxus::prelude::*;

#[component]
fn SalesReceiptTableRow(
    receipt: SalesReceipt,
    contacts: Vec<Contact>,
    ondelete: EventHandler<i32>,
) -> Element {
    let customer = contacts
        .iter()
        .find(|contact| contact.id == receipt.customer_id)
        .map_or_else(
            || format!("Customer #{}", receipt.customer_id),
            |contact| format!("{} {}", contact.first_name, contact.last_name),
        );
    rsx! { TableRow {
        TableCell { div { class: "font-medium text-zinc-900 dark:text-zinc-100", "{receipt.doc_number}" } div { class: "mt-1 text-sm text-zinc-500 dark:text-zinc-400", "{customer}" } }
        TableCell { class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{receipt.sales_receipt_date.format(\"%Y-%m-%d\")}" }
        TableCell { class: "max-w-md truncate text-zinc-600 dark:text-zinc-300", "{receipt.description.clone().unwrap_or_else(|| \"No description\".to_string())}" }
        TableCell { div { class: "flex justify-end gap-2", ButtonLink { class: "px-3 py-1.5 text-sm", to: Route::EditSalesReceipt { id: receipt.id }, variant: ButtonVariant::Secondary, "Edit" } Button { class: "px-3 py-1.5 text-sm", variant: ButtonVariant::Secondary, r#type: "button", onclick: move |_| ondelete.call(receipt.id), "Delete" } } }
    } }
}

#[component]
pub fn IndexSalesReceipts() -> Element {
    let mut error = use_signal(|| None::<String>);
    let receipts = use_server_future(list_sales_receipts)?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let receipts_ready = receipts().context("Failed to get sales receipts")?;
    let contacts_ready = contacts().context("Failed to get contacts")?;
    let mut receipts = use_signal(|| receipts_ready.clone());
    if let Some(message) = error() {
        return Err(ServerFnError::Response(message).into());
    }
    rsx! {
        PageShell { max_width: "max-w-6xl".to_string(),
            PageHeader {
                title: "Sales receipts".to_string(),
                description: "Create and manage customer sales receipts.".to_string(),
                action: rsx! { ButtonLink { to: Route::NewSalesReceipt {}, "Add sales receipt" } },
            }
            if receipts.is_empty() {
                EmptyState {
                    title: "No sales receipts yet".to_string(),
                    description: "Create your first sales receipt to record a sale.".to_string(),
                }
            } else {
                Table {
                    TableHead { tr {
                        TableHeaderCell { "Sales receipt" }
                        TableHeaderCell { "Date" }
                        TableHeaderCell { "Description" }
                        TableHeaderCell { align_end: true, "Actions" }
                    } }
                    TableBody {
                        for receipt in receipts() {
                            SalesReceiptTableRow {
                                receipt,
                                contacts: contacts_ready.clone(),
                                ondelete: move |receipt_id| async move {
                                    if let Err(message) = delete_sales_receipt(receipt_id).await {
                                        error.set(Some(message.to_string()));
                                    } else {
                                        receipts.retain(|value| value.id != receipt_id);
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
