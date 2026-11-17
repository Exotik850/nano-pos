#[cfg(feature = "server")]
use crate::api::auth::Session;
use crate::models::{Credit, CreditDetails, CreditDraft};
#[cfg(feature = "server")]
use crate::models::{CreditForm, CreditProductForm};
#[cfg(feature = "server")]
use crate::server::db::DbHandle;
use dioxus::prelude::*;

#[post("/credits", db: DbHandle, _session: Session)]
pub async fn list_credits() -> Result<Vec<Credit>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Credit::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load credits: {error}")))
}

#[post("/credits/{credit_id}", db: DbHandle, _session: Session)]
pub async fn get_credit(credit_id: i32) -> Result<Option<CreditDetails>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    match Credit::find(&mut conn, credit_id) {
        Ok(credit) => Credit::products(&mut conn, credit_id)
            .map(|products| Some(CreditDetails { credit, products }))
            .map_err(|error| {
                ServerFnError::Response(format!("Could not load credit products: {error}"))
            }),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(error) => Err(ServerFnError::Response(format!(
            "Could not load credit: {error}"
        ))),
    }
}

#[post("/credits/create", db: DbHandle, session: Session)]
pub async fn create_credit(
    CreditDraft {
        mut credit,
        products,
    }: CreditDraft,
) -> Result<Credit, ServerFnError> {
    credit.user_id = session.id;
    validate_credit(&credit, &products)?;
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Credit::create(&mut conn, &credit, products)
        .map_err(|error| ServerFnError::Response(format!("Could not create credit: {error}")))
}

#[post("/credits/{credit_id}/update", db: DbHandle, session: Session)]
pub async fn update_credit(
    credit_id: i32,
    CreditDraft {
        mut credit,
        products,
    }: CreditDraft,
) -> Result<Credit, ServerFnError> {
    credit.user_id = session.id;
    validate_credit(&credit, &products)?;
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Credit::update(&mut conn, credit_id, &credit, products)
        .map_err(|error| ServerFnError::Response(format!("Could not update credit: {error}")))
}

#[post("/credits/{credit_id}/delete", db: DbHandle, _session: Session)]
pub async fn delete_credit(credit_id: i32) -> Result<(), ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Credit::delete(&mut conn, credit_id)
        .map(|_| ())
        .map_err(|error| ServerFnError::Response(format!("Could not delete credit: {error}")))
}

#[cfg(feature = "server")]
fn validate_credit(form: &CreditForm, products: &[CreditProductForm]) -> Result<(), ServerFnError> {
    if form.doc_number.trim().is_empty() {
        return Err(ServerFnError::Response(
            "Document number is required.".into(),
        ));
    }
    if products.iter().any(|product| product.quantity <= 0) {
        return Err(ServerFnError::Response(
            "Credit quantities must be greater than zero.".into(),
        ));
    }
    Ok(())
}
