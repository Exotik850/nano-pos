use dioxus::prelude::*;
use dioxus_icons::lucide::Search;

#[component]
pub fn SearchBar() -> Element {
  rsx! {
    div {
      class: "flex flex-1 items-center justify-center px-4 sm:px-6",
      div {
        class: "flex w-full max-w-2xl items-center gap-3 rounded-xl border border-zinc-200/80 bg-zinc-50/80 px-4 py-2.5 text-zinc-900 shadow-sm transition-all focus-within:border-zinc-400 focus-within:bg-white focus-within:shadow-md dark:border-zinc-700/80 dark:bg-zinc-900/60 dark:text-zinc-100 dark:focus-within:border-zinc-500 dark:focus-within:bg-zinc-800",
        Search {
          size: 16,
          class: "shrink-0 text-zinc-400 dark:text-zinc-500",
        }
        input {
          class: "min-w-0 w-full bg-transparent text-sm text-zinc-900 placeholder-zinc-400 outline-none dark:text-zinc-100 dark:placeholder-zinc-500",
          r#type: "text",
          placeholder: "Search...",
        }
      }
    }
  }
}