use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;

#[derive(Debug)]
#[cfg_attr(feature = "server", derive(HasQuery))]
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
