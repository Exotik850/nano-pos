use crate::Route;
use crate::components::{FormPage, SalesReceiptFormView, submit_form};
use crate::contacts::list_contacts;
use crate::models::SalesReceiptDraft;
use crate::products::list_products;
use crate::sales_receipts::create_sales_receipt;
use dioxus::prelude::*;

#[component]
pub fn NewSalesReceipt() -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;
    rsx! {
        FormPage {
            title: "Add sales receipt",
            description: "Create a sales receipt and attach products.",
            max_width: "max-w-5xl",
            error: error(),
            SalesReceiptFormView {
                contacts,
                products,
                submit_label: if loading() { "Creating..." } else { "Create sales receipt" },
                onsubmit: move |draft: Result<SalesReceiptDraft, String>| async move {
                    let Ok(draft) = draft else {
                        error.set(Some(draft.unwrap_err()));
                        return;
                    };
                    submit_form(
                            loading,
                            error,
                            navigator,
                            Route::IndexSalesReceipts {},
                            || async move {
                                create_sales_receipt(draft)
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
