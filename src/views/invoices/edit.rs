use dioxus::prelude::*;

use crate::Route;
use crate::components::{EditPage, InvoiceFormView, submit_form};
use crate::contacts::list_contacts;
use crate::invoices::{get_invoice, update_invoice};
use crate::models::InvoiceDraft;
use crate::products::list_products;

#[component]
pub fn EditInvoice(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let invoice = use_server_future(move || get_invoice(id))?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    let result = invoice();
    let load_error = result.as_ref().err().map(ToString::to_string);
    let initial = result.ok().flatten();
    let found = initial.is_some();
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;
    let description = initial.as_ref().map_or_else(
        || "Update this invoice.".to_string(),
        |invoice| format!("Update invoice {}.", invoice.invoice.doc_number),
    );

    rsx! {
        EditPage {
            title: "Edit invoice",
            description,
            resource: "Invoice",
            back_to: Route::IndexInvoices {},
            back_label: "Back to invoices",
            error: error(),
            load_error,
            max_width: "max-w-5xl",
            found,
            if let Some(invoice) = initial {
                InvoiceFormView {
                    initial: invoice,
                    contacts,
                    products,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
                    onsubmit: move |draft: Result<InvoiceDraft, String>| async move {
                        let Ok(draft) = draft else {
                            error.set(Some(draft.unwrap_err()));
                            return;
                        };
                        submit_form(loading, error, navigator, Route::IndexInvoices {}, || async move {
                            update_invoice(id, draft)
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
}
