use dioxus::prelude::*;

use crate::Route;
use crate::components::{EditPage, ProductFormView, submit_form};
use crate::models::ProductForm;
use crate::products::{get_product, update_product};

#[component]
pub fn EditProduct(id: i32) -> Element {
    let navigator = use_navigator();
    let error = use_signal(|| None::<String>);
    let loading = use_signal(|| false);
    let product = use_server_future(move || get_product(id))?.suspend()?;

    let result = product();
    let load_error = result.as_ref().err().map(ToString::to_string);
    let initial = result.ok().flatten();
    let found = initial.is_some();
    let description = initial.as_ref().map_or_else(
        || "Update this product.".to_string(),
        |product| format!("Update the details for {}.", product.name),
    );

    rsx! {
        EditPage {
            title: "Edit product",
            description,
            resource: "Product",
            back_to: Route::IndexProducts {},
            back_label: "Back to products",
            error: error(),
            load_error,
            max_width: "max-w-3xl",
            found,
            if let Some(product) = initial {
                ProductFormView {
                    initial: product,
                    submit_label: if loading() { "Saving..." } else { "Save changes" },
                    onsubmit: move |draft: ProductForm| async move {
                        submit_form(
                                loading,
                                error,
                                navigator,
                                Route::IndexProducts {},
                                || async move {
                                    update_product(id, draft)
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
