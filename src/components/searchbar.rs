use dioxus::prelude::*;
use dioxus_icons::lucide::Search;
use dioxus_primitives::popover::{PopoverContent, PopoverRoot};
use dioxus_sdk_time::use_debounce;
use std::time::Duration;

use crate::api::search::search_all;

#[component]
pub fn SearchBar(
  #[props(into, default)] class: String,
  #[props(extends = GlobalAttributes)]
  #[props(extends = div)] 
  prop: Vec<Attribute>,
) -> Element {

    let mut value = use_signal(String::new);
    let mut query = use_signal(String::new);
    let mut focused = use_signal(|| false);
    let mut debounce = use_debounce(Duration::from_millis(150), move |next_query: String| {
        query.set(next_query);
    });
    let results = use_resource(move || {
        let query = query().trim().to_owned();
        async move {
            if query.is_empty() {
                return Ok(Vec::new());
            }

            search_all(query).await
        }
    });

    rsx! {
        div { class: "flex flex-1 items-center justify-center px-4 sm:px-6 {class}",
            ..prop,
            PopoverRoot {
                open: focused(),
                is_modal: false,
                on_open_change: move |open| focused.set(open),
                class: "relative w-full max-w-2xl",
                div { class: "flex items-center gap-3 rounded-xl border border-zinc-200/80 bg-zinc-50/80 px-4 py-2.5 text-zinc-900 shadow-sm transition-all focus-within:border-zinc-400 focus-within:bg-white focus-within:shadow-md dark:border-zinc-700/80 dark:bg-zinc-900/60 dark:text-zinc-100 dark:focus-within:border-zinc-500 dark:focus-within:bg-zinc-800",
                    Search { size: 16, class: "shrink-0 text-zinc-400 dark:text-zinc-500" }
                    input {
                        class: "min-w-0 w-full bg-transparent text-sm text-zinc-900 placeholder-zinc-400 outline-none dark:text-zinc-100 dark:placeholder-zinc-500",
                        r#type: "search",
                        placeholder: "Search...",
                        value,
                        oninput: move |e| {
                            let next_value = e.value();
                            value.set(next_value.clone());
                            debounce.action(next_value);
                        },
                        onfocus: move |_| focused.set(true),
                    }
                }

                if focused() && !value().trim().is_empty() {
                    PopoverContent {
                        class: "absolute left-0 right-0 top-full z-50 mt-2 overflow-hidden rounded-xl border border-zinc-200 bg-white p-1.5 text-zinc-900 shadow-xl dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-100",
                        align: dioxus_primitives::ContentAlign::Start,
                        match results() {
                            Some(Ok(items)) if !items.is_empty() => rsx! {
                                div { class: "max-h-80 overflow-y-auto",
                                    for item in items {
                                        Link {
                                            class: "flex w-full flex-col gap-0.5 rounded-lg px-3 py-2 text-left transition-colors hover:bg-zinc-100 focus-visible:bg-zinc-100 focus-visible:outline-none dark:hover:bg-zinc-800 dark:focus-visible:bg-zinc-800",
                                            to: item.route,
                                            onclick: move |_| focused.set(false),
                                            span { class: "text-sm font-medium", "{item.title}" }
                                            span { class: "truncate text-xs text-zinc-500 dark:text-zinc-400", "{item.description}" }
                                        }
                                    }
                                }
                            },
                            Some(Ok(_)) => rsx! {
                                div { class: "px-3 py-2 text-sm text-zinc-500 dark:text-zinc-400", "No results found." }
                            },
                            Some(Err(error)) => rsx! {
                                div { class: "px-3 py-2 text-sm text-red-600 dark:text-red-400", "{error}" }
                            },
                            None => rsx! {
                                div { class: "flex items-center gap-2 px-3 py-2 text-sm text-zinc-500 dark:text-zinc-400",
                                    span { class: "h-3.5 w-3.5 animate-spin rounded-full border-2 border-zinc-300 border-t-zinc-700 dark:border-zinc-600 dark:border-t-zinc-200", aria_label: "Loading" }
                                    "Searching..."
                                }
                            },
                        }
                    },
                }
            }
        }
    }
}
