use dioxus::prelude::*;

use crate::{
    components::{Input, input::FormInput, label::Label, textarea::Textarea}, models::{Product, ProductForm},
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
        .map(|product| format!("{:.2}", product.price as f64 / 100.0))
        .unwrap_or_default();
    let mut name = use_signal(|| {
        initial
            .as_ref()
            .map(|product| product.name.clone())
            .unwrap_or_default()
    });
    let mut sku = use_signal(|| {
        initial
            .as_ref()
            .map(|product| product.sku.clone())
            .unwrap_or_default()
    });
    let mut description = use_signal(|| {
        initial
            .as_ref()
            .and_then(|product| product.description.clone())
            .unwrap_or_default()
    });
    let mut price = use_signal(|| initial_price);
    let mut quantity = use_signal(|| {
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
                  description: if description().is_empty() { None } else { Some(description()) },
                  price: (price().parse::<f64>().unwrap_or(0.0) * 100.0).round() as i32,
                  quantity: quantity().parse::<i32>().unwrap_or(0),
                };
                onsubmit.call(form);
            }, 
            div {
              class: "flex flex-row gap-4",
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

            Label {
                html_for: "description",
                "Description"
                Textarea { 
                  class: "min-h-32 w-full",
                  value: description(),
                  placeholder: "Enter a description for the product",
                  resize: "none",
                  oninput: move |event: FormEvent| description.set(event.value()),
                }
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

                // Label {
                //   html_for: "price",
                //   "Price"
                // }
                // Input {
                //   class: "w-full",
                //   id: "price",
                //   r#type: "number",
                //   required: true,
                //   min: "0",
                //   step: "0.01",
                //   value: price(),
                //   oninput: move |event: FormEvent| price.set(event.value()),
                // }
                // Label {
                //   html_for: "quantity",
                //   "Quantity"
                // }
                // Input {
                //   class: "w-full",
                //   id: "quantity",
                //   r#type: "number",
                //   required: true,
                //   min: "0",
                //   step: "1",
                //   value: quantity(),
                //   oninput: move |event: FormEvent| quantity.set(event.value()),
                // }
            }
            Button { class: "self-start px-5 py-2.5", r#type: "submit", "{submit_label}" }
        }
    }
}
