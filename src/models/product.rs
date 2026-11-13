use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub description: Option<String>,
    pub price: i32,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct ProductForm {
    pub name: String,
    pub sku: String,
    pub description: Option<String>,
    pub price: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg(feature = "server")]
#[cfg_attr(feature = "server", derive(HasQuery))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct ProductSearchResult {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub price: i32,
    pub quantity: i32,
}

#[cfg(feature = "server")]
mod product_server {
    use super::{Product, ProductForm};
    use crate::server::schema::product::dsl::*;
    use diesel::prelude::*;

    impl Product {
        pub fn all(conn: &mut diesel::PgConnection) -> diesel::QueryResult<Vec<Self>> {
            product.order(name.asc()).load(conn)
        }

        pub fn find(conn: &mut diesel::PgConnection, product_id: i32) -> diesel::QueryResult<Self> {
            product.find(product_id).first(conn)
        }

        pub fn create(
            conn: &mut diesel::PgConnection,
            form: &ProductForm,
        ) -> diesel::QueryResult<Self> {
            diesel::insert_into(product).values(form).get_result(conn)
        }

        pub fn update(
            conn: &mut diesel::PgConnection,
            product_id: i32,
            form: &ProductForm,
        ) -> diesel::QueryResult<Self> {
            diesel::update(product.find(product_id))
                .set((form, updated_at.eq(diesel::dsl::now)))
                .get_result(conn)
        }

        pub fn delete(
            conn: &mut diesel::PgConnection,
            product_id: i32,
        ) -> diesel::QueryResult<usize> {
            diesel::delete(product.find(product_id)).execute(conn)
        }
    }
}

#[cfg(feature = "server")]
mod product_search {
    use super::ProductSearchResult;
    use crate::{
        api::search::SearchResult,
        server::{schema::product::dsl::*, search::SearchFilter},
    };
    use diesel::prelude::*;
    use dioxus::server::inventory;

    impl From<ProductSearchResult> for SearchResult {
        fn from(value: ProductSearchResult) -> Self {
            SearchResult::new()
                .with_route(crate::Route::EditProduct { id: value.id })
                .with_title(value.name)
                .with_description(value.sku)
                .with_category("Products")
        }
    }

    inventory::submit! {
      SearchFilter::new(
        |query: &str, conn: &mut PgConnection| {
          let query = format!("%{}%", query);
          product
              .filter(name.ilike(&query).or(sku.ilike(&query)))
              .select((id, name, sku, price, quantity))
              .load::<ProductSearchResult>(conn)
              .map(|results| results.into_iter().map(SearchResult::from).collect())
        }
      )
    }
}
