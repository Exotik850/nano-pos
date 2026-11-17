use dioxus::fullstack::Form;
use dioxus::prelude::*;

use crate::Route;
use crate::components::{FormPage, ProductFormView, submit_form};
use crate::models::ProductForm;
use crate::products::create_product;

#[component]
pub fn NewProduct() -> Element {
    let navigator = use_navigator();
    let error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);

    rsx! {
        FormPage {
            title: "Add product",
            description: "Create an item for your inventory.",
            max_width: "max-w-3xl",
            error: error(),
            ProductFormView {
                submit_label: if loading() { "Creating..." } else { "Create product" },
                onsubmit: move |draft: ProductForm| async move {
                    submit_form(loading, error, navigator, Route::IndexProducts {}, || async move {
                        create_product(Form(draft))
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
