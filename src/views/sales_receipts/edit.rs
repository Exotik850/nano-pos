use crate::Route;
use crate::components::{EditPage, SalesReceiptFormView, submit_form};
use crate::contacts::list_contacts;
use crate::models::SalesReceiptDraft;
use crate::products::list_products;
use crate::sales_receipts::{get_sales_receipt, update_sales_receipt};
use dioxus::prelude::*;

#[component]
pub fn EditSalesReceipt(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let receipt = use_server_future(move || get_sales_receipt(id))?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    let result = receipt();
    let load_error = result.as_ref().err().map(ToString::to_string);
    let initial = result.ok().flatten();
    let found = initial.is_some();
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;
    let description = initial.as_ref().map_or_else(
        || "Update this sales receipt.".to_string(),
        |receipt| format!("Update sales receipt {}.", receipt.sales_receipt.doc_number),
    );

    rsx! {
        EditPage {
            title: "Edit sales receipt",
            description,
            resource: "Sales receipt",
            back_to: Route::IndexSalesReceipts {},
            back_label: "Back to sales receipts",
            error: error(),
            load_error,
            max_width: "max-w-5xl",
            found,
            if let Some(receipt) = initial {
                SalesReceiptFormView {
                    initial: receipt,
                    contacts,
                    products,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
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
                                    update_sales_receipt(id, draft)
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
}
