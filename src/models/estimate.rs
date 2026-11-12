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
#[cfg_attr(
    feature = "server",
    derive(Insertable, AsChangeset, Associations, HasQuery)
)]
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

impl From<EstimateProduct> for EstimateProductForm {
    fn from(product: EstimateProduct) -> Self {
        Self {
            estimate_id: product.estimate_id,
            product_id: product.product_id,
            quantity: product.quantity,
            price: product.price,
            tax_rate: product.tax_rate,
            description: product.description,
        }
    }
}

#[cfg(feature = "server")]
mod estimate_server {
    use super::{Estimate, EstimateForm, EstimateProduct, EstimateProductForm};
    use crate::server::schema::estimate::dsl::*;
    use crate::server::schema::estimate_product::dsl::*;
    use diesel::prelude::*;

    impl Estimate {
        pub fn all(conn: &mut diesel::PgConnection) -> diesel::QueryResult<Vec<Self>> {
            estimate
                .select(Self::as_select())
                .order(updated_at.desc())
                .load(conn)
        }

        pub fn find(conn: &mut diesel::PgConnection, est_id: i32) -> diesel::QueryResult<Self> {
            estimate.select(Self::as_select()).find(est_id).first(conn)
        }

        pub fn products(
            conn: &mut diesel::PgConnection,
            est_id: i32,
        ) -> diesel::QueryResult<Vec<EstimateProduct>> {
            EstimateProduct::query()
                .filter(estimate_id.eq(est_id))
                .order(product_id.asc())
                .load(conn)
        }

        pub fn create(
            conn: &mut diesel::PgConnection,
            form: &EstimateForm,
            products: Vec<EstimateProductForm>,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                let est_id = diesel::insert_into(estimate)
                    .values(form)
                    .returning(crate::server::schema::estimate::id)
                    .get_result::<i32>(conn)?;
                if !products.is_empty() {
                    diesel::insert_into(estimate_product)
                        .values(
                            products
                                .into_iter()
                                .map(|mut p| {
                                    p.estimate_id = est_id;
                                    p
                                })
                                .collect::<Vec<_>>(),
                        )
                        .execute(conn)?;
                }
                Self::find(conn, est_id)
            })
        }

        pub fn update(
            conn: &mut diesel::PgConnection,
            est_id: i32,
            form: &EstimateForm,
            products: Vec<EstimateProductForm>,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                diesel::update(estimate.find(est_id))
                    .set((form, updated_at.eq(diesel::dsl::now)))
                    .execute(conn)?;
                diesel::delete(estimate_product.filter(estimate_id.eq(est_id))).execute(conn)?;
                if !products.is_empty() {
                    diesel::insert_into(estimate_product)
                        .values(
                            products
                                .into_iter()
                                .map(|mut p| {
                                    p.estimate_id = est_id;
                                    p
                                })
                                .collect::<Vec<_>>(),
                        )
                        .execute(conn)?;
                }
                Self::find(conn, est_id)
            })
        }

        pub fn delete(conn: &mut diesel::PgConnection, est_id: i32) -> diesel::QueryResult<usize> {
            conn.transaction(|conn| {
                diesel::delete(estimate_product.filter(estimate_id.eq(est_id))).execute(conn)?;
                diesel::delete(estimate.find(est_id)).execute(conn)
            })
        }
    }
}

#[cfg(feature = "server")]
mod estimate_product_server {
    use super::EstimateProduct;
    use crate::server::schema::estimate_product::dsl::*;
    use diesel::prelude::*;

    impl EstimateProduct {
        pub fn find(conn: &mut diesel::PgConnection, id_: i32) -> diesel::QueryResult<Self> {
            estimate_product.find(id_).first(conn)
        }

        pub fn create(conn: &mut diesel::PgConnection, form: &Self) -> diesel::QueryResult<Self> {
            diesel::insert_into(estimate_product)
                .values(form)
                .get_result(conn)
        }

        pub fn update(
            conn: &mut diesel::PgConnection,
            id_: i32,
            form: &Self,
        ) -> diesel::QueryResult<Self> {
            diesel::update(estimate_product.find(id_))
                .set(form)
                .get_result(conn)
        }

        pub fn delete(conn: &mut diesel::PgConnection, id_: i32) -> diesel::QueryResult<usize> {
            diesel::delete(estimate_product.find(id_)).execute(conn)
        }
    }
}

#[cfg(feature = "server")]
mod estimate_search {
    use crate::{api::search::SearchResult, server::search::SearchFilter};
    use diesel::prelude::*;
    use dioxus::server::inventory;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize, HasQuery)]
    #[diesel(table_name = crate::server::schema::estimate)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct EstimateSearchResult {
        pub id: i32,
        pub doc_number: String,
        pub description: Option<String>,
    }

    impl From<EstimateSearchResult> for SearchResult {
        fn from(value: EstimateSearchResult) -> Self {
            SearchResult {
                route: crate::Route::EditEstimate { id: value.id },
                title: format!("Estimate #{}", value.doc_number),
                description: value.description.unwrap_or_default(),
            }
        }
    }

    inventory::submit! {
       SearchFilter::new(
         |query, conn| {
             use crate::server::schema::estimate::dsl::*;
             let pattern = format!("%{}%", query);
             EstimateSearchResult::query()
                 .filter(
                     doc_number.ilike(&pattern)
                         .or(description.ilike(&pattern)),
                 )
                 .load(conn)
                 .map(|results| {
                     results.into_iter().map(|est| {
                         SearchResult::from(est)
                     }).collect::<Vec<_>>()
                 })
         }
       )
    }
}
