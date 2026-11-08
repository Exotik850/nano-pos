use dioxus::prelude::*;

use crate::Route;
use crate::components::{Button, ButtonVariant};


#[component]
pub fn Sidebar() -> Element {
    let mut hidden = use_signal(|| false);

    rsx! {
        div {
            class: "w-screen flex flex-row",

          if !hidden() {
            div {
              class: "flex flex-col gap-2 border-r border-zinc-200 px-6 py-3 dark:border-zinc-700 items-stretch",

              Button {
                variant: ButtonVariant::Secondary,
                r#type: "button",
                onclick: move |_| {
                  hidden.set(true);
                },
                "Hide Sidebar"
              }
  
              Link {
                  class: "font-semibold text-zinc-900 hover:underline dark:text-zinc-100",
                  to: Route::Home {},
                  "Home"
              }
            }
          } else {
            Button {
              variant: ButtonVariant::Secondary,
              r#type: "button",
              onclick: move |_| {
                hidden.set(false);
              },
              "Show Sidebar"
            }
          }

          div {
            class: "flex flex-col border-r border-zinc-200 py-3 dark:border-zinc-700 w-full",
            Outlet::<Route> {}
          }

        }
    }
}