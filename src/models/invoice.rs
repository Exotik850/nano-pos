use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::invoice))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Contact, foreign_key = customer_id)))]
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
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Contact, foreign_key = customer_id)))]
pub struct InvoiceForm {
    pub customer_id: i32,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub order_date: NaiveDateTime,
    pub delivery_date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Insertable, AsChangeset, Associations)
)]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::invoice_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Invoice)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
pub struct InvoiceProduct {
    pub id: i32,
    pub invoice_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset, Associations))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::invoice_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Invoice)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
pub struct InvoiceProductForm {
    pub invoice_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}

impl From<InvoiceProduct> for InvoiceProductForm {
    fn from(product: InvoiceProduct) -> Self {
        Self {
            invoice_id: product.invoice_id,
            product_id: product.product_id,
            quantity: product.quantity,
            price: product.price,
            tax_rate: product.tax_rate,
            description: product.description,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature="server", diesel(table_name = crate::server::schema::invoice))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct InvoiceSearchResult {
    pub id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub order_date: NaiveDateTime,
    pub delivery_date: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InvoiceDraft {
    pub form: InvoiceForm,
    pub products: Vec<InvoiceProductForm>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InvoiceDetails {
    pub invoice: Invoice,
    pub products: Vec<InvoiceProduct>,
}

#[cfg(feature = "server")]
mod invoice_server {
    use super::{Invoice, InvoiceForm, InvoiceProduct, InvoiceProductForm};
    use crate::server::schema::invoice::dsl::*;
    use crate::server::schema::invoice_product::dsl::*;
    use diesel::prelude::*;

    impl Invoice {
        pub fn all(conn: &mut diesel::PgConnection) -> diesel::QueryResult<Vec<Self>> {
            invoice
                .select(Self::as_select())
                .order(order_date.desc())
                .load(conn)
        }

        pub fn find(conn: &mut diesel::PgConnection, inv_id: i32) -> diesel::QueryResult<Self> {
            invoice.select(Self::as_select()).find(inv_id).first(conn)
        }

        pub fn products(
            conn: &mut diesel::PgConnection,
            inv_id: i32,
        ) -> diesel::QueryResult<Vec<InvoiceProduct>> {
            invoice_product
                .filter(invoice_id.eq(inv_id))
                .order(product_id.asc())
                .load(conn)
        }

        pub fn create(
            conn: &mut diesel::PgConnection,
            form: &InvoiceForm,
            products: Vec<InvoiceProductForm>,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                let inv_id = diesel::insert_into(invoice)
                    .values(form)
                    .returning(crate::server::schema::invoice::id)
                    .get_result::<i32>(conn)?;
                if !products.is_empty() {
                    diesel::insert_into(invoice_product)
                        .values(
                            products
                                .into_iter()
                                .map(|mut p| {
                                    p.invoice_id = inv_id;
                                    p
                                })
                                .collect::<Vec<_>>(),
                        )
                        .execute(conn)?;
                }
                Self::find(conn, inv_id)
            })
        }

        pub fn update(
            conn: &mut diesel::PgConnection,
            inv_id: i32,
            form: &InvoiceForm,
            products: Vec<InvoiceProductForm>,
        ) -> diesel::QueryResult<Self> {
            use crate::server::schema::invoice::dsl::*;
            use crate::server::schema::invoice_product::dsl::*;
            use diesel::prelude::*;

            conn.transaction(|conn| {
                diesel::update(invoice.find(inv_id))
                    .set((form, updated_at.eq(diesel::dsl::now)))
                    .execute(conn)?;
                diesel::delete(invoice_product.filter(invoice_id.eq(inv_id))).execute(conn)?;
                if !products.is_empty() {
                    diesel::insert_into(invoice_product)
                        .values(
                            products
                                .into_iter()
                                .map(|mut p| {
                                    p.invoice_id = inv_id;
                                    p
                                })
                                .collect::<Vec<_>>(),
                        )
                        .execute(conn)?;
                }
                Self::find(conn, inv_id)
            })
        }

        pub fn delete(conn: &mut diesel::PgConnection, inv_id: i32) -> diesel::QueryResult<usize> {
            conn.transaction(|conn| {
                diesel::delete(invoice_product.filter(invoice_id.eq(inv_id))).execute(conn)?;
                diesel::delete(invoice.find(inv_id)).execute(conn)
            })
        }
    }
}

#[cfg(feature = "server")]
mod invoice_product_server {
    use super::InvoiceProduct;
    use crate::server::schema::invoice_product::dsl::*;
    use diesel::prelude::*;
    impl InvoiceProduct {
        pub fn find(conn: &mut diesel::PgConnection, id_: i32) -> diesel::QueryResult<Self> {
            invoice_product.find(id_).first(conn)
        }

        pub fn create(conn: &mut diesel::PgConnection, form: &Self) -> diesel::QueryResult<Self> {
            diesel::insert_into(invoice_product)
                .values(form)
                .get_result(conn)
        }

        pub fn update(
            conn: &mut diesel::PgConnection,
            id_: i32,
            form: &Self,
        ) -> diesel::QueryResult<Self> {
            diesel::update(invoice_product.find(id_))
                .set(form)
                .get_result(conn)
        }

        pub fn delete(conn: &mut diesel::PgConnection, id_: i32) -> diesel::QueryResult<usize> {
            diesel::delete(invoice_product.find(id_)).execute(conn)
        }
    }
}

#[cfg(feature = "server")]
mod invoice_search {
    use crate::{api::search::SearchResult, server::search::SearchFilter};

    use super::InvoiceSearchResult;
    use diesel::prelude::*;
    use dioxus::server::inventory;

    impl From<InvoiceSearchResult> for SearchResult {
        fn from(value: InvoiceSearchResult) -> Self {
            SearchResult {
                route: crate::Route::EditInvoice { id: value.id },
                title: format!("Invoice #{}", value.doc_number),
                description: value.description.unwrap_or_default(),
            }
        }
    }

    inventory::submit! {
       SearchFilter::new(
         |query, conn| {
             use crate::server::schema::invoice::dsl::*;
             let pattern = format!("%{}%", query);
             invoice
                 .filter(
                     doc_number.ilike(&pattern)
                         .or(description.ilike(&pattern)),
                 )
                 .select(InvoiceSearchResult::as_select())
                 .load::<InvoiceSearchResult>(conn)
                 .map(|results| {
                     results.into_iter().map(|inv| {
                         SearchResult::from(inv)
                     }).collect::<Vec<_>>()
                 })
         }
       )
    }
}
