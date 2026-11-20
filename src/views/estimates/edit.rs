use dioxus::prelude::*;

use crate::Route;
use crate::components::{EditPage, EstimateFormView, submit_form};
use crate::contacts::list_contacts;
use crate::estimates::{get_estimate, update_estimate};
use crate::models::EstimateDraft;
use crate::products::list_products;

#[component]
pub fn EditEstimate(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let estimate = use_server_future(move || get_estimate(id))?.suspend()?;
    let contacts = use_loader(list_contacts)?;
    let products = use_loader(list_products)?;
    let result = estimate();
    let load_error = result.as_ref().err().map(ToString::to_string);
    let initial = result.ok().flatten();
    let found = initial.is_some();
    let contacts = contacts();
    let products = products();
    let description = initial.as_ref().map_or_else(
        || "Update this estimate.".to_string(),
        |estimate| format!("Update estimate {}.", estimate.estimate.doc_number),
    );

    rsx! {
        EditPage {
            title: "Edit estimate",
            description,
            resource: "Estimate",
            back_to: Route::IndexEstimates {},
            back_label: "Back to estimates",
            error: error(),
            load_error,
            max_width: "max-w-5xl",
            found,
            if let Some(estimate) = initial {
                EstimateFormView {
                    initial: estimate,
                    contacts,
                    products,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
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
                                    update_estimate(id, draft)
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
