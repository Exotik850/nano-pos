//! Shared building blocks for the CRUD pages (index / new / edit).
//!
//! These components capture the layout and state-handling patterns that every
//! resource page repeats so the individual views only describe their data.

use dioxus::prelude::*;
use dioxus_router::Navigator;

use crate::Route;

use super::{
    Button, ButtonLink, ButtonVariant, EmptyState, NotFoundState, PageError, PageHeader, PageShell,
};

/// The standard "Edit" + "Delete" action pair shown at the end of a table row.
#[component]
pub fn TableActions(edit_to: Route, ondelete: EventHandler<()>) -> Element {
    rsx! {
        div { class: "flex justify-end gap-2",
            ButtonLink {
                class: "px-3 py-1.5 text-sm",
                to: edit_to,
                variant: ButtonVariant::Secondary,
                "Edit"
            }
            Button {
                class: "px-3 py-1.5 text-sm",
                variant: ButtonVariant::Secondary,
                r#type: "button",
                onclick: move |_| ondelete.call(()),
                "Delete"
            }
        }
    }
}

/// The outer shell for a list page: header, optional empty state, and content.
#[component]
pub fn IndexPage(
    #[props(into)] title: String,
    #[props(into)] description: String,
    #[props(default)] action: Option<Element>,
    #[props(into)] empty_title: String,
    #[props(into)] empty_description: String,
    is_empty: bool,
    children: Element,
) -> Element {
    rsx! {
        PageShell { max_width: "max-w-6xl",
            PageHeader { title, description, action }
            if is_empty {
                EmptyState { title: empty_title, description: empty_description }
            } else {
                {children}
            }
        }
    }
}

/// The outer shell for a create/edit form page: header, optional error, content.
#[component]
pub fn FormPage(
    #[props(into)] title: String,
    #[props(into)] description: String,
    #[props(default)] error: Option<String>,
    #[props(default, into)] max_width: String,
    children: Element,
) -> Element {
    rsx! {
        PageShell { max_width,
            PageHeader { title, description }
            if let Some(message) = error {
                PageError { message }
            }
            {children}
        }
    }
}

/// The outer shell for an edit page that may not find its resource.
///
/// Handles the three states of a loaded resource: a fetch error, a missing
/// resource, or the found resource whose form is passed as `children`.
#[component]
pub fn EditPage(
    #[props(into)] title: String,
    #[props(into)] description: String,
    #[props(into)] resource: String,
    back_to: Route,
    #[props(into)] back_label: String,
    #[props(default)] error: Option<String>,
    #[props(default)] load_error: Option<String>,
    #[props(default, into)] max_width: String,
    found: bool,
    children: Element,
) -> Element {
    if let Some(message) = load_error {
        return rsx! {
            PageShell { max_width,
                PageError { message }
            }
        };
    } 
    
    rsx! {
        if !found {
            NotFoundState { resource, back_to, "{back_label}" }
        } else {
            FormPage {
                title,
                description,
                error,
                max_width,
                {children}
            }
        }
    }
}

/// Run a form submission with the shared loading / error / navigation flow.
///
/// Guards against double submits, clears any previous error, and navigates to
/// `target` on success. On failure the error message is surfaced to the user.
pub async fn submit_form<F, Fut>(
    mut loading: Signal<bool>,
    mut error: Signal<Option<String>>,
    navigator: Navigator,
    target: Route,
    action: F,
) where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), String>>,
{
    if loading() {
        return;
    }
    loading.set(true);
    error.set(None);
    match action().await {
        Ok(()) => {
            navigator.replace(target);
        }
        Err(message) => {
            error.set(Some(message));
            loading.set(false);
        }
    }
}
