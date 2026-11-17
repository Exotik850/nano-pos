use dioxus::prelude::*;

use crate::Route;
use crate::components::{FormPage, InvoiceFormView, submit_form};
use crate::contacts::list_contacts;
use crate::invoices::create_invoice;
use crate::models::InvoiceDraft;
use crate::products::list_products;

#[component]
pub fn NewInvoice() -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;

    rsx! {
        FormPage {
            title: "Add invoice",
            description: "Create an invoice and attach products.",
            max_width: "max-w-5xl",
            error: error(),
            InvoiceFormView {
                contacts,
                products,
                submit_label: if loading() { "Creating..." } else { "Create invoice" },
                onsubmit: move |draft: Result<InvoiceDraft, String>| async move {
                    let Ok(draft) = draft else {
                        error.set(Some(draft.unwrap_err()));
                        return;
                    };
                    submit_form(loading, error, navigator, Route::IndexInvoices {}, || async move {
                        create_invoice(draft)
                            .await
                            .map(|_| ())
                            .map_err(|message| message.to_string())
                    })
                    .await;
                },
            }
        }
    }
}
