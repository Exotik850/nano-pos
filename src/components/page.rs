use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn PageShell(
  #[props(default, into)]
  max_width: String, 
  #[props(default, into)]
  children: Element) -> Element {
    rsx! {
        div { class: "mx-auto flex w-full flex-col gap-8 px-6 py-8 {max_width}", {children} }
    }
}

#[component]
pub fn PageHeader(
    #[props(default, into)]
    title: String,
    #[props(default, into)]
    description: String,
    #[props(default)] action: Option<Element>,
) -> Element {
    rsx! {
        div { class: "flex flex-wrap items-end justify-between gap-4",
            div {
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                    "{title}"
                }
                p { class: "mt-1 text-zinc-500 dark:text-zinc-400", "{description}" }
            }
            if let Some(action) = action {
                {action}
            }
        }
    }
}

#[component]
pub fn PageError(message: String) -> Element {
    rsx! {
        p { class: "rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700 dark:bg-red-950 dark:text-red-200",
            "{message}"
        }
    }
}

#[component]
pub fn EmptyState(title: String, description: String) -> Element {
    rsx! {
        div { class: "rounded-xl border border-dashed border-zinc-300 px-6 py-16 text-center dark:border-zinc-700",
            h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100", "{title}" }
            p { class: "mt-2 text-sm text-zinc-500 dark:text-zinc-400", "{description}" }
        }
    }
}

#[component]
pub fn NotFoundState(resource: String, back_to: Route, children: Element) -> Element {
    rsx! {
        PageShell { max_width: "max-w-3xl",
            h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", "{resource} not found" }
            Link {
                to: back_to,
                class: "text-zinc-700 underline dark:text-zinc-300",
                {children}
            }
        }
    }
}
