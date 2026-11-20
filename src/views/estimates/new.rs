use dioxus::prelude::*;

use crate::Route;
use crate::components::{EstimateFormView, FormPage, submit_form};
use crate::contacts::list_contacts;
use crate::estimates::create_estimate;
use crate::models::EstimateDraft;
use crate::products::list_products;

#[component]
pub fn NewEstimate() -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let contacts = use_loader(list_contacts)?;
    let products = use_loader(list_products)?;
    let contacts = contacts();
    let products = products();

    rsx! {
        FormPage {
            title: "Add estimate",
            description: "Create an estimate and attach products.",
            max_width: "max-w-5xl",
            error: error(),
            EstimateFormView {
                contacts,
                products,
                submit_label: if loading() { "Creating..." } else { "Create estimate" },
                onsubmit: move |draft: Result<EstimateDraft, String>| async move {
                    let Ok(draft) = draft else {
                        error.set(Some(draft.unwrap_err()));
                        return;
                    };
                    submit_form(
                            loading,
                            error,
                            navigator,
                            Route::IndexEstimates {},
                            || async move {
                                create_estimate(draft)
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
