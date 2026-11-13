use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::api::auth::Session;
use crate::models::{Invoice, InvoiceDetails, InvoiceDraft};
#[cfg(feature = "server")]
use crate::models::{InvoiceForm, InvoiceProductForm};
#[cfg(feature = "server")]
use crate::server::db::DbHandle;

#[server(db: DbHandle, _session: Session)]
pub async fn list_invoices() -> Result<Vec<Invoice>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Invoice::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load invoices: {error}")))
}

#[server(db: DbHandle, _session: Session)]
pub async fn get_invoice(invoice_id: i32) -> Result<Option<InvoiceDetails>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
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

#[server(db: DbHandle, session: Session)]
pub async fn create_invoice(
    InvoiceDraft { mut form, products }: InvoiceDraft,
) -> Result<Invoice, ServerFnError> {
    form.user_id = session.id;
    validate_invoice(&form, &products)?;
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    let invoice = Invoice::create(&mut conn, &form, products)
        .map_err(|error| ServerFnError::Response(format!("Could not create invoice: {error}")))?;
    Ok(invoice)
}

#[server(db: DbHandle, session: Session)]
pub async fn update_invoice(
    invoice_id: i32,
    InvoiceDraft { mut form, products }: InvoiceDraft,
) -> Result<Invoice, ServerFnError> {
    form.user_id = session.id;
    validate_invoice(&form, &products)?;
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Invoice::update(&mut conn, invoice_id, &form, products)
        .map_err(|error| ServerFnError::Response(format!("Could not update invoice: {error}")))
}

#[server(db: DbHandle, _session: Session)]
pub async fn delete_invoice(invoice_id: i32) -> Result<(), ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Invoice::delete(&mut conn, invoice_id)
        .map(|_| ())
        .map_err(|error| ServerFnError::Response(format!("Could not delete invoice: {error}")))
}

#[cfg(feature = "server")]
fn validate_invoice(
    form: &InvoiceForm,
    products: &[InvoiceProductForm],
) -> Result<(), ServerFnError> {
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
