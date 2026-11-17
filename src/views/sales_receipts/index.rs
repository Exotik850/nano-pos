use crate::Route;
use crate::components::{
    IndexPage, Table, TableActions, TableBody, TableCell, TableHead, TableHeaderCell, TableRow,
    customer_name, format_date,
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
    let customer = customer_name(&contacts, Some(receipt.customer_id));
    let receipt_date = format_date(receipt.sales_receipt_date);
    let id = receipt.id;
    rsx! {
        TableRow {
            TableCell {
                div { class: "font-medium text-zinc-900 dark:text-zinc-100", "{receipt.doc_number}" }
                div { class: "mt-1 text-sm text-zinc-500 dark:text-zinc-400", "{customer}" }
            }
            TableCell { class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{receipt_date}" }
            TableCell { class: "max-w-md truncate text-zinc-600 dark:text-zinc-300",
                "{receipt.description.clone().unwrap_or_else(|| \"No description\".to_string())}"
            }
            TableCell {
                TableActions {
                    edit_to: Route::EditSalesReceipt { id },
                    ondelete: move |_| ondelete.call(id),
                }
            }
        }
    }
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
        IndexPage {
            title: "Sales receipts",
            description: "Create and manage customer sales receipts.",
            empty_title: "No sales receipts yet",
            empty_description: "Create your first sales receipt to record a sale.",
            is_empty: receipts.is_empty(),
            action: rsx! {
                crate::components::ButtonLink { to: Route::NewSalesReceipt {}, "Add sales receipt" }
            },
            Table {
                TableHead {
                    tr {
                        TableHeaderCell { "Sales receipt" }
                        TableHeaderCell { "Date" }
                        TableHeaderCell { "Description" }
                        TableHeaderCell { align_end: true, "Actions" }
                    }
                }
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
