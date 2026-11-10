use dioxus::prelude::*;

use crate::Route;
use crate::components::{InvoiceDraft, InvoiceFormView};
use crate::contacts::list_contacts;
use crate::invoices::{get_invoice, update_invoice};
use crate::products::list_products;

#[component]
pub fn EditInvoice(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut invoice_opt = use_signal(|| None);
    let invoice = use_server_future(move || get_invoice(id))?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    invoice_opt.set(invoice().context("Failed to get invoice")?);
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;

    match invoice_opt() {
        Some(invoice) => rsx! {
            div { class: "mx-auto flex w-full max-w-5xl flex-col gap-8 px-6 py-8",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Edit invoice"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Update invoice {invoice.invoice.doc_number}."
                    }
                }
                if let Some(message) = error() {
                    p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                        "{message}"
                    }
                }
                InvoiceFormView {
                    initial: invoice,
                    contacts,
                    products,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
                    onsubmit: move |draft: Result<InvoiceDraft, String>| async move {
                        if loading() {
                            return;
                        }
                        let Ok(draft) = draft else {
                            error.set(Some(draft.unwrap_err()));
                            return;
                        };
                        loading.set(true);
                        error.set(None);
                        match update_invoice(id, draft).await {
                            Ok(_) => {
                                navigator.replace(Route::IndexInvoices {});
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
                    "Invoice not found"
                }
                Link {
                    to: Route::IndexInvoices {},
                    class: "text-zinc-700 underline dark:text-zinc-300",
                    "Back to invoices"
                }
            }
        },
    }
}
