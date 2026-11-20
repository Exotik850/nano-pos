use crate::Route;
use crate::components::{CreditFormView, EditPage, submit_form};
use crate::contacts::list_contacts;
use crate::credits::{get_credit, update_credit};
use crate::models::CreditDraft;
use crate::products::list_products;
use dioxus::prelude::*;

#[component]
pub fn EditCredit(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let credit = use_server_future(move || get_credit(id))?.suspend()?;
    let contacts = use_loader(list_contacts)?;
    let products = use_loader(list_products)?;
    let result = credit();
    let load_error = result.as_ref().err().map(ToString::to_string);
    let initial = result.ok().flatten();
    let found = initial.is_some();
    let contacts = contacts();
    let products = products();
    let description = initial.as_ref().map_or_else(
        || "Update this credit.".to_string(),
        |credit| format!("Update credit {}.", credit.credit.doc_number),
    );

    rsx! {
        EditPage {
            title: "Edit credit",
            description,
            resource: "Credit",
            back_to: Route::IndexCredits {},
            back_label: "Back to credits",
            error: error(),
            load_error,
            max_width: "max-w-5xl",
            found,
            if let Some(credit) = initial {
                CreditFormView {
                    initial: credit,
                    contacts,
                    products,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
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
                                    update_credit(id, draft)
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
