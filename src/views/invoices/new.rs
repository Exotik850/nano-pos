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
    let contacts = use_loader(list_contacts)?;
    let products = use_loader(list_products)?;
    let contacts = contacts();
    let products = products();

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
                    submit_form(
                            loading,
                            error,
                            navigator,
                            Route::IndexInvoices {},
                            || async move {
                                create_invoice(draft)
                                    .await
                                    .map(|_| ())
                                    .map_err(|message| message.to_string())
                            },
                        )
                        .await;
                },
            }
        }
    }
}
