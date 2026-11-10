use dioxus::prelude::*;

use crate::components::ProductFormView;
use crate::models::ProductForm;
use crate::products::{get_product, update_product};
use crate::Route;

#[component]
pub fn EditProduct(id: i32) -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let product = use_server_future(move || get_product(id))?.suspend()?;

    match product() {
        Ok(Some(product)) => rsx! {
            div { class: "mx-auto flex w-full max-w-3xl flex-col gap-8 px-6 py-8",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Edit product"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Update the details for {product.name}."
                    }
                }
                if let Some(message) = error() {
                    p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                        "{message}"
                    }
                }
                ProductFormView {
                    initial: product,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
                    onsubmit: move |draft: ProductForm| async move {
                        if loading() {
                            return;
                        }
                        loading.set(true);
                        error.set(None);
                        match update_product(id, draft).await {
                            Ok(_) => {
                                navigator.replace(Route::IndexProducts {});
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
        Ok(None) => rsx! {
            div { class: "mx-auto flex w-full max-w-3xl flex-col gap-4 px-6 py-8",
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                    "Product not found"
                }
                Link {
                    to: Route::IndexProducts {},
                    class: "text-zinc-700 underline dark:text-zinc-300",
                    "Back to products"
                }
            }
        },
        Err(message) => rsx! {
            div { class: "mx-auto w-full max-w-3xl px-6 py-8",
                p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
                    "{message}"
                }
            }
        },
    }
}