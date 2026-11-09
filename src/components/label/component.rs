use dioxus::prelude::*;
use dioxus_primitives::label::{self, LabelProps};
#[css_module("/src/components/label/style.css")]
struct Styles;

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label::Label {
            class: "{Styles::dx_label} flex w-full flex-col items-stretch gap-2 text-left text-sm font-medium text-zinc-700 dark:text-zinc-300",
            html_for: props.html_for,
            attributes: props.attributes,
            {props.children}
        }
    }
}
