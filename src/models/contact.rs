use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::server::schema::contact))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct ContactSearchResult {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
}

#[cfg(feature = "server")]
mod contact_server {
    use super::{Contact, ContactForm};
    use crate::server::schema::contact::dsl::*;
    use diesel::prelude::*;

    impl Contact {
        pub fn all(conn: &mut diesel::PgConnection) -> diesel::QueryResult<Vec<Self>> {
            contact
                .order((last_name.asc(), first_name.asc()))
                .load(conn)
        }

        pub fn find(conn: &mut diesel::PgConnection, contact_id: i32) -> diesel::QueryResult<Self> {
            contact.filter(id.eq(contact_id)).first(conn)
        }

        pub fn create(
            conn: &mut diesel::PgConnection,
            form: &ContactForm,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                diesel::insert_into(contact)
                    .values(form)
                    .returning(id)
                    .get_result::<i32>(conn)
                    .and_then(|contact_id| Self::find(conn, contact_id))
            })
        }

        pub fn update(
            conn: &mut diesel::PgConnection,
            contact_id: i32,
            form: &ContactForm,
        ) -> diesel::QueryResult<Self> {
            conn.transaction(|conn| {
                diesel::update(contact.find(contact_id))
                    .set(form)
                    .execute(conn)?;
                Self::find(conn, contact_id)
            })
        }

        pub fn delete(
            conn: &mut diesel::PgConnection,
            contact_id: i32,
        ) -> diesel::QueryResult<usize> {
            diesel::delete(contact.find(contact_id)).execute(conn)
        }
    }
}

#[cfg(feature = "server")]
mod contact_search {
    use super::ContactSearchResult;
    use dioxus::server::inventory;

    use crate::{api::search::SearchResult, server::search::SearchFilter};

    impl From<ContactSearchResult> for SearchResult {
        fn from(contact: ContactSearchResult) -> Self {
            SearchResult {
                route: crate::Route::EditContact { id: contact.id },
                title: format!("{} {}", contact.first_name, contact.last_name),
                description: format!("Email: {}, Phone: {}", contact.email, contact.phone_number),
            }
        }
    }

    inventory::submit! {
       SearchFilter::new(
           |query: &str, conn: &mut diesel::PgConnection| {
               use crate::server::schema::contact::dsl::*;
               use diesel::prelude::*;

               let pattern = format!("%{}%", query);
               contact
                   .filter(
                       first_name.ilike(&pattern)
                           .or(last_name.ilike(&pattern))
                           .or(email.ilike(&pattern))
                           .or(phone_number.ilike(&pattern)),
                   )
                   .select(ContactSearchResult::as_select())
                   .load::<ContactSearchResult>(conn)
                   .map(|results| {
                       results.into_iter().map(|ctc| {
                           SearchResult::from(ctc)
                       }).collect::<Vec<_>>()
                   })
           },
       )
    }
}
