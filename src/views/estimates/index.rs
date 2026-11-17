use crate::Route;
use crate::components::{
    IndexPage, Table, TableActions, TableBody, TableCell, TableHead, TableHeaderCell, TableRow,
    customer_name, format_date,
};
use crate::contacts::list_contacts;
use crate::estimates::{delete_estimate, list_estimates};
use crate::models::{Contact, Estimate};
use dioxus::prelude::*;

#[component]
fn EstimateTableRow(
    estimate: Estimate,
    contacts: Vec<Contact>,
    ondelete: EventHandler<i32>,
) -> Element {
    let customer_name = customer_name(&contacts, estimate.customer_id);
    let created = format_date(estimate.created_at);
    let id = estimate.id;
    rsx! {
        TableRow {
            TableCell {
                div { class: "font-medium text-zinc-900 dark:text-zinc-100", "{estimate.doc_number}" }
                div { class: "mt-1 text-sm text-zinc-500 dark:text-zinc-400", "{customer_name}" }
            }
            TableCell { class: "whitespace-nowrap text-zinc-600 dark:text-zinc-300", "{created}" }
            TableCell { class: "max-w-md truncate text-zinc-600 dark:text-zinc-300",
                "{estimate.description.clone().unwrap_or_else(|| \"No description\".to_string())}"
            }
            TableCell {
                TableActions {
                    edit_to: Route::EditEstimate { id },
                    ondelete: move |_| ondelete.call(id),
                }
            }
        }
    }
}

#[component]
pub fn IndexEstimates() -> Element {
    let mut error = use_signal(|| None::<String>);
    let estimates = use_server_future(list_estimates)?.suspend()?;
    let contacts = use_server_future(list_contacts)?.suspend()?;
    let estimates_ready = estimates().context("Failed to get estimates")?;
    let contacts_ready = contacts().context("Failed to get contacts")?;
    let mut estimates = use_signal(|| estimates_ready.clone());

    if let Some(message) = error() {
        return Err(ServerFnError::Response(message).into());
    }

    rsx! {
        IndexPage {
            title: "Estimates",
            description: "Create and manage customer estimates.",
            empty_title: "No estimates yet",
            empty_description: "Create your first estimate to start quoting customers.",
            is_empty: estimates.is_empty(),
            action: rsx! {
                crate::components::ButtonLink { to: Route::NewEstimate {}, "Add estimate" }
            },
            Table {
                TableHead {
                    tr {
                        TableHeaderCell { "Estimate" }
                        TableHeaderCell { "Created" }
                        TableHeaderCell { "Description" }
                        TableHeaderCell { align_end: true, "Actions" }
                    }
                }
                TableBody {
                    for estimate in estimates() {
                        EstimateTableRow {
                            estimate,
                            contacts: contacts_ready.clone(),
                            ondelete: move |estimate_id| async move {
                                if let Err(message) = delete_estimate(estimate_id).await {
                                    error.set(Some(message.to_string()));
                                } else {
                                    estimates.retain(|estimate| estimate.id != estimate_id);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
