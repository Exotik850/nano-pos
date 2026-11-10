use dioxus::{fullstack::Form, prelude::*};

use crate::components::InvoiceDraft;
#[cfg(feature = "server")]
use crate::models::InvoiceForm;
use crate::models::{Invoice, InvoiceDetails, InvoiceProduct};

#[cfg(feature = "server")]
fn require_invoice_session(
    headers: &dioxus::fullstack::HeaderMap,
) -> Result<crate::auth::Session, ServerFnError> {
    crate::server::auth::current_user_impl(headers)?
        .ok_or_else(|| ServerFnError::Response("You must be signed in to manage invoices.".into()))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn list_invoices() -> Result<Vec<Invoice>, ServerFnError> {
    require_invoice_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    Invoice::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load invoices: {error}")))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn get_invoice(invoice_id: i32) -> Result<Option<InvoiceDetails>, ServerFnError> {
    require_invoice_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    match Invoice::find(&mut conn, invoice_id) {
        Ok(invoice) => Invoice::products(&mut conn, invoice_id)
            .map(|products| Some(InvoiceDetails { invoice, products }))
            .map_err(|error| {
                ServerFnError::Response(format!("Could not load invoice products: {error}"))
            }),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(error) => Err(ServerFnError::Response(format!(
            "Could not load invoice: {error}"
        ))),
    }
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn create_invoice(
    InvoiceDraft { mut form, products }: InvoiceDraft,
) -> Result<Invoice, ServerFnError> {
    let session = require_invoice_session(&cookies)?;
    form.user_id = session.id;
    validate_invoice(&form, &products)?;
    let mut conn = crate::server::db::establish_connection();
    let invoice = Invoice::create(&mut conn, &form, &products)
        .map_err(|error| ServerFnError::Response(format!("Could not create invoice: {error}")))?;
    Ok(invoice)
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn update_invoice(
    invoice_id: i32,
    InvoiceDraft {
        mut form,
        mut products,
    }: InvoiceDraft,
) -> Result<Invoice, ServerFnError> {
    let session = require_invoice_session(&cookies)?;
    form.user_id = session.id;
    validate_invoice(&form, &products)?;
    products
        .iter_mut()
        .for_each(|product| product.invoice_id = invoice_id);
    let mut conn = crate::server::db::establish_connection();
    Invoice::update(&mut conn, invoice_id, &form, &products)
        .map_err(|error| ServerFnError::Response(format!("Could not update invoice: {error}")))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn delete_invoice(invoice_id: i32) -> Result<(), ServerFnError> {
    require_invoice_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    Invoice::delete(&mut conn, invoice_id)
        .map(|_| ())
        .map_err(|error| ServerFnError::Response(format!("Could not delete invoice: {error}")))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn list_invoice_products(invoice_id: i32) -> Result<Vec<InvoiceProduct>, ServerFnError> {
    require_invoice_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    Invoice::products(&mut conn, invoice_id).map_err(|error| {
        ServerFnError::Response(format!("Could not load invoice products: {error}"))
    })
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn create_invoice_product(
    form: Form<InvoiceProduct>,
) -> Result<InvoiceProduct, ServerFnError> {
    require_invoice_session(&cookies)?;
    let product = form.0;
    let mut conn = crate::server::db::establish_connection();
    InvoiceProduct::create(&mut conn, &product)
        .map_err(|error| ServerFnError::Response(format!("Could not add invoice product: {error}")))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn update_invoice_product(form: InvoiceProduct) -> Result<InvoiceProduct, ServerFnError> {
    require_invoice_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    InvoiceProduct::update(&mut conn, form.id, &form).map_err(|error| {
        ServerFnError::Response(format!("Could not update invoice product: {error}"))
    })
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn delete_invoice_product(id: i32) -> Result<(), ServerFnError> {
    require_invoice_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    InvoiceProduct::delete(&mut conn, id)
        .map(|_| ())
        .map_err(|error| {
            ServerFnError::Response(format!("Could not delete invoice product: {error}"))
        })
}

#[cfg(feature = "server")]
fn validate_invoice(form: &InvoiceForm, products: &[InvoiceProduct]) -> Result<(), ServerFnError> {
    use chrono::NaiveDateTime;
    if form.doc_number.trim().is_empty() {
        return Err(ServerFnError::Response(
            "Document number is required.".into(),
        ));
    }
    if form.order_date > form.delivery_date.unwrap_or(NaiveDateTime::MAX) {
        return Err(ServerFnError::Response(
            "Delivery date cannot be before the order date.".into(),
        ));
    }
    if products.iter().any(|product| product.quantity <= 0) {
        return Err(ServerFnError::Response(
            "Invoice quantities must be greater than zero.".into(),
        ));
    }
    Ok(())
}
