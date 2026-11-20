use dioxus::prelude::*;

use crate::Route;
use crate::components::{
    IndexPage, Table, TableActions, TableBody, TableCell, TableHead, TableHeaderCell, TableRow,
    format_price,
};
use crate::models::Product;
use crate::products::{delete_product, list_products};

#[component]
fn ProductTableRow(product: Product, ondelete: EventHandler<i32>) -> Element {
    let description = product
        .description
        .clone()
        .unwrap_or_else(|| "No description".to_string());
    let price = format_price(product.price);
    let id = product.id;

    rsx! {
        TableRow {
            TableCell {
                div { class: "font-medium text-zinc-900 dark:text-zinc-100", "{product.name}" }
                div { class: "mt-1 max-w-xs truncate text-sm text-zinc-500 dark:text-zinc-400",
                    "{description}"
                }
            }
            TableCell {
                align_end: true,
                class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300",
                "{product.sku}"
            }
            TableCell {
                align_end: true,
                class: "whitespace-nowrap font-medium text-zinc-900 dark:text-zinc-100",
                "{price}"
            }
            TableCell {
                align_end: true,
                class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300",
                "{product.quantity}"
            }
            TableCell {
                TableActions {
                    edit_to: Route::EditProduct { id },
                    ondelete: move |_| ondelete.call(id),
                }
            }
        }
    }
}

#[component]
fn ProductTable(products: ReadSignal<Vec<Product>>, ondelete: EventHandler<i32>) -> Element {
    rsx! {
        Table {
            TableHead {
                tr {
                    TableHeaderCell { "Product" }
                    TableHeaderCell { align_end: true, "SKU" }
                    TableHeaderCell { align_end: true, "Price" }
                    TableHeaderCell { align_end: true, "In stock" }
                    TableHeaderCell { align_end: true, "Actions" }
                }
            }
            TableBody {
                for product in products() {
                    ProductTableRow { product, ondelete }
                }
            }
        }
    }
}

#[component]
pub fn IndexProducts() -> Element {
    let mut error = use_signal(|| None::<String>);
    let products = use_loader(list_products)?;
    let products_ready = products();
    let mut prods = use_signal(|| products_ready.clone());

    if let Some(message) = error() {
        return Err(ServerFnError::Response(message).into());
    }

    rsx! {
        IndexPage {
            title: "Products",
            description: "Manage your inventory and pricing.",
            empty_title: "No products yet",
            empty_description: "Add your first product to start tracking inventory.",
            is_empty: prods.is_empty(),
            action: rsx! {
                crate::components::ButtonLink { to: Route::NewProduct {}, "Add product" }
            },
            ProductTable {
                products: prods,
                ondelete: move |product_id| async move {
                    if let Err(message) = delete_product(product_id).await {
                        warn!("Failed to delete product: {}", message);
                        error.set(Some(message.to_string()));
                    } else {
                        prods.retain(|product| product.id != product_id);
                    }
                },
            }
        }
    }
}
