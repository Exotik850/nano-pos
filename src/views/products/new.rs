use dioxus::fullstack::Form;
use dioxus::prelude::*;

use crate::Route;
use crate::components::{PageError, PageHeader, PageShell, ProductFormView};
use crate::models::ProductForm;
use crate::products::create_product;

#[component]
pub fn NewProduct() -> Element {
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    rsx! {
        PageShell { max_width: "max-w-3xl",
            PageHeader {
                title: "Add product",
                description: "Create an item for your inventory.",
            }
            if let Some(message) = error() {
                PageError { message }
            }
            ProductFormView {
                submit_label: if loading() { "Creating..." } else { "Create product" },
                onsubmit: move |draft: ProductForm| async move {
                    if loading() {
                        return;
                    }
                    loading.set(true);
                    error.set(None);
                    match create_product(Form(draft)).await {
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
    }
}
