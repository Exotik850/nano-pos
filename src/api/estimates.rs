#[cfg(feature = "server")]
use crate::api::auth::Session;
use crate::models::{Estimate, EstimateDetails, EstimateDraft, EstimateProduct};
#[cfg(feature = "server")]
use crate::models::{EstimateForm, EstimateProductForm};
#[cfg(feature = "server")]
use crate::server::db::DbHandle;
use dioxus::prelude::*;

#[server(db: DbHandle, _session: Session)]
pub async fn list_estimates() -> Result<Vec<Estimate>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Estimate::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load estimates: {error}")))
}

#[server(db: DbHandle, _session: Session)]
pub async fn get_estimate(estimate_id: i32) -> Result<Option<EstimateDetails>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    match Estimate::find(&mut conn, estimate_id) {
        Ok(estimate) => Estimate::products(&mut conn, estimate_id)
            .map(|products| Some(EstimateDetails { estimate, products }))
            .map_err(|error| {
                ServerFnError::Response(format!("Could not load estimate products: {error}"))
            }),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(error) => Err(ServerFnError::Response(format!(
            "Could not load estimate: {error}"
        ))),
    }
}

#[server(db: DbHandle, session: Session)]
pub async fn create_estimate(
    EstimateDraft {
        mut estimate,
        products,
    }: EstimateDraft,
) -> Result<Estimate, ServerFnError> {
    estimate.user_id = session.id;
    validate_estimate(&estimate, &products)?;
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Estimate::create(&mut conn, &estimate, products)
        .map_err(|error| ServerFnError::Response(format!("Could not create estimate: {error}")))
}

#[server(db: DbHandle, session: Session)]
pub async fn update_estimate(
    estimate_id: i32,
    EstimateDraft {
        mut estimate,
        products,
    }: EstimateDraft,
) -> Result<Estimate, ServerFnError> {
    estimate.user_id = session.id;
    validate_estimate(&estimate, &products)?;
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Estimate::update(&mut conn, estimate_id, &estimate, products)
        .map_err(|error| ServerFnError::Response(format!("Could not update estimate: {error}")))
}

#[server(db: DbHandle, _session: Session)]
pub async fn delete_estimate(estimate_id: i32) -> Result<(), ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    Estimate::delete(&mut conn, estimate_id)
        .map(|_| ())
        .map_err(|error| ServerFnError::Response(format!("Could not delete estimate: {error}")))
}

#[cfg(feature = "server")]
fn validate_estimate(
    form: &EstimateForm,
    products: &[EstimateProductForm],
) -> Result<(), ServerFnError> {
    if form.doc_number.trim().is_empty() {
        return Err(ServerFnError::Response(
            "Document number is required.".into(),
        ));
    }
    if products.iter().any(|product| product.quantity <= 0) {
        return Err(ServerFnError::Response(
            "Estimate quantities must be greater than zero.".into(),
        ));
    }
    Ok(())
}
