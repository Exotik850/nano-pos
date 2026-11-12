use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectMultiProps, SelectOptionProps, SelectProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::components::label::Label;

#[css_module("/src/components/select/style.css")]
struct Styles;

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::Select {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger { class: "{Styles::dx_select_trigger} w-full rounded-lg border border-zinc-300 bg-white px-3 py-2 text-left text-zinc-900 outline-none transition focus:border-zinc-500 dark:border-zinc-600 dark:bg-zinc-800 dark:text-zinc-100",
                select::SelectValue {}
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList { class: "{Styles::dx_select_list} max-h-60 overflow-y-auto p-1", {props.children} }
        }
    }
}

#[component]
pub fn FormSelect<T: Clone + PartialEq + 'static>(
    #[props(into)] label: String,
    #[props(into)] name: String,
    #[props(default, into)] class: String,
    #[props(default, into)] required: bool,
    #[props(default)] value: Option<ReadSignal<Option<T>>>,
    #[props(default)] default_value: Option<T>,
    #[props(default)] on_value_change: Callback<Option<T>>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] open: ReadSignal<Option<bool>>,
    #[props(default)] default_open: ReadSignal<bool>,
    #[props(default)] on_open_change: Callback<bool>,
    #[props(default)] roving_loop: ReadSignal<bool>,
    #[props(default)] typeahead_timeout: ReadSignal<std::time::Duration>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex w-full flex-col gap-1",
            Label {
                class: "flex flex-col gap-1 text-sm font-medium text-zinc-700 dark:text-zinc-300",
                html_for: "{name}",
                "{label}"
            }
            Select::<T> {
                id: "{name}",
                class: "w-full {class}",
                value,
                default_value,
                on_value_change,
                disabled,
                open,
                default_open,
                on_open_change,
                name,
                roving_loop,
                typeahead_timeout,
                aria_required: required,
                attributes,
                {children}
            }
        }
    }
}

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectMulti {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger { class: Styles::dx_select_trigger,
                select::SelectValue {}
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList { class: "{Styles::dx_select_list} max-h-60 overflow-y-auto p-1", {props.children} }
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_group_label
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectGroupLabel { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_option,
        class: "cursor-pointer rounded-md px-3 py-2 text-sm transition-colors hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-700 dark:hover:text-zinc-100",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            select::SelectItemIndicator {
                Check { size: "1rem", stroke: "var(--secondary-color-5)" }
            }
        }
    }
}
