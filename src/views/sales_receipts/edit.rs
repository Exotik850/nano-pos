use crate::Route;
use crate::components::SalesReceiptFormView;
use crate::contacts::list_contacts;
use crate::models::SalesReceiptDraft;
use crate::products::list_products;
use crate::sales_receipts::{get_sales_receipt, update_sales_receipt};
use dioxus::prelude::*;

#[component]
pub fn EditSalesReceipt(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let receipt = use_server_future(move || get_sales_receipt(id))?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    let receipt = receipt().context("Failed to get sales receipt")?;
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;
    match receipt {
        Some(receipt) => rsx! {
            div { class: "mx-auto flex w-full max-w-5xl flex-col gap-8 px-6 py-8",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Edit sales receipt"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Update sales receipt {receipt.sales_receipt.doc_number}."
                    }
                }
                if let Some(message) = error() {
                    p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                        "{message}"
                    }
                }
                SalesReceiptFormView {
                    initial: receipt,
                    contacts,
                    products,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
                    onsubmit: move |draft: Result<SalesReceiptDraft, String>| async move {
                        if loading() {
                            return;
                        }
                        let Ok(draft) = draft else {
                            error.set(Some(draft.unwrap_err()));
                            return;
                        };
                        loading.set(true);
                        error.set(None);
                        match update_sales_receipt(id, draft).await {
                            Ok(_) => {
                                navigator.replace(Route::IndexSalesReceipts {});
                            }
                            Err(message) => {
                                error.set(Some(message.to_string()));
                                loading.set(false);
                            }
                        }
                    },
                }
            }
        },
        None => rsx! {
            div { class: "mx-auto flex w-full max-w-3xl flex-col gap-4 px-6 py-8",
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                    "Sales receipt not found"
                }
                Link {
                    to: Route::IndexSalesReceipts {},
                    class: "text-zinc-700 underline dark:text-zinc-300",
                    "Back to sales receipts"
                }
            }
        },
    }
}
