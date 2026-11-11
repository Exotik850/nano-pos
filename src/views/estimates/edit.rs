use dioxus::prelude::*;

use crate::Route;
use crate::components::EstimateFormView;
use crate::contacts::list_contacts;
use crate::estimates::{get_estimate, update_estimate};
use crate::models::EstimateDraft;
use crate::products::list_products;

#[component]
pub fn EditEstimate(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut estimate_opt = use_signal(|| None);
    let estimate = use_server_future(move || get_estimate(id))?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    estimate_opt.set(estimate().context("Failed to get estimate")?);
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;

    match estimate_opt() {
        Some(estimate) => rsx! {
            div { class: "mx-auto flex w-full max-w-5xl flex-col gap-8 px-6 py-8",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Edit estimate"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Update estimate {estimate.estimate.doc_number}."
                    }
                }
                if let Some(message) = error() {
                    p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                        "{message}"
                    }
                }
                EstimateFormView {
                    initial: estimate,
                    contacts,
                    products,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
                    onsubmit: move |draft: Result<EstimateDraft, String>| async move {
                        if loading() {
                            return;
                        }
                        let Ok(draft) = draft else {
                            error.set(Some(draft.unwrap_err()));
                            return;
                        };
                        loading.set(true);
                        error.set(None);
                        match update_estimate(id, draft).await {
                            Ok(_) => {
                                navigator.replace(Route::IndexEstimates {});
                            }
                            Err(message) => {
                                error.set(Some(message.to_string()));
                                loading.set(false);
                            }
                        }
                    },
                }
            }
        },
        None => rsx! {
            div { class: "mx-auto flex w-full max-w-3xl flex-col gap-4 px-6 py-8",
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                    "Estimate not found"
                }
                Link {
                    to: Route::IndexEstimates {},
                    class: "text-zinc-700 underline dark:text-zinc-300",
                    "Back to estimates"
                }
            }
        },
    }
}
