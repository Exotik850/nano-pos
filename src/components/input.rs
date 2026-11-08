use dioxus::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
  #[default]
  Primary,
  Secondary,
}







#[component]
pub fn Button(
  children: Element,
  #[props(default, into)]
  class: String,
  #[props(default)]
  variant: ButtonVariant,
  onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = Button)]
  button_props: Vec<Attribute>
) -> Element {
  let variant_class = match variant {
    ButtonVariant::Primary => "bg-zinc-900 text-white hover:bg-zinc-700 dark:bg-zinc-100 dark:text-zinc-900 dark:hover:bg-zinc-300",
    ButtonVariant::Secondary => "border border-zinc-300 text-zinc-800 hover:bg-zinc-100 dark:border-zinc-600 dark:text-zinc-200 dark:hover:bg-zinc-800",
  };

  rsx! {
    button {
      class: "cursor-pointer rounded-lg px-3 py-1.5 text-sm font-medium transition {variant_class} {class}",
      onclick: move |evt| {
        if let Some(handler) = onclick.as_ref() {
          handler.call(evt);
        }
      },
      ..button_props,
      {children},
    }
  }
}

#[component]
pub fn ButtonLink(
  #[props(into)] to: NavigationTarget,
  children: Element,
  #[props(default, into)] class: String,
  #[props(default)] variant: ButtonVariant,
) -> Element {
  let variant_class = match variant {
    ButtonVariant::Primary => "bg-zinc-900 text-white hover:bg-zinc-700 dark:bg-zinc-100 dark:text-zinc-900 dark:hover:bg-zinc-300",
    ButtonVariant::Secondary => "border border-zinc-300 text-zinc-800 hover:bg-zinc-100 dark:border-zinc-600 dark:text-zinc-200 dark:hover:bg-zinc-800",
  };

  rsx! {
    Link {
      class: "inline-block rounded-lg px-5 py-2.5 font-semibold transition {variant_class} {class}",
      to,
      {children},
    }
  }
}

#[component]
pub fn AuthField(
  label: String,
  name: String,
  #[props(default, into)] r#type: String,
  #[props(default, into)] placeholder: String,
  #[props(default, into)] autocomplete: String,
) -> Element {
  rsx! {
    label {
      class: "flex flex-col gap-1 text-sm font-medium text-zinc-700 dark:text-zinc-300",
      "{label}"
      input {
        class: "rounded-lg border border-zinc-300 bg-white px-3 py-2 text-zinc-900 outline-none focus:border-zinc-500 dark:border-zinc-600 dark:bg-zinc-800 dark:text-zinc-100",
        name,
        r#type,
        placeholder,
        autocomplete,
      }
    }
  }
}

#[component]
pub fn AuthLink(
  #[props(into)] to: NavigationTarget,
  #[props(into)] label: String,
) -> Element {
  rsx! {
    Link {
      class: "font-medium text-zinc-900 underline-offset-4 hover:underline dark:text-zinc-100",
      to,
      "{label}"
    }
  }
}