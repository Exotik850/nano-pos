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

#[component]
pub fn TableInput(
    #[props(into)] value: Signal<String>,
    #[props(default, into)] r#type: String,
    #[props(default, into)] placeholder: String,
    #[props(default, into)] name: String,
) -> Element {
    rsx! {
        input {
            class: "w-full rounded-md border border-transparent bg-transparent px-2.5 py-2 text-sm text-zinc-900 outline-none transition placeholder:text-zinc-400 hover:border-zinc-300 hover:bg-zinc-50 focus:border-zinc-500 focus:ring-2 focus:ring-zinc-200 dark:text-zinc-100 dark:hover:border-zinc-600 dark:hover:bg-zinc-800/60 dark:focus:border-zinc-400 dark:focus:ring-zinc-700",
            name,
            placeholder,
            r#type,
            value: "{value}",
            oninput: move |event| value.set(event.value()),
        }
    }
}

#[component]
pub fn TableSelect(
    #[props(into)] value: Signal<String>,
    children: Element,
) -> Element {
    rsx! {
        select {
            class: "w-full rounded-md border border-transparent bg-transparent px-2.5 py-2 text-sm text-zinc-900 outline-none transition hover:border-zinc-300 hover:bg-zinc-50 focus:border-zinc-500 focus:ring-2 focus:ring-zinc-200 dark:text-zinc-100 dark:hover:border-zinc-600 dark:hover:bg-zinc-800/60 dark:focus:border-zinc-400 dark:focus:ring-zinc-700",
            value: "{value}",
            onchange: move |event| value.set(event.value()),
            {children}
        }
    }
}
