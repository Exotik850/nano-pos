use crate::Route;
use crate::components::CreditFormView;
use crate::contacts::list_contacts;
use crate::credits::{get_credit, update_credit};
use crate::models::CreditDraft;
use crate::products::list_products;
use dioxus::prelude::*;

#[component]
pub fn EditCredit(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let credit = use_server_future(move || get_credit(id))?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let products = use_server_future(list_products)?.suspend()?;
    let credit = credit().context("Failed to get credit")?;
    let contacts = contacts().context("Failed to get contacts")?;
    let products = products().context("Failed to get products")?;
    match credit {
        Some(credit) => rsx! {
            div { class: "mx-auto flex w-full max-w-5xl flex-col gap-8 px-6 py-8",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Edit credit"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Update credit {credit.credit.doc_number}."
                    }
                }
                if let Some(message) = error() {
                    p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                        "{message}"
                    }
                }
                CreditFormView {
                    initial: credit,
                    contacts,
                    products,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
                    onsubmit: move |draft: Result<CreditDraft, String>| async move {
                        if loading() {
                            return;
                        }
                        let Ok(draft) = draft else {
                            error.set(Some(draft.unwrap_err()));
                            return;
                        };
                        loading.set(true);
                        error.set(None);
                        match update_credit(id, draft).await {
                            Ok(_) => {
                                navigator.replace(Route::IndexCredits {});
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
                    "Credit not found"
                }
                Link {
                    to: Route::IndexCredits {},
                    class: "text-zinc-700 underline dark:text-zinc-300",
                    "Back to credits"
                }
            }
        },
    }
}
