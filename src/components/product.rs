use dioxus::prelude::*;

use crate::{
    components::{input::FormInput, textarea::FormTextarea},
    models::{Product, ProductForm},
};

use super::Button;

#[component]
pub fn ProductFormView(
    #[props(default)] initial: Option<Product>,
    submit_label: String,
    onsubmit: EventHandler<ProductForm>,
) -> Element {
    let initial_price = initial
        .as_ref()
        .map(|product| format!("{:.2}", f64::from(product.price) / 100.0))
        .unwrap_or_default();
    let name = use_signal(|| {
        initial
            .as_ref()
            .map(|product| product.name.clone())
            .unwrap_or_default()
    });
    let sku = use_signal(|| {
        initial
            .as_ref()
            .map(|product| product.sku.clone())
            .unwrap_or_default()
    });
    let description = use_signal(|| {
        initial
            .as_ref()
            .and_then(|product| product.description.clone())
            .unwrap_or_default()
    });
    let price = use_signal(|| initial_price);
    let quantity = use_signal(|| {
        initial
            .as_ref()
            .map(|product| product.quantity.to_string())
            .unwrap_or_default()
    });

    rsx! {
        form {
            class: "flex w-full max-w-4xl flex-col gap-6",
            onsubmit: move |event| {
                event.prevent_default();
                let form = ProductForm {
                    name: name(),
                    sku: sku(),
                    description: if description().is_empty() {
                        None
                    } else {
                        Some(description())
                    },
                    price: (price().parse::<f64>().unwrap_or(0.0) * 100.0).round() as i32,
                    quantity: quantity().parse::<i32>().unwrap_or(0),
                };
                onsubmit.call(form);
            },
            div { class: "flex flex-row gap-4",
                FormInput {
                    label: "Name",
                    name: "name",
                    r#type: "text",
                    required: true,
                    value: name,
                }
                FormInput {
                    label: "SKU",
                    name: "sku",
                    r#type: "text",
                    required: true,
                    value: sku,
                }
            }

            FormTextarea {
                label: "Description",
                name: "description",
                value: description,
                placeholder: "Enter a description for the product",
                resize: "none",
                class: "min-h-32 w-full",
            }

            div { class: "flex flex-row gap-4",
                FormInput {
                    label: "Price",
                    name: "price",
                    r#type: "number",
                    required: true,
                    min: "0",
                    step: "0.01",
                    value: price,
                }
                FormInput {
                    label: "Quantity",
                    name: "quantity",
                    r#type: "number",
                    required: true,
                    min: "0",
                    step: "1",
                    value: quantity,
                }
            }

            Button { class: "self-center w-1/4 px-5 py-2.5", r#type: "submit", "{submit_label}" }
        }
    }
}
