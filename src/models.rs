use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::users))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::users))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct UserForm<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub email: &'a str,
}

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
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::product))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct ProductForm {
    pub name: String,
    pub sku: String,
    pub description: Option<String>,
    pub price: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::contact))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Contact {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    pub message: Option<String>,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "server", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::contact))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct ContactForm {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    pub message: Option<String>,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
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
                crate::server::schema::product::name.eq(&form.name),
                crate::server::schema::product::description.eq(&form.description),
                crate::server::schema::product::price.eq(form.price),
                crate::server::schema::product::quantity.eq(form.quantity),
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

#[cfg(feature = "server")]
impl Contact {
    pub fn all(conn: &mut diesel::PgConnection) -> diesel::QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        crate::server::schema::contact::table
            .select(Contact::as_select())
            .order((
                crate::server::schema::contact::last_name.asc(),
                crate::server::schema::contact::first_name.asc(),
            ))
            .load(conn)
    }

    pub fn find(conn: &mut diesel::PgConnection, contact_id: i32) -> diesel::QueryResult<Self> {
        use diesel::prelude::*;
        crate::server::schema::contact::table
            .select(Contact::as_select())
            .filter(crate::server::schema::contact::id.eq(contact_id))
            .first(conn)
    }

    pub fn create(
        conn: &mut diesel::PgConnection,
        form: &ContactForm,
    ) -> diesel::QueryResult<Self> {
        use diesel::prelude::*;

        conn.transaction(|conn| {
            diesel::insert_into(crate::server::schema::contact::table)
                .values(form)
                .returning(crate::server::schema::contact::id)
                .get_result::<i32>(conn)
                .and_then(|id| Self::find(conn, id))
        })
    }

    pub fn update(
        conn: &mut diesel::PgConnection,
        contact_id: i32,
        form: &ContactForm,
    ) -> diesel::QueryResult<Self> {
        use diesel::prelude::*;

        conn.transaction(|conn| {
            diesel::update(crate::server::schema::contact::table.find(contact_id))
                .set(form)
                .execute(conn)?;
            Self::find(conn, contact_id)
        })
    }

    pub fn delete(conn: &mut diesel::PgConnection, contact_id: i32) -> diesel::QueryResult<usize> {
        use diesel::prelude::*;

        conn.transaction(|conn| {
            let deleted = diesel::delete(crate::server::schema::contact::table.find(contact_id))
                .execute(conn)?;
            Ok(deleted)
        })
    }
}
