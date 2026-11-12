use dioxus::{fullstack::Form, prelude::*};

#[cfg(feature = "server")]
use crate::api::auth::Session;
use crate::models::{Invoice, InvoiceDetails, InvoiceDraft, InvoiceProduct};
#[cfg(feature = "server")]
use crate::models::{InvoiceForm, InvoiceProductForm};

#[server(_session: Session)]
pub async fn list_invoices() -> Result<Vec<Invoice>, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Invoice::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load invoices: {error}")))
}

#[server(_session: Session)]
pub async fn get_invoice(invoice_id: i32) -> Result<Option<InvoiceDetails>, ServerFnError> {
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

#[server(session: Session)]
pub async fn create_invoice(
    InvoiceDraft { mut form, products }: InvoiceDraft,
) -> Result<Invoice, ServerFnError> {
    form.user_id = session.id;
    validate_invoice(&form, &products)?;
    let mut conn = crate::server::db::establish_connection();
    let invoice = Invoice::create(&mut conn, &form, products)
        .map_err(|error| ServerFnError::Response(format!("Could not create invoice: {error}")))?;
    Ok(invoice)
}

#[server(session: Session)]
pub async fn update_invoice(
    invoice_id: i32,
    InvoiceDraft { mut form, products }: InvoiceDraft,
) -> Result<Invoice, ServerFnError> {
    form.user_id = session.id;
    validate_invoice(&form, &products)?;
    let mut conn = crate::server::db::establish_connection();
    Invoice::update(&mut conn, invoice_id, &form, products)
        .map_err(|error| ServerFnError::Response(format!("Could not update invoice: {error}")))
}

#[server(_session: Session)]
pub async fn delete_invoice(invoice_id: i32) -> Result<(), ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Invoice::delete(&mut conn, invoice_id)
        .map(|_| ())
        .map_err(|error| ServerFnError::Response(format!("Could not delete invoice: {error}")))
}

#[server(_session: Session)]
pub async fn list_invoice_products(invoice_id: i32) -> Result<Vec<InvoiceProduct>, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Invoice::products(&mut conn, invoice_id).map_err(|error| {
        ServerFnError::Response(format!("Could not load invoice products: {error}"))
    })
}

#[server(_session: Session)]
pub async fn create_invoice_product(
    form: Form<InvoiceProduct>,
) -> Result<InvoiceProduct, ServerFnError> {
    let product = form.0;
    let mut conn = crate::server::db::establish_connection();
    InvoiceProduct::create(&mut conn, &product)
        .map_err(|error| ServerFnError::Response(format!("Could not add invoice product: {error}")))
}

#[server(_session: Session)]
pub async fn update_invoice_product(form: InvoiceProduct) -> Result<InvoiceProduct, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    InvoiceProduct::update(&mut conn, form.id, &form).map_err(|error| {
        ServerFnError::Response(format!("Could not update invoice product: {error}"))
    })
}

#[server(_session: Session)]
pub async fn delete_invoice_product(id: i32) -> Result<(), ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    InvoiceProduct::delete(&mut conn, id)
        .map(|_| ())
        .map_err(|error| {
            ServerFnError::Response(format!("Could not delete invoice product: {error}"))
        })
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
