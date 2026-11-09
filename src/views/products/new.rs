use dioxus::fullstack::Form;
use dioxus::prelude::*;

use crate::components::ProductFormView;
use crate::models::ProductForm;
use crate::products::{create_product};
use crate::Route;

#[component]
pub fn NewProduct() -> Element {
  let navigator = use_navigator();
  let mut error = use_signal(|| None::<String>);
  let mut loading = use_signal(|| false);

  rsx! {
    div { class: "mx-auto flex w-full max-w-3xl flex-col gap-8 px-6 py-8",
      div {
        h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", "Add product" }
        p { class: "mt-1 text-zinc-500 dark:text-zinc-400", "Create an item for your inventory." }
      }
      if let Some(message) = error() {
        p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200", "{message}" }
      }
      ProductFormView {
        submit_label: if loading() { "Creating..." } else { "Create product" },
        onsubmit: move |draft: ProductForm| async move {
          if loading() { return; }
          loading.set(true);
          error.set(None);
          match create_product(Form(draft)).await {
            Ok(_) => { navigator.replace(Route::IndexProducts {}); }
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