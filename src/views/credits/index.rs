use crate::Route;
use crate::components::{
    IndexPage, Table, TableActions, TableBody, TableCell, TableHead, TableHeaderCell, TableRow,
    customer_name, format_date,
};
use crate::contacts::list_contacts;
use crate::credits::{delete_credit, list_credits};
use crate::models::{Contact, Credit};
use dioxus::prelude::*;

#[component]
fn CreditTableRow(credit: Credit, contacts: Contact, ondelete: EventHandler<i32>) -> Element {
    let customer = customer_name(&[contacts], Some(credit.customer_id));
    let credit_date = format_date(credit.credit_date);
    let id = credit.id;
    rsx! {
        TableRow {
            TableCell {
                div { class: "font-medium text-zinc-900 dark:text-zinc-100", "{credit.doc_number}" }
                div { class: "mt-1 text-sm text-zinc-500 dark:text-zinc-400", "{customer}" }
            }
            TableCell { class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{credit_date}" }
            TableCell { class: "max-w-md truncate text-zinc-600 dark:text-zinc-300",
                "{credit.description.clone().unwrap_or_else(|| \"No description\".to_string())}"
            }
            TableCell {
                TableActions {
                    edit_to: Route::EditCredit { id },
                    ondelete: move |_| ondelete.call(id),
                }
            }
        }
    }
}

#[component]
pub fn IndexCredits() -> Element {
    let mut error = use_signal(|| None::<String>);
    let credits = use_loader(list_credits)?;
    let contacts = use_loader(list_contacts)?;
    let credits_ready = credits();
    let contacts_ready = contacts();
    let mut credits = use_signal(|| credits_ready.clone());
    if let Some(message) = error() {
        return Err(ServerFnError::Response(message).into());
    }
    rsx! {
        IndexPage {
            title: "Credits",
            description: "Create and manage customer credits.",
            empty_title: "No credits yet",
            empty_description: "Create your first credit to record a customer credit.",
            is_empty: credits.is_empty(),
            action: rsx! {
                crate::components::ButtonLink { to: Route::NewCredit {}, "Add credit" }
            },
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
                            contacts: contacts_ready
                                .iter()
                                .find(|c| c.id == credit.customer_id)
                                .cloned()
                                .unwrap_or_default(),
                            credit,
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
