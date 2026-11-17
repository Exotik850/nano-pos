use crate::Route;
use crate::components::{CreditFormView, FormPage, submit_form};
use crate::contacts::list_contacts;
use crate::credits::create_credit;
use crate::models::CreditDraft;
use crate::products::list_products;
use dioxus::prelude::*;

#[component]
pub fn NewCredit() -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;
    rsx! {
        FormPage {
            title: "Add credit",
            description: "Create a customer credit and attach products.",
            max_width: "max-w-5xl",
            error: error(),
            CreditFormView {
                contacts,
                products,
                submit_label: if loading() { "Creating..." } else { "Create credit" },
                onsubmit: move |draft: Result<CreditDraft, String>| async move {
                    let Ok(draft) = draft else {
                        error.set(Some(draft.unwrap_err()));
                        return;
                    };
                    submit_form(loading, error, navigator, Route::IndexCredits {}, || async move {
                        create_credit(draft)
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
