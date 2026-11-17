use dioxus::prelude::*;

use crate::Route;
use crate::components::{
    Button, ButtonLink, ButtonVariant, EmptyState, PageHeader, PageShell, Table, TableBody,
    TableCell, TableHead, TableHeaderCell, TableRow,
};
use crate::models::Product;
use crate::products::{delete_product, list_products};

#[component]
fn ProductTableRow(product: Product, ondelete: EventHandler<i32>) -> Element {
    let description = product
        .description
        .clone()
        .unwrap_or_else(|| "No description".to_string());
    let price = format!("${:.2}", f64::from(product.price) / 100.0);

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
                div { class: "flex justify-end gap-2",
                    ButtonLink {
                        class: "px-3 py-1.5 text-sm",
                        to: Route::EditProduct {
                            id: product.id,
                        },
                        variant: ButtonVariant::Secondary,
                        "Edit"
                    }
                    Button {
                        class: "px-3 py-1.5 text-sm",
                        variant: ButtonVariant::Secondary,
                        r#type: "button",
                        onclick: move |_| ondelete.call(product.id),
                        "Delete"
                    }
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
    let products = use_server_future(list_products)?.suspend()?;
    let products_ready = products().context("Failed to get products")?;
    let mut prods = use_signal(|| products_ready.clone());

    if let Some(message) = error() {
        return Err(ServerFnError::Response(message).into());
    }

    rsx! {
        PageShell { max_width: "max-w-6xl".to_string(),
            PageHeader {
                title: "Products".to_string(),
                description: "Manage your inventory and pricing.".to_string(),
                action: rsx! { ButtonLink { to: Route::NewProduct {}, "Add product" } },
            }
            if prods.is_empty() {
                EmptyState {
                    title: "No products yet".to_string(),
                    description: "Add your first product to start tracking inventory.".to_string(),
                }
            } else {
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
}
