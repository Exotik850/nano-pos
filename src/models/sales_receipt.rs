use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::sales_receipt))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Contact, foreign_key = customer_id)))]
pub struct SalesReceipt {
    pub id: i32,
    pub customer_id: i32,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub sales_receipt_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::sales_receipt))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Contact, foreign_key = customer_id)))]
pub struct SalesReceiptForm {
    pub customer_id: i32,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub sales_receipt_date: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Insertable, AsChangeset, Associations)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::sales_receipt_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(SalesReceipt)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
pub struct SalesReceiptProduct {
    pub id: i32,
    pub sales_receipt_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset, Associations))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::sales_receipt_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(SalesReceipt)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
pub struct SalesReceiptProductForm {
    #[serde(default)]
    pub sales_receipt_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}

impl From<SalesReceiptProduct> for SalesReceiptProductForm {
    fn from(product: SalesReceiptProduct) -> Self {
        Self {
            sales_receipt_id: product.sales_receipt_id,
            product_id: product.product_id,
            quantity: product.quantity,
            price: product.price,
            tax_rate: product.tax_rate,
            description: product.description,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SalesReceiptDraft {
    pub sales_receipt: SalesReceiptForm,
    pub products: Vec<SalesReceiptProductForm>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SalesReceiptDetails {
    pub sales_receipt: SalesReceipt,
    pub products: Vec<SalesReceiptProduct>,
}

#[cfg(feature = "server")]
mod sales_receipt_server {
    use super::{SalesReceipt, SalesReceiptForm, SalesReceiptProduct, SalesReceiptProductForm};
    use crate::server::schema::sales_receipt::dsl::*;
    use crate::server::schema::sales_receipt_product::dsl::*;
    use diesel::prelude::*;

    impl SalesReceipt {
        pub fn all(conn: &mut diesel::PgConnection) -> diesel::QueryResult<Vec<Self>> {
            sales_receipt
                .select(Self::as_select())
                .order(sales_receipt_date.desc())
                .load(conn)
        }

        pub fn find(conn: &mut diesel::PgConnection, receipt_id: i32) -> diesel::QueryResult<Self> {
            sales_receipt
                .select(Self::as_select())
                .find(receipt_id)
                .first(conn)
        }

        pub fn products(
            conn: &mut diesel::PgConnection,
            receipt_id: i32,
        ) -> diesel::QueryResult<Vec<SalesReceiptProduct>> {
            sales_receipt_product
                .filter(sales_receipt_id.eq(receipt_id))
                .order(product_id.asc())
                .load(conn)
        }

        pub fn create(
            conn: &mut diesel::PgConnection,
            form: &SalesReceiptForm,
            products: Vec<SalesReceiptProductForm>,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                let new_id = diesel::insert_into(sales_receipt)
                    .values(form)
                    .returning(crate::server::schema::sales_receipt::id)
                    .get_result::<i32>(conn)?;
                if !products.is_empty() {
                    diesel::insert_into(sales_receipt_product)
                        .values(
                            products
                                .into_iter()
                                .map(|mut product| {
                                    product.sales_receipt_id = new_id;
                                    product
                                })
                                .collect::<Vec<_>>(),
                        )
                        .execute(conn)?;
                }
                Self::find(conn, new_id)
            })
        }

        pub fn update(
            conn: &mut diesel::PgConnection,
            receipt_id: i32,
            form: &SalesReceiptForm,
            products: Vec<SalesReceiptProductForm>,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                diesel::update(sales_receipt.find(receipt_id))
                    .set((form, updated_at.eq(diesel::dsl::now)))
                    .execute(conn)?;
                diesel::delete(sales_receipt_product.filter(sales_receipt_id.eq(receipt_id)))
                    .execute(conn)?;
                if !products.is_empty() {
                    diesel::insert_into(sales_receipt_product)
                        .values(
                            products
                                .into_iter()
                                .map(|mut product| {
                                    product.sales_receipt_id = receipt_id;
                                    product
                                })
                                .collect::<Vec<_>>(),
                        )
                        .execute(conn)?;
                }
                Self::find(conn, receipt_id)
            })
        }

        pub fn delete(
            conn: &mut diesel::PgConnection,
            receipt_id: i32,
        ) -> diesel::QueryResult<usize> {
            conn.transaction(|conn| {
                diesel::delete(sales_receipt_product.filter(sales_receipt_id.eq(receipt_id)))
                    .execute(conn)?;
                diesel::delete(sales_receipt.find(receipt_id)).execute(conn)
            })
        }
    }
}
