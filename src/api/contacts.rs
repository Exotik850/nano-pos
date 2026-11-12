use dioxus::{fullstack::Form, prelude::*};

use crate::models::{Contact, ContactForm};

#[server(_session: crate::auth::Session)]
pub async fn list_contacts() -> Result<Vec<Contact>, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Contact::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load contacts: {error}")))
}

#[server(_session: crate::auth::Session)]
pub async fn get_contact(contact_id: i32) -> Result<Option<Contact>, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    match Contact::find(&mut conn, contact_id) {
        Ok(contact) => Ok(Some(contact)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(error) => Err(ServerFnError::Response(format!(
            "Could not load contact: {error}"
        ))),
    }
}

#[server(_session: crate::auth::Session)]
pub async fn create_contact(form: Form<ContactForm>) -> Result<Contact, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Contact::create(&mut conn, &form)
        .map_err(|error| ServerFnError::Response(format!("Could not create contact: {error}")))
}

#[server(_session: crate::auth::Session)]
pub async fn update_contact(contact_id: i32, form: ContactForm) -> Result<Contact, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Contact::update(&mut conn, contact_id, &form)
        .map_err(|error| ServerFnError::Response(format!("Could not update contact: {error}")))
}

#[server(_session: crate::auth::Session)]
pub async fn delete_contact(contact_id: i32) -> Result<(), ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Contact::delete(&mut conn, contact_id)
        .map(|_| ())
        .map_err(|error| ServerFnError::Response(format!("Could not delete contact: {error}")))
}
