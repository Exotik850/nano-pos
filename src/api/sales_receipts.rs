#[cfg(feature = "server")]
use crate::api::auth::Session;
use crate::models::{SalesReceipt, SalesReceiptDetails, SalesReceiptDraft};
#[cfg(feature = "server")]
use crate::models::{SalesReceiptForm, SalesReceiptProductForm};
#[cfg(feature = "server")]
use crate::server::db::DbHandle;
use dioxus::prelude::*;

#[post("/sales-receipts", db: DbHandle, _session: Session)]
pub async fn list_sales_receipts() -> Result<Vec<SalesReceipt>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    SalesReceipt::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load sales receipts: {error}")))
}

#[post("/sales-receipts/{receipt_id}", db: DbHandle, _session: Session)]
pub async fn get_sales_receipt(
    receipt_id: i32,
) -> Result<Option<SalesReceiptDetails>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    match SalesReceipt::find(&mut conn, receipt_id) {
        Ok(sales_receipt) => SalesReceipt::products(&mut conn, receipt_id)
            .map(|products| {
                Some(SalesReceiptDetails {
                    sales_receipt,
                    products,
                })
            })
            .map_err(|error| {
                ServerFnError::Response(format!("Could not load sales receipt products: {error}"))
            }),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(error) => Err(ServerFnError::Response(format!(
            "Could not load sales receipt: {error}"
        ))),
    }
}

#[post("/sales-receipts/create", db: DbHandle, session: Session)]
pub async fn create_sales_receipt(
    SalesReceiptDraft {
        mut sales_receipt,
        products,
    }: SalesReceiptDraft,
) -> Result<SalesReceipt, ServerFnError> {
    sales_receipt.user_id = session.id;
    validate_sales_receipt(&sales_receipt, &products)?;
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    SalesReceipt::create(&mut conn, &sales_receipt, products).map_err(|error| {
        ServerFnError::Response(format!("Could not create sales receipt: {error}"))
    })
}

#[post("/sales-receipts/{receipt_id}/update", db: DbHandle, session: Session)]
pub async fn update_sales_receipt(
    receipt_id: i32,
    SalesReceiptDraft {
        mut sales_receipt,
        products,
    }: SalesReceiptDraft,
) -> Result<SalesReceipt, ServerFnError> {
    sales_receipt.user_id = session.id;
    validate_sales_receipt(&sales_receipt, &products)?;
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    SalesReceipt::update(&mut conn, receipt_id, &sales_receipt, products).map_err(|error| {
        ServerFnError::Response(format!("Could not update sales receipt: {error}"))
    })
}

#[post("/sales-receipts/{receipt_id}/delete", db: DbHandle, _session: Session)]
pub async fn delete_sales_receipt(receipt_id: i32) -> Result<(), ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    SalesReceipt::delete(&mut conn, receipt_id)
        .map(|_| ())
        .map_err(|error| {
            ServerFnError::Response(format!("Could not delete sales receipt: {error}"))
        })
}

#[cfg(feature = "server")]
fn validate_sales_receipt(
    form: &SalesReceiptForm,
    products: &[SalesReceiptProductForm],
) -> Result<(), ServerFnError> {
    if form.doc_number.trim().is_empty() {
        return Err(ServerFnError::Response(
            "Document number is required.".into(),
        ));
    }
    if products.iter().any(|product| product.quantity <= 0) {
        return Err(ServerFnError::Response(
            "Sales receipt quantities must be greater than zero.".into(),
        ));
    }
    Ok(())
}
