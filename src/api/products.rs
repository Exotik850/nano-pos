use dioxus::{fullstack::Form, prelude::*};

use crate::models::{Product, ProductForm};

#[server(_session: crate::auth::Session)]
pub async fn list_products() -> Result<Vec<Product>, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Product::all(&mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not load products: {error}")))
}

#[server(_session: crate::auth::Session)]
pub async fn get_product(product_id: i32) -> Result<Option<Product>, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    match Product::find(&mut conn, product_id) {
        Ok(product) => Ok(Some(product)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(error) => Err(ServerFnError::Response(format!(
            "Could not load product: {error}"
        ))),
    }
}

#[server(_session: crate::auth::Session)]
pub async fn create_product(form: Form<ProductForm>) -> Result<Product, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Product::create(&mut conn, &form)
        .map_err(|error| ServerFnError::Response(format!("Could not create product: {error}")))
}

#[server(_session: crate::auth::Session)]
pub async fn update_product(product_id: i32, form: ProductForm) -> Result<Product, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Product::update(&mut conn, product_id, &form)
        .map_err(|error| ServerFnError::Response(format!("Could not update product: {error}")))
}

#[server(_session: crate::auth::Session)]
pub async fn delete_product(product_id: i32) -> Result<(), ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    Product::delete(&mut conn, product_id)
        .map(|_| ())
        .map_err(|error| ServerFnError::Response(format!("Could not delete product: {error}")))
}
