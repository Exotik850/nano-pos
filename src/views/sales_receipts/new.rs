use crate::Route;
use crate::components::SalesReceiptFormView;
use crate::contacts::list_contacts;
use crate::models::SalesReceiptDraft;
use crate::products::list_products;
use crate::sales_receipts::create_sales_receipt;
use dioxus::prelude::*;

#[component]
pub fn NewSalesReceipt() -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;
    rsx! {
        div { class: "mx-auto flex w-full max-w-5xl flex-col gap-8 px-6 py-8",
            div {
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                    "Add sales receipt"
                }
                p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                    "Create a sales receipt and attach products."
                }
            }
            if let Some(message) = error() {
                p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                    "{message}"
                }
            }
            SalesReceiptFormView {
                contacts,
                products,
                submit_label: if loading() { "Creating..." } else { "Create sales receipt" },
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
                    match create_sales_receipt(draft).await {
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
    }
}
