use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::models::EstimateForm;
#[cfg(feature = "server")]
use crate::models::EstimateProductForm;
use crate::models::{Estimate, EstimateDetails, EstimateDraft, EstimateProduct};

#[cfg(feature = "server")]
fn require_estimate_session(
    headers: &dioxus::fullstack::HeaderMap,
) -> Result<crate::auth::Session, ServerFnError> {
    crate::server::auth::current_user_impl(headers)?
        .ok_or_else(|| ServerFnError::Response("You must be signed in to manage estimates.".into()))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn list_estimates() -> Result<Vec<Estimate>, ServerFnError> {
    require_estimate_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    Estimate::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load estimates: {error}")))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn get_estimate(estimate_id: i32) -> Result<Option<EstimateDetails>, ServerFnError> {
    require_estimate_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
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

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn create_estimate(
    EstimateDraft {
        mut estimate,
        products,
    }: EstimateDraft,
) -> Result<Estimate, ServerFnError> {
    let session = require_estimate_session(&cookies)?;
    estimate.user_id = session.id;
    validate_estimate(&estimate, &products)?;
    let mut conn = crate::server::db::establish_connection();
    Estimate::create(&mut conn, &estimate, products)
        .map_err(|error| ServerFnError::Response(format!("Could not create estimate: {error}")))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn update_estimate(
    estimate_id: i32,
    EstimateDraft {
        mut estimate,
        products,
    }: EstimateDraft,
) -> Result<Estimate, ServerFnError> {
    let session = require_estimate_session(&cookies)?;
    estimate.user_id = session.id;
    validate_estimate(&estimate, &products)?;
    let mut conn = crate::server::db::establish_connection();
    Estimate::update(&mut conn, estimate_id, &estimate, products)
        .map_err(|error| ServerFnError::Response(format!("Could not update estimate: {error}")))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn delete_estimate(estimate_id: i32) -> Result<(), ServerFnError> {
    require_estimate_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    Estimate::delete(&mut conn, estimate_id)
        .map(|_| ())
        .map_err(|error| ServerFnError::Response(format!("Could not delete estimate: {error}")))
}

#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn list_estimate_products(
    estimate_id: i32,
) -> Result<Vec<EstimateProduct>, ServerFnError> {
    require_estimate_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    Estimate::products(&mut conn, estimate_id).map_err(|error| {
        ServerFnError::Response(format!("Could not load estimate products: {error}"))
    })
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
