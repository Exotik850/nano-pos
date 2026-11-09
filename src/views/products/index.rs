use dioxus::prelude::*;

use crate::components::{
  Button, ButtonLink, ButtonVariant, Table, TableBody, TableCell, TableHead, TableHeaderCell,
  TableRow,
};
use crate::models::Product;
use crate::products::{delete_product, list_products};
use crate::Route;

#[component]
fn ProductTableRow(product: Product, ondelete: EventHandler<i32>) -> Element {
  let description = product.description.clone().unwrap_or_else(|| "No description".to_string());
  let price = format!("${:.2}", product.price as f64 / 100.0);

  rsx! {
    TableRow {
      TableCell {
        div { class: "font-medium text-zinc-900 dark:text-zinc-100", "{product.name}" }
        div { class: "mt-1 max-w-xs truncate text-sm text-zinc-500 dark:text-zinc-400", "{description}" }
      }
      TableCell { align_end: true, class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{product.sku}" }
      TableCell { align_end: true, class: "whitespace-nowrap font-medium text-zinc-900 dark:text-zinc-100", "{price}" }
      TableCell { align_end: true, class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{product.quantity}" }
      TableCell {
        div { class: "flex justify-end gap-2",
          ButtonLink {
            class: "px-3 py-1.5 text-sm",
            to: Route::EditProduct { id: product.id },
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
  let navigator = use_navigator();
  let products = use_server_future(list_products)?.suspend()?;
  let mut error = use_signal(|| None::<String>);

  let products = products()?;

  rsx! {
    div { class: "mx-auto flex w-full max-w-6xl flex-col gap-8 px-6 py-8",
      div { class: "flex flex-wrap items-end justify-between gap-4",
        div {
          h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", "Products" }
          p { class: "mt-1 text-zinc-500 dark:text-zinc-400", "Manage your inventory and pricing." }
        }
        ButtonLink { to: Route::NewProduct {}, "Add product" }
      }
      if let Some(message) = error() {
        p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200", "{message}" }
      }

      if products.is_empty() {
          div { class: "rounded-xl border border-dashed border-zinc-300 px-6 py-16 text-center dark:border-zinc-700",
            h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100", "No products yet" }
            p { class: "mt-2 text-sm text-zinc-500 dark:text-zinc-400", "Add your first product to start tracking inventory." }
          }
      } else {
          ProductTable {
            products: products,
            ondelete: move |product_id| async move {
              error.set(None);
              if let Err(message) = delete_product(product_id).await {
                warn!("Failed to delete product: {}", message);
                error.set(Some(message.to_string()));
              } else {
                navigator.replace(Route::IndexProducts {});
              }
            },
          }
      }
    }
  }
}