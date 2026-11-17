//! Small formatting helpers shared across views and form components.

use chrono::NaiveDateTime;

use crate::models::Contact;

/// Resolve a customer id to a display name using the loaded contacts.
pub fn customer_name(contacts: &[Contact], customer_id: Option<i32>) -> String {
    let Some(customer_id) = customer_id else {
        return "No customer".to_string();
    };
    contacts
        .iter()
        .find(|contact| contact.id == customer_id)
        .map_or_else(
            || format!("Customer #{customer_id}"),
            |contact| format!("{} {}", contact.first_name, contact.last_name),
        )
}

/// Format a price stored in cents as a dollar string.
pub fn format_price(cents: i32) -> String {
    format!("${:.2}", f64::from(cents) / 100.0)
}

/// Format a date as `YYYY-MM-DD`.
pub fn format_date(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%d").to_string()
}

/// Format a datetime for a `datetime-local` input.
pub fn datetime_input(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%dT%H:%M").to_string()
}
