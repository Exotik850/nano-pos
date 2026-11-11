use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::estimate))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Contact, foreign_key = customer_id)))]
pub struct Estimate {
    pub id: i32,
    pub customer_id: Option<i32>,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::estimate))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Contact, foreign_key = customer_id)))]
pub struct EstimateForm {
    pub customer_id: Option<i32>,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset, Associations))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::estimate_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Estimate)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
pub struct EstimateProduct {
    pub id: i32,
    pub estimate_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset, Associations))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::estimate_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Estimate)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
pub struct EstimateProductForm {
    #[serde(default)]
    pub estimate_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateDraft {
    pub estimate: EstimateForm,
    pub products: Vec<EstimateProductForm>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateDetails {
    pub estimate: Estimate,
    pub products: Vec<EstimateProduct>,
}