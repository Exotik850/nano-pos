use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::credit))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Contact, foreign_key = customer_id)))]
pub struct Credit {
    pub id: i32,
    pub customer_id: i32,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub credit_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::credit))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Contact, foreign_key = customer_id)))]
pub struct CreditForm {
    pub customer_id: i32,
    pub user_id: i32,
    pub doc_number: String,
    pub description: Option<String>,
    pub credit_date: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Insertable, AsChangeset, Associations)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::credit_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Credit)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
pub struct CreditProduct {
    pub id: i32,
    pub credit_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset, Associations))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::credit_product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Credit)))]
#[cfg_attr(feature = "server", diesel(belongs_to(crate::models::Product)))]
pub struct CreditProductForm {
    #[serde(default)]
    pub credit_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Option<i32>,
    pub tax_rate: Option<f64>,
    pub description: Option<String>,
}

impl From<CreditProduct> for CreditProductForm {
    fn from(product: CreditProduct) -> Self {
        Self {
            credit_id: product.credit_id,
            product_id: product.product_id,
            quantity: product.quantity,
            price: product.price,
            tax_rate: product.tax_rate,
            description: product.description,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreditDraft {
    pub credit: CreditForm,
    pub products: Vec<CreditProductForm>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreditDetails {
    pub credit: Credit,
    pub products: Vec<CreditProduct>,
}

#[cfg(feature = "server")]
mod credit_server {
    use super::{Credit, CreditForm, CreditProduct, CreditProductForm};
    use crate::server::schema::credit::dsl::*;
    use crate::server::schema::credit_product::dsl::*;
    use diesel::prelude::*;

    impl Credit {
        pub fn all(conn: &mut diesel::PgConnection) -> diesel::QueryResult<Vec<Self>> {
            credit
                .select(Self::as_select())
                .order(credit_date.desc())
                .load(conn)
        }

        pub fn find(conn: &mut diesel::PgConnection, credit_id_: i32) -> diesel::QueryResult<Self> {
            credit
                .select(Self::as_select())
                .find(credit_id_)
                .first(conn)
        }

        pub fn products(
            conn: &mut diesel::PgConnection,
            credit_id_: i32,
        ) -> diesel::QueryResult<Vec<CreditProduct>> {
            credit_product
                .filter(credit_id.eq(credit_id_))
                .order(product_id.asc())
                .load(conn)
        }

        pub fn create(
            conn: &mut diesel::PgConnection,
            form: &CreditForm,
            products: Vec<CreditProductForm>,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                let new_id = diesel::insert_into(credit)
                    .values(form)
                    .returning(crate::server::schema::credit::id)
                    .get_result::<i32>(conn)?;
                if !products.is_empty() {
                    diesel::insert_into(credit_product)
                        .values(
                            products
                                .into_iter()
                                .map(|mut p| {
                                    p.credit_id = new_id;
                                    p
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
            credit_id_: i32,
            form: &CreditForm,
            products: Vec<CreditProductForm>,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                diesel::update(credit.find(credit_id_))
                    .set((form, updated_at.eq(diesel::dsl::now)))
                    .execute(conn)?;
                diesel::delete(credit_product.filter(credit_id.eq(credit_id_))).execute(conn)?;
                if !products.is_empty() {
                    diesel::insert_into(credit_product)
                        .values(
                            products
                                .into_iter()
                                .map(|mut p| {
                                    p.credit_id = credit_id_;
                                    p
                                })
                                .collect::<Vec<_>>(),
                        )
                        .execute(conn)?;
                }
                Self::find(conn, credit_id_)
            })
        }

        pub fn delete(
            conn: &mut diesel::PgConnection,
            credit_id_: i32,
        ) -> diesel::QueryResult<usize> {
            conn.transaction(|conn| {
                diesel::delete(credit_product.filter(credit_id.eq(credit_id_))).execute(conn)?;
                diesel::delete(credit.find(credit_id_)).execute(conn)
            })
        }
    }
}
