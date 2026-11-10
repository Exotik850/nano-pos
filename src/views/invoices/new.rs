use dioxus::fullstack::Form;
use dioxus::prelude::*;

use crate::components::{InvoiceDraft, InvoiceFormView};
use crate::contacts::list_contacts;
use crate::invoices::{create_invoice, InvoicePayload};
use crate::products::list_products;
use crate::Route;

#[component]
pub fn NewInvoice() -> Element {
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
                    "Add invoice"
                }
                p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                    "Create an invoice and attach products."
                }
            }
            if let Some(message) = error() {
                p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                    "{message}"
                }
            }
            InvoiceFormView {
                contacts,
                products,
                submit_label: if loading() { "Creating..." } else { "Create invoice" },
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
                    match create_invoice(
                            Form(InvoicePayload {
                                form: draft.form,
                                products: draft.products,
                            }),
                        )
                        .await
                    {
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
    }
}