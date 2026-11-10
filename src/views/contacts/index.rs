use dioxus::prelude::*;

use crate::Route;
use crate::components::{
    Button, ButtonLink, ButtonVariant, Table, TableBody, TableCell, TableHead, TableHeaderCell,
    TableRow,
};
use crate::contacts::{delete_contact, list_contacts};
use crate::models::Contact;

#[component]
fn ContactTableRow(contact: Contact, ondelete: EventHandler<i32>) -> Element {
    let message = contact
        .message
        .clone()
        .unwrap_or_else(|| "No message".to_string());
    rsx! {
        TableRow {
            TableCell {
                div { class: "font-medium text-zinc-900 dark:text-zinc-100",
                    "{contact.first_name} {contact.last_name}"
                }
                div { class: "mt-1 max-w-xs truncate text-sm text-zinc-500 dark:text-zinc-400",
                    "{message}"
                }
            }
            TableCell { class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{contact.email}" }
            TableCell { class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{contact.phone_number}" }
            TableCell {
                div { class: "flex justify-end gap-2",
                    ButtonLink {
                        class: "px-3 py-1.5 text-sm",
                        to: Route::EditContact {
                            id: contact.id,
                        },
                        variant: ButtonVariant::Secondary,
                        "Edit"
                    }
                    Button {
                        class: "px-3 py-1.5 text-sm",
                        variant: ButtonVariant::Secondary,
                        r#type: "button",
                        onclick: move |_| ondelete.call(contact.id),
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
fn ContactTable(contacts: ReadSignal<Vec<Contact>>, ondelete: EventHandler<i32>) -> Element {
    rsx! {
        Table {
            TableHead {
                tr {
                    TableHeaderCell { "Contact" }
                    TableHeaderCell { "Email" }
                    TableHeaderCell { "Phone" }
                    TableHeaderCell { align_end: true, "Actions" }
                }
            }
            TableBody {
                for contact in contacts() {
                    ContactTableRow { contact, ondelete }
                }
            }
        }
    }
}

#[component]
pub fn IndexContacts() -> Element {
    let mut error = use_signal(|| None::<String>);
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let contacts_ready = contacts().context("Failed to get contacts")?;
    let mut contacts = use_signal(|| contacts_ready.clone());

    if let Some(message) = error() {
        return Err(ServerFnError::Response(message).into());
    }

    rsx! {
        div { class: "mx-auto flex w-full max-w-6xl flex-col gap-8 px-6 py-8",
            div { class: "flex flex-wrap items-end justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Contacts"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Manage your customer contacts."
                    }
                }
                ButtonLink { to: Route::NewContact {}, "Add contact" }
            }
            if contacts.is_empty() {
                div { class: "rounded-xl border border-dashed border-zinc-300 px-6 py-16 text-center dark:border-zinc-700",
                    h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100",
                        "No contacts yet"
                    }
                    p { class: "mt-2 text-sm text-zinc-500 dark:text-zinc-400",
                        "Add your first contact to start building your customer list."
                    }
                }
            } else {
                ContactTable {
                    contacts,
                    ondelete: move |contact_id| async move {
                        if let Err(message) = delete_contact(contact_id).await {
                            warn!("Failed to delete contact: {}", message);
                            error.set(Some(message.to_string()));
                        } else {
                            contacts.retain(|contact| contact.id != contact_id);
                        }
                    },
                }
            }
        }
    }
}
