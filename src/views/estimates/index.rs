use crate::Route;
use crate::components::{
    Button, ButtonLink, ButtonVariant, Table, TableBody, TableCell, TableHead, TableHeaderCell,
    TableRow,
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
    let customer_name = estimate.customer_id.map_or("No customer".to_string(), |customer_id| {
        contacts
            .iter()
            .find(|contact| contact.id == customer_id)
            .map_or_else(
                || format!("Customer #{customer_id}"),
                |contact| format!("{} {}", contact.first_name, contact.last_name),
            )
    });
    let created = estimate.created_at.format("%Y-%m-%d").to_string();
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
                div { class: "flex justify-end gap-2",
                    ButtonLink {
                        class: "px-3 py-1.5 text-sm",
                        to: Route::EditEstimate {
                            id: estimate.id,
                        },
                        variant: ButtonVariant::Secondary,
                        "Edit"
                    }
                    Button {
                        class: "px-3 py-1.5 text-sm",
                        variant: ButtonVariant::Secondary,
                        r#type: "button",
                        onclick: move |_| ondelete.call(estimate.id),
                        "Delete"
                    }
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
        div { class: "mx-auto flex w-full max-w-6xl flex-col gap-8 px-6 py-8",
            div { class: "flex flex-wrap items-end justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Estimates"
                    }
                    p { class: "mt-1 text-zinc-500 dark:text-zinc-400",
                        "Create and manage customer estimates."
                    }
                }
                ButtonLink { to: Route::NewEstimate {}, "Add estimate" }
            }
            if estimates.is_empty() {
                div { class: "rounded-xl border border-dashed border-zinc-300 px-6 py-16 text-center dark:border-zinc-700",
                    h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100",
                        "No estimates yet"
                    }
                    p { class: "mt-2 text-sm text-zinc-500 dark:text-zinc-400",
                        "Create your first estimate to start quoting customers."
                    }
                }
            } else {
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
}
