use dioxus::prelude::*;

#[component]
pub fn Table(children: Element) -> Element {
    rsx! {
        div { class: "overflow-hidden rounded-xl border border-zinc-200 bg-white shadow-sm dark:border-zinc-700 dark:bg-zinc-900",
            div { class: "overflow-x-auto",
                table { class: "w-full min-w-[42rem] text-left text-sm", {children} }
            }
        }
    }
}

#[component]
pub fn TableHead(children: Element) -> Element {
    rsx! {
        thead { class: "bg-zinc-50 text-xs uppercase tracking-wide text-zinc-500 dark:bg-zinc-800/60 dark:text-zinc-400",
            {children}
        }
    }
}

#[component]
pub fn TableBody(children: Element) -> Element {
    rsx! {
        tbody { {children} }
    }
}

#[component]
pub fn TableRow(children: Element) -> Element {
    rsx! {
        tr { class: "border-t border-zinc-100 dark:border-zinc-800", {children} }
    }
}

#[component]
pub fn TableHeaderCell(
    children: Element,
    #[props(default)] align_end: bool,
    #[props(default, into)] class: String,
    #[props(extends = GlobalAttributes)] props: Vec<Attribute>,
) -> Element {
    let alignment = if align_end { "text-right" } else { "" };
    rsx! {
        th { class: "px-4 py-3 font-semibold {alignment} {class}", ..props, {children} }
    }
}

#[component]
pub fn TableCell(
    children: Element,
    #[props(default)] align_end: bool,
    #[props(default, into)] class: String,
    #[props(extends = GlobalAttributes)] props: Vec<Attribute>,
) -> Element {
    let alignment = if align_end { "text-right" } else { "" };
    rsx! {
        td { class: "px-4 py-4 {alignment} {class}", ..props, {children} }
    }
}
