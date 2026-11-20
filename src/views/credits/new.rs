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
    let contacts = use_loader(list_contacts)?;
    let products = use_loader(list_products)?;
    let contacts = contacts();
    let products = products();
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
                    submit_form(
                            loading,
                            error,
                            navigator,
                            Route::IndexCredits {},
                            || async move {
                                create_credit(draft)
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
