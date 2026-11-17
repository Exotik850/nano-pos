use crate::Route;
use crate::components::{
    Button, ButtonLink, ButtonVariant, Table, TableBody, TableCell, TableHead, TableHeaderCell,
    TableRow,
};
use crate::contacts::list_contacts;
use crate::credits::{delete_credit, list_credits};
use crate::models::{Contact, Credit};
use dioxus::prelude::*;

#[component]
fn CreditTableRow(credit: Credit, contacts: Vec<Contact>, ondelete: EventHandler<i32>) -> Element {
    let customer = contacts
        .iter()
        .find(|contact| contact.id == credit.customer_id)
        .map_or_else(
            || format!("Customer #{}", credit.customer_id),
            |contact| format!("{} {}", contact.first_name, contact.last_name),
        );
    rsx! {
        TableRow {
            TableCell {
                div { class: "font-medium text-zinc-900 dark:text-zinc-100", "{credit.doc_number}" }
                div { class: "mt-1 text-sm text-zinc-500 dark:text-zinc-400", "{customer}" }
            }
            TableCell { class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300",
                "{credit.credit_date.format(\"%Y-%m-%d\")}"
            }
            TableCell { class: "max-w-md truncate text-zinc-600 dark:text-zinc-300",
                "{credit.description.clone().unwrap_or_else(|| \"No description\".to_string())}"
            }
            TableCell {
                div { class: "flex justify-end gap-2",
                    ButtonLink {
                        class: "px-3 py-1.5 text-sm",
                        to: Route::EditCredit { id: credit.id },
                        variant: ButtonVariant::Secondary,
                        "Edit"
                    }
                    Button {
                        class: "px-3 py-1.5 text-sm",
                        variant: ButtonVariant::Secondary,
                        r#type: "button",
                        onclick: move |_| ondelete.call(credit.id),
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
pub fn IndexCredits() -> Element {
    let mut error = use_signal(|| None::<String>);
    let credits = use_server_future(list_credits)?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let credits_ready = credits().context("Failed to get credits")?;
    let contacts_ready = contacts().context("Failed to get contacts")?;
    let mut credits = use_signal(|| credits_ready.clone());
    if let Some(message) = error() {
        return Err(ServerFnError::Response(message).into());
    }
    rsx! {
        div { class: "mx-auto flex w-full max-w-6xl flex-col gap-8 px-6 py-8",
            div { class: "flex flex-wrap items-end justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Credits"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Create and manage customer credits."
                    }
                }
                ButtonLink { to: Route::NewCredit {}, "Add credit" }
            }
            if credits.is_empty() {
                div { class: "rounded-xl border border-dashed border-zinc-300 px-6 py-16 text-center dark:border-zinc-700",
                    h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100",
                        "No credits yet"
                    }
                    p { class: "mt-2 text-sm text-zinc-500 dark:text-zinc-400",
                        "Create your first credit to record a customer credit."
                    }
                }
            } else {
                Table {
                    TableHead {
                        tr {
                            TableHeaderCell { "Credit" }
                            TableHeaderCell { "Date" }
                            TableHeaderCell { "Description" }
                            TableHeaderCell { align_end: true, "Actions" }
                        }
                    }
                    TableBody {
                        for credit in credits() {
                            CreditTableRow {
                                credit,
                                contacts: contacts_ready.clone(),
                                ondelete: move |credit_id| async move {
                                    if let Err(message) = delete_credit(credit_id).await {
                                        error.set(Some(message.to_string()));
                                    } else {
                                        credits.retain(|value| value.id != credit_id);
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
