use dioxus::prelude::*;

use crate::components::{Button, ButtonVariant};
use crate::Route;

#[component]
pub fn Sidebar() -> Element {
    let auth = use_context::<crate::auth::AuthContext>();
    let mut hidden = use_signal(|| false);

    if !auth.session().is_some() {
       return rsx! {
          div {
            class: "flex min-h-screen w-full",
            main {
              class: "min-w-0 flex-1",
              Outlet::<Route> {}
            }
          } 
       };
    }

    rsx! {
        div {
            class: "flex min-h-screen w-full",
            if !hidden() {
                aside {
                    class: "flex w-56 shrink-0 flex-col gap-6 border-r border-zinc-200 px-4 py-4 dark:border-zinc-700",
                    div { class: "flex items-center justify-between",
                        h2 { class: "text-sm font-semibold uppercase tracking-wide text-zinc-500 dark:text-zinc-400", "Workspace" }
                        Button {
                            class: "px-2 py-1 text-xs",
                            variant: ButtonVariant::Secondary,
                            r#type: "button",
                            onclick: move |_| hidden.set(true),
                            "Hide"
                        }
                    }
                    nav { class: "flex flex-col gap-1", "aria-label": "Main navigation",
                        Link {
                            class: "rounded-lg px-3 py-2 text-sm font-medium text-zinc-700 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800",
                            to: Route::Home {},
                            "Home"
                        }
                        Link {
                            class: "rounded-lg px-3 py-2 text-sm font-medium text-zinc-700 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800",
                            to: Route::IndexProducts {},
                            "Products"
                        }
                    }
                }
            } else {
                div { class: "flex shrink-0 items-start border-r border-zinc-200 px-2 py-4 dark:border-zinc-700",
                    Button {
                        class: "px-2 py-1 text-xs",
                        variant: ButtonVariant::Secondary,
                        r#type: "button",
                        onclick: move |_| hidden.set(false),
                        "Show"
                    }
                }
            }
            main { class: "min-w-0 flex-1",
                Outlet::<Route> {}
            }
        }
    }
}