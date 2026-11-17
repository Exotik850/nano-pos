use dioxus::prelude::*;
use dioxus_primitives::select::SelectGroup;

use crate::{
    components::select::{SelectGroupLabel, SelectOption},
    models::Product,
};

use super::{
    Button, ButtonVariant, Table, TableBody, TableCell, TableHead, TableHeaderCell, TableInput,
    TableRow, TableSelect,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ProductLineDraft {
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
}

#[component]
pub fn ProductLineEditor(
    products: Vec<Product>,
    mut lines: Signal<Vec<ProductLineDraft>>,
    heading: String,
    description: String,
    #[props(default)] action: Option<Element>,
) -> Element {
    let mut selected_product = use_signal(|| None);
    let mut quantity = use_signal(|| "1".to_string());
    let mut price = use_signal(|| "0".to_string());

    rsx! {
        div { class: "flex flex-col gap-4 rounded-xl border border-zinc-200 p-4 dark:border-zinc-700",
            div {
                div { class: "flex flex-wrap items-end justify-between gap-3",
                    div {
                        h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100",
                            "{heading}"
                        }
                        p { class: "text-sm text-zinc-500 dark:text-zinc-400", "{description}" }
                    }
                    if let Some(action) = action {
                        {action}
                    }
                }
            }
            Table {
                TableHead {
                    tr {
                        TableHeaderCell { "Product" }
                        TableHeaderCell { align_end: true, "Quantity" }
                        TableHeaderCell { align_end: true, "Price" }
                        TableHeaderCell { align_end: true, "Actions" }
                    }
                }
                TableBody {
                    TableRow {
                        TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40",
                            TableSelect::<i32> {
                                value: selected_product(),
                                on_value_change: move |product_id| {
                                    selected_product.set(product_id);
                                },
                                SelectGroup { class: "w-full",
                                    SelectGroupLabel { "Products" }
                                    for (index, product) in products.iter().enumerate() {
                                        SelectOption::<i32> {
                                            index,
                                            value: product.id,
                                            text_value: "{product.name} ({product.sku})",
                                            "{product.name} ({product.sku})"
                                        }
                                    }
                                }
                            }
                        }
                        TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40",
                            TableInput {
                                name: "line_quantity",
                                r#type: "number",
                                placeholder: "1",
                                value: quantity,
                            }
                        }
                        TableCell { class: "bg-zinc-50 dark:bg-zinc-800/40",
                            TableInput {
                                name: "line_price",
                                r#type: "number",
                                value: price,
                            }
                        }
                        TableCell {
                            align_end: true,
                            class: "bg-zinc-50 dark:bg-zinc-800/40",
                            Button {
                                r#type: "button",
                                onclick: move |_| {
                                    let Some(product_id) = selected_product() else {
                                        return;
                                    };
                                    let Ok(parsed_quantity) = quantity().parse::<i32>() else {
                                        return;
                                    };
                                    let parsed_price = (!price().is_empty())
                                        .then(|| price().parse::<i32>().ok())
                                        .flatten();
                                    lines
                                        .push(ProductLineDraft {
                                            product_id,
                                            quantity: parsed_quantity,
                                            price: parsed_price,
                                        });
                                    selected_product.set(None);
                                    quantity.set("1".to_string());
                                    price.set("0".to_string());
                                },
                                "Add line"
                            }
                        }
                    }
                    for line in lines() {
                        {
                            let product_name = products
                                .iter()
                                .find(|product| product.id == line.product_id)
                                .map_or_else(
                                    || format!("Product #{}", line.product_id),
                                    |product| product.name.clone(),
                                );
                            let line_price = line
                                .price
                                .map_or_else(
                                    || "-".to_string(),
                                    |value| format!("${:.2}", f64::from(value) / 100.0),
                                );
                            rsx! {
                                TableRow {
                                    TableCell { "{product_name}" }
                                    TableCell { align_end: true, "{line.quantity}" }
                                    TableCell { align_end: true, "{line_price}" }
                                    TableCell { align_end: true,
                                        Button {
                                            class: "px-3 py-1.5 text-sm",
                                            variant: ButtonVariant::Secondary,
                                            r#type: "button",
                                            onclick: move |_| lines.retain(|value| value.product_id != line.product_id),
                                            "Remove"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
