use dioxus::prelude::*;

use super::Button;

/// A shared, tailwind-styled form shell for the auth pages (login / signup).
///
/// Renders a centered card containing a title, a subtitle, the fields supplied
/// through `children`, a submit button labelled `submit_label`, a red error
/// banner when `error` is set, and a row of footer links (`footer`).
#[component]
pub fn AuthForm(
    /// Card heading, e.g. "Log in" or "Sign up".
    title: String,
    /// Supporting text shown under the heading.
    subtitle: String,
    /// Label on the submit button.
    submit_label: String,
    /// Field elements (inputs / labels) rendered inside the form.
    children: Element,
    /// Validation / server error to surface in a banner, if any.
    #[props(into)]
    error: Option<String>,
    /// Footer links rendered under the submit button.
    #[props(into)]
    footer: Element,
    /// Invoked when the form is submitted.
    onsubmit: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "flex min-h-[70vh] items-center justify-center px-4 py-12",
            form {
                class: "w-full max-w-md rounded-2xl border border-zinc-200 bg-white p-8 shadow-sm dark:border-zinc-700 dark:bg-zinc-900",
                onsubmit: move |evt| {
                    // Stop the browser from performing a native form submission,
                    // which would navigate away and abort the server-fn fetch.
                    evt.prevent_default();
                    onsubmit.call(evt);
                },
                h1 { class: "text-2xl font-bold text-zinc-900 dark:text-zinc-100", "{title}" }
                p { class: "mt-1 text-sm text-zinc-500 dark:text-zinc-400", "{subtitle}" }

                div { class: "mt-6 flex flex-col gap-4", {children} }

                if let Some(message) = error {
                    p {
                        class: "mt-4 rounded-lg bg-red-50 px-3 py-2 text-sm text-red-700 dark:bg-red-950 dark:text-red-300",
                        "{message}"
                    }
                }

                Button {
                    class: "mt-6 w-full px-4 py-2.5 text-base font-semibold",
                    r#type: "submit",
                    "{submit_label}"
                }

                div { class: "mt-6 flex items-center justify-center gap-4 text-sm", {footer} }
            }
        }
    }
}
