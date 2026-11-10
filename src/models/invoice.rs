use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::invoice))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Customer)))]
pub struct Invoice {
    pub id: i32,
    pub customer_id: i32,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub order_date: NaiveDateTime,
    pub delivery_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::invoice))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Customer)))]
pub struct InvoiceForm {
    pub customer_id: i32,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub order_date: NaiveDateTime,
    pub delivery_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable, Insertable, AsChangeset, Associations))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::invoice_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Invoice)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
#[cfg_attr(feature = "server", diesel(primary_key(invoice_id, product_id)))]
pub struct InvoiceProduct {
    pub invoice_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}
