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

#[cfg(feature = "server")]
impl Product {
    pub fn all(conn: &mut diesel::PgConnection) -> diesel::QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        crate::server::schema::product::table
            .select(Self::as_select())
            .order(crate::server::schema::product::name.asc())
            .load(conn)
    }

    pub fn find(conn: &mut diesel::PgConnection, product_id: i32) -> diesel::QueryResult<Self> {
        use diesel::prelude::*;

        crate::server::schema::product::table
            .select(Self::as_select())
            .find(product_id)
            .first(conn)
    }

    pub fn create(
        conn: &mut diesel::PgConnection,
        form: &ProductForm,
    ) -> diesel::QueryResult<Self> {
        use diesel::prelude::*;

        diesel::insert_into(crate::server::schema::product::table)
            .values(form)
            .returning(Self::as_returning())
            .get_result(conn)
    }

    pub fn update(
        conn: &mut diesel::PgConnection,
        product_id: i32,
        form: &ProductForm,
    ) -> diesel::QueryResult<Self> {
        use diesel::prelude::*;

        diesel::update(crate::server::schema::product::table.find(product_id))
            .set((
                form,
                crate::server::schema::product::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Self::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut diesel::PgConnection, product_id: i32) -> diesel::QueryResult<usize> {
        use diesel::prelude::*;

        diesel::delete(crate::server::schema::product::table.find(product_id)).execute(conn)
    }
}
