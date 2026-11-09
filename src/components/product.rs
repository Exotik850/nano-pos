use dioxus::prelude::*;

use crate::models::{Product, ProductForm};

use super::Button;

#[component]
pub fn ProductFormView(
    #[props(default)] initial: Option<Product>,
    submit_label: String,
    onsubmit: EventHandler<ProductForm>,
) -> Element {
    let initial_price = initial.as_ref().map(|product| format!("{:.2}", product.price as f64 / 100.0)).unwrap_or_default();
    let mut name = use_signal(|| initial.as_ref().map(|product| product.name.clone()).unwrap_or_default());
    let mut description = use_signal(|| initial.as_ref().and_then(|product| product.description.clone()).unwrap_or_default());
    let mut price = use_signal(|| initial_price);
    let mut quantity = use_signal(|| initial.as_ref().map(|product| product.quantity.to_string()).unwrap_or_default());

    rsx! {
        form { class: "flex max-w-2xl flex-col gap-5", onsubmit: move |event| {
            event.prevent_default();
            let form = ProductForm {
              name: name(),
              description: if description().is_empty() { None } else { Some(description()) },
              price: (price().parse::<f64>().unwrap_or(0.0) * 100.0).round() as i32,
              quantity: quantity().parse::<i32>().unwrap_or(0),
            };
            onsubmit.call(form);
        },
            label { class: "flex flex-col gap-1 text-sm font-medium text-zinc-700 dark:text-zinc-300", "Name",
                input { class: "rounded-lg border border-zinc-300 bg-white px-3 py-2 text-zinc-900 outline-none focus:border-zinc-500 dark:border-zinc-600 dark:bg-zinc-800 dark:text-zinc-100", required: true, value: name(), oninput: move |event| name.set(event.value()) }
            }
            label { class: "flex flex-col gap-1 text-sm font-medium text-zinc-700 dark:text-zinc-300", "Description",
                textarea { class: "min-h-24 rounded-lg border border-zinc-300 bg-white px-3 py-2 text-zinc-900 outline-none focus:border-zinc-500 dark:border-zinc-600 dark:bg-zinc-800 dark:text-zinc-100", value: description(), oninput: move |event| description.set(event.value()) }
            }
            div { class: "grid gap-5 sm:grid-cols-2",
                label { class: "flex flex-col gap-1 text-sm font-medium text-zinc-700 dark:text-zinc-300", "Price",
                    input { class: "rounded-lg border border-zinc-300 bg-white px-3 py-2 text-zinc-900 outline-none focus:border-zinc-500 dark:border-zinc-600 dark:bg-zinc-800 dark:text-zinc-100", required: true, r#type: "number", min: "0", step: "0.01", value: price(), oninput: move |event| price.set(event.value()) }
                }
                label { class: "flex flex-col gap-1 text-sm font-medium text-zinc-700 dark:text-zinc-300", "Quantity",
                    input { class: "rounded-lg border border-zinc-300 bg-white px-3 py-2 text-zinc-900 outline-none focus:border-zinc-500 dark:border-zinc-600 dark:bg-zinc-800 dark:text-zinc-100", required: true, r#type: "number", min: "0", step: "1", value: quantity(), oninput: move |event| quantity.set(event.value()) }
                }
            }
            Button { r#type: "submit", "{submit_label}" }
        }
    }
}