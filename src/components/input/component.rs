use dioxus::prelude::*;

use crate::components::label::Label;
#[css_module("/src/components/input/style.css")]
struct Styles;

#[component]
pub fn FormInput(
  #[props(into)] label: String,
  #[props(into)] name: String,
  #[props(default, into)] r#type: String,
  #[props(default, into)] required: bool,
  #[props(default, into)] value: Signal<String>,
  #[props(default, into)] placeholder: String,
  #[props(default, into)] autocomplete: String,
  oninput: Option<EventHandler<FormEvent>>,
  #[props(extends=GlobalAttributes)]
  #[props(extends=input)]
  attributes: Vec<Attribute>,
) -> Element {
  rsx! {
    div {
      class: "flex flex-col gap-1 w-full",
      Label {
        html_for: "{name}",
        "{label}"
      }
      input {
        class: "w-full {Styles::dx_input} ",
        id: "{name}",
        name,
        r#type,
        placeholder,
        autocomplete,
        oninput: move |event: FormEvent| {
          value.set(event.value());
        },
        required,
        ..attributes,
      }
    }
  }
}

#[component]
pub fn Input(
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    oninvalid: Option<EventHandler<FormEvent>>,
    onselect: Option<EventHandler<SelectionEvent>>,
    onselectionchange: Option<EventHandler<SelectionEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    onwheel: Option<EventHandler<WheelEvent>>,
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        input {
            class: "{Styles::dx_input} ",
            // class: "rounded-lg border border-zinc-300 bg-white px-3 py-2 text-zinc-900 outline-none focus:border-zinc-500 dark:border-zinc-600 dark:bg-zinc-800 dark:text-zinc-100",
            oninput: move |e| _ = oninput.map(|callback| callback(e)),
            onchange: move |e| _ = onchange.map(|callback| callback(e)),
            oninvalid: move |e| _ = oninvalid.map(|callback| callback(e)),
            onselect: move |e| _ = onselect.map(|callback| callback(e)),
            onselectionchange: move |e| _ = onselectionchange.map(|callback| callback(e)),
            onfocus: move |e| _ = onfocus.map(|callback| callback(e)),
            onblur: move |e| _ = onblur.map(|callback| callback(e)),
            onfocusin: move |e| _ = onfocusin.map(|callback| callback(e)),
            onfocusout: move |e| _ = onfocusout.map(|callback| callback(e)),
            onkeydown: move |e| _ = onkeydown.map(|callback| callback(e)),
            onkeypress: move |e| _ = onkeypress.map(|callback| callback(e)),
            onkeyup: move |e| _ = onkeyup.map(|callback| callback(e)),
            onwheel: move |e| _ = onwheel.map(|callback| callback(e)),
            oncompositionstart: move |e| _ = oncompositionstart.map(|callback| callback(e)),
            oncompositionupdate: move |e| _ = oncompositionupdate.map(|callback| callback(e)),
            oncompositionend: move |e| _ = oncompositionend.map(|callback| callback(e)),
            oncopy: move |e| _ = oncopy.map(|callback| callback(e)),
            oncut: move |e| _ = oncut.map(|callback| callback(e)),
            onpaste: move |e| _ = onpaste.map(|callback| callback(e)),
            ..attributes,
            {children}
        }
    }
}


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